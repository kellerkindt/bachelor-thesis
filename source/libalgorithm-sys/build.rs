extern crate bindgen;
extern crate walkdir;

use std::fs;
use std::io::Write;
use std::path::Path;

use walkdir::DirEntry;
use walkdir::WalkDir;

const BINDINGS_FILE: &'static str = "src/bindings.rs";

fn main() {
    println!("cargo:rustc-link-search=native={}", ".");
    println!("cargo:rustc-flags=-l dylib=stdc++");

    let mut headers = find_headers("cpp/mecview-sdk/algorithm");
    headers.append(&mut find_headers("cpp/mecview-sdk/extension"));
    headers.append(&mut find_headers("cpp/shim"));
    headers.append(&mut find_headers("cpp/impl"));

    if !Path::new(BINDINGS_FILE).exists() {
        let main_header = "cpp/wrapper/wrapper.cpp";

        generate_main_header(&headers, main_header);
        generate_bindings(
            "cpp/",
            main_header,
            Path::new(BINDINGS_FILE).to_str().unwrap(),
        );
    }
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
            })
            .unwrap_or(None);
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

    let _ = writeln!(file, "\n\nint main() {{ return 0; }}");
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
        //.whitelist_recursively(false)
        .opaque_type("asn_struct_ctx_s")
        .opaque_type("asn_struct_ctx_t")
        .opaque_type("std::shared_ptr")
        .opaque_type("RustShimInternal")
        .opaque_type("RustEventListener")
        .opaque_type("EnvironmentFrame_t")
        .opaque_type("SensorFrame_t")
        .opaque_type("InitMessage_t")
        .whitelist_type("RustShim")
        .whitelist_function("create_rust_shim_for_algorithm")
        .whitelist_function("send_sensor_frame")
        .rustfmt_bindings(true)
        .impl_debug(true)
        .derive_copy(false)
        .impl_partialeq(true)
        .generate_comments(true)
        .generate()
            .unwrap()
            .write_to_file(out).expect("Couldn't write bindings!");
}
