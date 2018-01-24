fn main() {
    if cfg!(target_os="linux") {
        println!("cargo:rustc-link-lib=pulse-mainloop-glib::libpulse-mainloop-glib.so.0");
    }
    else {
        println!("cargo:rustc-link-lib=pulse-mainloop-glib");
    }
}
