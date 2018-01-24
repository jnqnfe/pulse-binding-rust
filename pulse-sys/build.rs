fn main() {
    if cfg!(target_os="linux") {
        println!("cargo:rustc-link-lib=pulse::libpulse.so.0");
    }
    else {
        println!("cargo:rustc-link-lib=pulse");
    }
}
