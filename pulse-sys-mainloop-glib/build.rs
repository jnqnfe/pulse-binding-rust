fn main() {
    // Skip pkg-config check if just generating documentation.
    if cfg!(doc) {
        return;
    }

    if let Err(e) = system_deps::Config::new().probe() {
        println!("cargo:warning={}", e);
        std::process::exit(1);
    }
}
