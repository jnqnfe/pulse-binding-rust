#[cfg(target_os="linux")]
extern crate pkg_config;

#[cfg(target_os="linux")]
fn main() {
    let version = match cfg!(feature="pa_encoding_from_string") {
        true => "12.0",
        false => "11.0",
    };
    pkg_config::Config::new().atleast_version(version).probe("libpulse").unwrap();
}

#[cfg(not(target_os="linux"))]
fn main() {
    println!("cargo:rustc-link-lib=pulse");
}
