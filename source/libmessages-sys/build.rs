// extern crate cc;
extern crate gcc;
extern crate bindgen;

use std::fs;
use std::path::Path;
use std::io::Write;

const LIBRARY_FILE  : &'static str = "libmessage.a";
const BINDINGS_FILE : &'static str = "src/bindings.rs";

fn main() {

    println!("cargo:rustc-link-search=native={}", ".");
    println!("cargo:rustc-flags=-l dylib=stdc++");

    if !Path::new(LIBRARY_FILE).exists() || !Path::new(BINDINGS_FILE).exists() {
        let headers = compile_sdk("cpp/MECViewServerSDK-Build/proto/", LIBRARY_FILE);

        if !Path::new(BINDINGS_FILE).exists() {
            let main_header = "cpp/wrapper/wrapper.h";

            generate_main_header(&headers, main_header);
            generate_bindings("cpp/MECViewServerSDK-Build/proto/", main_header, Path::new(BINDINGS_FILE).to_str().unwrap());
        }
    }
}

fn compile_sdk(sdk_dir: &str, out: &str) -> Vec<String> {
    let mut headers = Vec::new();

    match fs::read_dir(sdk_dir) {
        Err(e) => panic!("Did not find sdk dir {:?}", e),
        Ok(dir) => {
            let mut gcc_build = gcc::Build::new();

            for file in dir {
                if let Ok(entry) = file {
                    if let Ok(t) = entry.file_type() {
                        if t.is_file() {
                            let path = entry.path();
                            let str = path.to_str().unwrap();
                            if !str.contains("sample") {
                                if str.ends_with(".c") {
                                    gcc_build.file(str);
                                } else if str.ends_with(".h") {
                                    headers.push(str.into());
                                }
                            }
                        }
                    }
                }
            }

            gcc_build
                .include(sdk_dir)
                .compile(out);
        }
    }

    headers
}

fn generate_main_header(headers: &[String], out: &str) {
    let _ = fs::create_dir(Path::new(out).parent().unwrap());
    let mut file = fs::File::create(Path::new(out)).unwrap();

    for header in headers.iter() {
        let _ = writeln!(file, "#include <{}>", header.split("/").last().unwrap());
    }
}

fn generate_bindings(include: &str, header: &str, out: &str) {
    bindgen::Builder::default()
        .clang_arg(format!("-I{}", include))
        // fix for link error, see rust-lang-nursery/rust-bindgen/issues/1046
        .trust_clang_mangling(false)
        .header(header)
        // not supported/test fails (not needed?, via indirect include)
        .blacklist_type("max_align_t")
        .opaque_type("asn_struct_ctx_s")
        .opaque_type("asn_struct_ctx_t")
        .rustfmt_bindings(true)
        .impl_debug(true)
        .derive_copy(false)
        .impl_partialeq(true)
        .generate_comments(true)
        .generate()
            .unwrap()
            .write_to_file(out).expect("Couldn't write bindings!");
}