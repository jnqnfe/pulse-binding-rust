fn main() {
    if cfg!(target_os="linux") {
        println!("cargo:rustc-link-lib=pulse-simple::libpulse-simple.so.0");
    }
    else {
        println!("cargo:rustc-link-lib=pulse-simple");
    }
}
