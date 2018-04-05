// extern crate cc;
extern crate gcc;
extern crate bindgen;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use std::env;

const LIBRARY_FILE  : &'static str = "libmessage.a";
const BINDINGS_FILE : &'static str = "bindings.rs";

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    if !Path::new(LIBRARY_FILE).exists() || !out_path.join(BINDINGS_FILE).exists() {
        let headers = compile_sdk("cpp/MECViewServerSDK-Build/proto/", LIBRARY_FILE);

        if !out_path.join(BINDINGS_FILE).exists() {
            let main_header = "cpp/wrapper/wrapper.h";

            generate_main_header(&headers, main_header);
            generate_bindings("cpp/MECViewServerSDK-Build/proto/", main_header, out_path.join(BINDINGS_FILE).as_path().to_str().unwrap());
        }
    }

    println!("cargo:rustc-link-search=native={}", ".");
    println!("cargo:rustc-flags=-l dylib=stdc++");
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
                                    println!("Found header file: {}", str);
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
        .header(header)
        .rustfmt_bindings(true)
        .generate()
            .unwrap()
            .write_to_file(out).expect("Couldn't write bindings!");
}