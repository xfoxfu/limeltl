extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

#[cfg(not(feature = "glucose"))]
pub fn main() {
    cc::Build::new()
        .cpp(true)
        .include("lib/minisat")
        .include("lib")
        .file("lib/minisat/core/Solver.cc")
        .file("lib/minisat/simp/SimpSolver.cc")
        .file("lib/minisat/utils/System.cc")
        //.file("lib/minisat/minisat/utils/Options.cc")
        .file("lib/minisat-c-bindings/minisat.cc")
        .define("__STDC_LIMIT_MACROS", None)
        .define("__STDC_FORMAT_MACROS", None)
        .define("fwrite_unlocked", Some("fwrite"))
        .include("/usr/include")
        .compile("minisat");

    let bindings = bindgen::Builder::default()
        .clang_arg("-Ilib/minisat-c-bindings")
        .header("wrapper.h")
        .generate()
        .expect("Could not create bindings to library");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(feature = "glucose")]
pub fn main() {
    cc::Build::new()
        .cpp(true)
        //.include("lib/minisat")
        .include("lib/glucose-syrup-4.1")
        .include("lib")
        .file("lib/glucose-syrup-4.1/core/Solver.cc")
        .file("lib/glucose-syrup-4.1/simp/SimpSolver.cc")
        .file("lib/glucose-syrup-4.1/utils/System.cc")
        //.file("lib/minisat/minisat/utils/Options.cc")
        .file("lib/minisat-c-bindings/minisat.cc")
        .flag("-std=c++11")
        .define("__STDC_LIMIT_MACROS", None)
        .define("__STDC_FORMAT_MACROS", None)
        .define("USE_GLUCOSE", None)
        .include("/usr/include")
        .compile("minisat");

    let bindings = bindgen::Builder::default()
        .clang_arg("-Ilib/minisat-c-bindings")
        .header("wrapper.h")
        .generate()
        .expect("Could not create bindings to library");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
