extern crate cpp_build;

fn main() {
    println!("cargo:rustc-link-lib=NazaraCore");
    cpp_build::Config::new()
        .flag("-std=c++14")
        .build("src/sys/mod.rs");
}
