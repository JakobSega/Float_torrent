fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=dylib=cblas");
    println!("cargo:rustc-link-lib=dylib=lapack");
    println!("cargo:rustc-link-search=native=C:\\Users\\blin\\vcpkg\\installed\\x64-windows\\lib");
}
