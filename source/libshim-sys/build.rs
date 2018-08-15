extern crate bindgen;
extern crate gcc;
extern crate walkdir;

use std::fs;
use std::io::Write;
use std::path::Path;

use walkdir::DirEntry;
use walkdir::WalkDir;

const LIBRARY_FILE: &'static str = "libalgorithm.a";
const BINDINGS_FILE: &'static str = "src/bindings.rs";

fn main() {
    println!("cargo:rustc-link-search=native={}", ".");
    // warning: redundant linker flag specified for library `stdc++`
    // println!("cargo:rustc-flags=-l dylib=stdc++");

    if !Path::new(LIBRARY_FILE).exists() || !Path::new(BINDINGS_FILE).exists() {
        let mut headers = find_headers("cpp/mecview-sdk/algorithm");
        headers.append(&mut find_headers("cpp/mecview-sdk/extension"));
        headers.append(&mut find_headers("cpp/shim"));
        headers.append(&mut find_headers("cpp/impl"));

        let main_header = "cpp/wrapper/wrapper.cpp";

        generate_main_header(&headers, main_header);

        let _headers = compile_sdk(&["cpp/impl", "cpp/wrapper/"], LIBRARY_FILE);

        if !Path::new(BINDINGS_FILE).exists() {
            generate_bindings(
                "cpp/",
                main_header,
                Path::new(BINDINGS_FILE).to_str().unwrap(),
            );
        }
    }
}

fn compile_sdk(sdk_dirs: &[&str], out: &str) -> Vec<String> {
    let mut headers = Vec::new();

    let mut gcc_build = gcc::Build::new();

    gcc_build.include("cpp/");
    gcc_build.include("cpp/mecview-sdk/proto/");

    let libs_path = ::std::path::Path::new("cpp/lib/").canonicalize().unwrap();
    println!(
        "cargo:rustc-link-search={}",
        libs_path.as_path().to_str().unwrap()
    );
    /*{
        let paths = ::std::fs::read_dir(&libs_path).unwrap();
        paths.filter_map(|entry| {
            entry.ok().and_then(|e|
                e.path().file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            )
        })
            .filter(|f| f.starts_with("lib") && f.ends_with(".a"))
            .for_each(|f| println!("cargo:rustc-link-lib={}", {
                let len = f.len();
                f.chars().skip(3).take(len-3-2).collect::<String>()
            }));
    }*/

    println!("cargo:rustc-link-lib=SampleAlgorithm");
    //println!("cargo:rustc-link-lib=MECFusionAlgorithm");
    //println!("cargo:rustc-link-lib=FusionToolbox");
    //gcc_build.flag("-Lcpp/lib/");
    // gcc_build.flag("-l:libMECFusionAlgorithm.a");
    // gcc_build.flag("-l:libFusionToolbox.a");

    gcc_build.cpp(true);
    gcc_build.flag("-std=c++11");
    gcc_build.static_flag(true);

    fn include_dir_in_compilation(
        sdk_dir: &str,
        headers: &mut Vec<String>,
        gcc_build: &mut gcc::Build,
    ) {
        let dir = fs::read_dir(sdk_dir).unwrap();
        for file in dir {
            if let Ok(entry) = file {
                if let Ok(t) = entry.file_type() {
                    if t.is_file() {
                        let path = entry.path();
                        let str = path.to_str().unwrap();
                        if str.ends_with(".c") || str.ends_with(".cpp") {
                            gcc_build.file(str);
                        } else if str.ends_with(".h") {
                            headers.push(str.into());
                        }
                    }
                }
            }
        }
    }

    for dir in sdk_dirs {
        gcc_build.include(*dir);
        include_dir_in_compilation(dir, &mut headers, &mut gcc_build);
    }

    gcc_build.compile(out);

    headers
}

fn find_headers(sdk_dir: &str) -> Vec<String> {
    let mut headers = Vec::new();

    fn is_header(entry: &DirEntry) -> bool {
        entry.file_type().is_dir()
            || entry.file_type().is_symlink()
            || entry.path().extension().map(|s| s.eq("h")).unwrap_or(false)
    }

    for entry in WalkDir::new(sdk_dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(is_header)
    {
        let entry = entry
            .map(|file| {
                if file.file_type().is_file() {
                    Some(format!("{}", file.path().display()))
                } else {
                    None
                }
            }).unwrap_or(None);
        if entry.is_some() {
            headers.push(entry.unwrap());
        }
    }

    headers
}

fn generate_main_header(headers: &[String], out: &str) {
    let _ = fs::create_dir(Path::new(out).parent().unwrap());
    let mut file = fs::File::create(Path::new(out)).unwrap();

    for header in headers.iter() {
        let _ = writeln!(
            file,
            "#include \"../{}\"",
            header.split("cpp/").last().unwrap(),
        );
    }
}

fn generate_bindings(include: &str, header: &str, out: &str) {
    bindgen::Builder::default()
        .clang_arg(format!("-Icpp/mecview-sdk/proto"))
        .clang_arg(format!("-Icpp/impl"))
        .clang_arg(format!("-I{}", include))
        .clang_arg("-std=c++14")
        // fix for link error, see rust-lang-nursery/rust-bindgen/issues/1046
        .trust_clang_mangling(false)
        .header(header)
        // not supported/test fails (not needed?, via indirect include)
        .blacklist_type("max_align_t")
        .whitelist_recursively(false)
        .opaque_type("std::shared_ptr")
        .opaque_type("asn_struct_ctx_s")
        .opaque_type("asn_struct_ctx_t")
        .opaque_type("RustShimInternal")
        .opaque_type("RustInstance")
        .opaque_type("EnvironmentFrame_t")
        .opaque_type("SensorFrame_t")
        .opaque_type("InitMessage_t")
        .whitelist_type("EnvironmentFrame_t")
        .whitelist_type("SensorFrame_t")
        .whitelist_type("InitMessage_t")
        .whitelist_type("RustShim")
        .whitelist_type("RustInstance")
        .whitelist_function("shim_create")
        .whitelist_function("shim_destroy")
        .whitelist_function("shim_send_sensor_frame")
        .rustfmt_bindings(true)
        .impl_debug(true)
        .derive_copy(false)
        .impl_partialeq(true)
        .generate_comments(true)
        .generate()
            .unwrap()
            .write_to_file(out).expect("Couldn't write bindings!");
}
