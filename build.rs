fn main() {
    if cfg!(all(target_env = "msvc", not(debug_assertions))) {
        println!("cargo:rustc-link-arg=/DEBUG:NONE");
    }
}
