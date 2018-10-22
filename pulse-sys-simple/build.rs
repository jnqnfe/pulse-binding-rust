#[cfg(target_os="linux")]
extern crate pkg_config;

#[cfg(target_os="linux")]
fn main() {
    pkg_config::Config::new().atleast_version("11.0").probe("libpulse-simple").unwrap();
}

#[cfg(not(target_os="linux"))]
fn main() {
    println!("cargo:rustc-link-lib=libpulse-simple");
}
