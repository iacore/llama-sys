extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("llama.cpp")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let ctx = BuildContext {
        libdir_path,
        out_path,
    };

    ctx.build_library();
    ctx.generate_bindings();
    println!("cargo:rustc-link-lib=static=llama");
}

struct BuildContext {
    libdir_path: PathBuf,
    out_path: PathBuf,
}
impl BuildContext {
    fn build_library(&self) {
        // Tell cargo to look for shared libraries in the specified directory
        println!(
            "cargo:rustc-link-search={}",
            self.out_path.to_str().unwrap()
        );

        println!("cargo:rustc-link-lib=pthread");

        cc::Build::new()
            .cpp(true)
            .file(self.libdir_path.join("llama.cpp"))
            .out_dir(&self.out_path)
            .compile("llama-b");

        cc::Build::new()
            .file(self.libdir_path.join("ggml.c"))
            .out_dir(&self.out_path)
            .compile("llama-a");
    }

    fn generate_bindings(&self) {
        // Tell cargo to invalidate the built crate whenever the wrapper changes
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=wrapper.h");
        println!("cargo:rerun-if-changed=llama.cpp/ggml.h");
        println!("cargo:rerun-if-changed=llama.cpp/llama.h");

        // The bindgen::Builder is the main entry point
        // to bindgen, and lets you build up options for
        // the resulting bindings.
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .clang_args(["-I", self.libdir_path.to_str().expect("path is valid str")])
            .header("wrapper.h")
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(self.out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
