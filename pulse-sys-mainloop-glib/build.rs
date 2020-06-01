extern crate pkg_config;

fn main() {
    let lib_name = "libpulse-mainloop-glib";
    let fallback_name = {
        #[cfg(target_os = "linux")]
        { "pulse-mainloop-glib::libpulse-mainloop-glib.so.0" }
        #[cfg(target_os = "macos")]
        { "pulse-mainloop-glib::libpulse-mainloop-glib.0.dylib" }
        #[cfg(windows)]
        { "pulse-mainloop-glib::libpulse-mainloop-glib-0.dll" }
        #[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
        { "pulse-mainloop-glib" }
    };
    let min_version = "4.0";

    let mut config = pkg_config::Config::new();

    // Has the user got pkg-config and the PA pkg-config file installed (via dev package)?
    // This is a little crude, since impossible to reliably distinguish between pkg-config errors
    // (it only gives strings, and they could be translated). We perform a non-version specific
    // check here, and disable generation of cargo meta data, thus doing a 'exists' type check.
    config.cargo_metadata(false);
    let fallback = match config.probe(lib_name) {
        // We assume all failure here (being a non-version specific check) indicates no *.pc file
        Err(pkg_config::Error::Failure { .. }) => {
            eprintln!("Pkg-config seems to not know about PulseAudio (dev package not installed?), \
                       trying generic fallback...");
            true
        },
        // Also allow fallback if pkg-config not installed, or disabled
        Err(pkg_config::Error::EnvNoPkgConfig(_)) |
        Err(pkg_config::Error::Command { .. }) => {
            eprintln!("No pkg-config or disabled, trying generic fallback...");
            true
        },
        // In all other cases we will perform a version-specfic check and honor the result
        _ => false,
    };

    // If the user does not have pkg-config or the PA *.pc file (they have not installed the dev
    // package), then let's try a default fallback (having to install dev packages for Rust
    // development is unnatural imo, ideally distros should start shipping *.pc files differently).
    if fallback {
        println!("cargo:rustc-link-lib={}", fallback_name);
        return;
    }

    config.cargo_metadata(true)
          .atleast_version(min_version);

    // Do version specific pkg-config check and honor result
    match config.probe(lib_name) {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
        Ok(_) => {},
    }
}
