// extern crate cc;
extern crate gcc;
extern crate bindgen;

use std::fs;
use std::fs::FileType;


fn main() {
    /*
    cc::Build::new()
        .file("src/double.c")
        .compile("libdouble.a");
        */

    let mut headers = Vec::new();

    let inc_dir = "cpp/MECViewServerSDK-Build/proto/";
    let sdk_dir = fs::read_dir(inc_dir);
    match sdk_dir {
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
                .include(inc_dir)
                .compile("libmessages.a");
        }
    }



    gen_binding(&headers);

    gcc::compile_library("libblubber.a", &["c-src/blubber.c"]);
    // gcc::compile_library("libmessages.a",  &["cpp/MECViewServerSDK-Build/proto/asn_application.h", "cpp/MECViewServerSDK-Build/proto/Path.c"]);
    //gcc::compile_library("libmessages.a", &["cpp/MECViewServerSDK/algorithm/algorithm_factory.h"]);

    println!("cargo:rustc-link-search=native={}", ".");
    // println!("cargo:rustc-flags=-l dylib=stdc++ -l static=blubber");
    println!("cargo:rustc-flags=-l dylib=stdc++");
    // println!("cargo:rustc-link-lib=static={}", "blubber");
    // println!("cargo:rustc-link-lib=static={}", "messages");
}


fn gen_binding(headers: &Vec<String>) {
    let mut bindings = bindgen::Builder::default();

    for header in headers.iter() {
        bindings = bindings.clang_arg(format!("-I{}", header));
    }

    bindings = bindings.clang_arg("-Icpp/MECViewServerSDK-Build/proto/");

    bindings
        .header("cpp/wrapper/wrapper.h").generate().unwrap()
        .write_to_file("src/bindings.rs").expect("Couldn't write bindings!");;
}