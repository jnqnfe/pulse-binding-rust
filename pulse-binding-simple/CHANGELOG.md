# [unreleased]

 * Made some changes to cargo features:
    - Removed the `pa_latest` and `pa_latest_common` cargo features.
    - Changed the default version feature level to `pa_v8`.
 * MSRV bumped from 1.40 to 1.41.
 * Made use of `#[cfg(doc)]` to always include stuff behind PA version feature guards in generated
   documentation. (Required bump of minimum supported Rust version from 1.40 to 1.41).

# 2.18.1 (November 25th, 2020)

 * Fixed deprecated license attribute syntax.

# 2.18.0 (November 25th, 2020)

 * Updated `libpulse-binding` version dependency (2.17 → 2.18).
 * Updated `libpulse-sys` version dependency (1.14 → 1.15).
 * Updated `libpulse-simple-sys` version dependency (1.14 → 1.15).

# 2.17.0 (November 24th, 2020)

 * Updated `libpulse-binding` version dependency (2.16 → 2.17).
 * Updated `libpulse-sys` version dependency (1.13 → 1.14).
 * Updated `libpulse-simple-sys` version dependency (1.13 → 1.14).

# 2.16.2 (November 21st, 2020)

 * Trivial documentation fixes.

# 2.16.1 (September 9th, 2020)

 * Bumped `pa_latest_common` feature to target PA v13.

# 2.16.0 (April 18th, 2020)

 * Removed deprecated Cargo features.
 * Updated `libpulse-binding` version dependency (2.15 → 2.16).
 * Updated `libpulse-sys` version dependency (1.12 → 1.13).
 * Updated `libpulse-simple-sys` version dependency (1.12 → 1.13).

# 2.15.0 (December 29th, 2019)

 * Updated `libpulse-binding` version dependency (2.14 → 2.15).
 * Updated `libpulse-sys` version dependency (1.11 → 1.12).
 * Updated `libpulse-simple-sys` version dependency (1.11 → 1.12).

# 2.14.0 (October 28th, 2019)

 * Updated `libpulse-binding` version dependency (2.13 → 2.14).

# 2.13.0 (September 17th, 2019)

 * Changed the license model from LGPL to dual MIT and Apache-2.0. See [here][issue26] for details.
 * Updated `libpulse-binding` version dependency (2.12 → 2.13).
 * Updated `libpulse-sys` version dependency (1.10 → 1.11).
 * Updated `libpulse-simple-sys` version dependency (1.10 → 1.11).

# 2.12.0 (September 15th, 2019)

 * Added PA v13 compatibility control feature.
 * Updated `libpulse-binding` version dependency (2.11 → 2.12).
 * Updated `libpulse-sys` version dependency (1.9 → 1.10).
 * Updated `libpulse-simple-sys` version dependency (1.9 → 1.10).

# 2.11.1 (August 19th, 2019)

 * Fixed broken doc.rs documentation generation.

# 2.11.0 (August 19th, 2019)

 * Extended support to even older versions of PA, specifically up to and including v4.
 * Simplified feature flags, old ones left as temorary aliases, to be removed later.
 * Added a `dox` feature flag, for use with `cargo doc`.
   It enables the very latest PA version compatibility, while bypassing the pkg-config check, thus
   is useful for generating documentation that includes information on features from versions of PA
   that may be newer than the version you have installed.
 * Updated `libpulse-binding` version dependency (2.10 → 2.11).
 * Updated `libpulse-sys` version dependency (1.8 → 1.9).
 * Updated `libpulse-simple-sys` version dependency (1.8 → 1.9).

# 2.10.0 (August 15th, 2019)

 * Updated `use` conventions to that of Rust 1.30/1.31.
 * Specified edition in toml file.
 * Updated `libpulse-binding` version dependency (2.7 → 2.10).
 * Updated `libpulse-sys` version dependency (1.6 → 1.8).
 * Updated `libpulse-simple-sys` version dependency (1.6 → 1.8).

Note: versions 2.8 and 2.9 skipped, used only for `libpulse-glib-binding` crate changes.

# 2.7.0 (August 12th, 2019)

 * Added a new `latest_pa_common_compatibility` feature flag, used by default now instead of
   `latest_pa_compatibility`.
 * Updated `libpulse-binding` version dependency (2.6 → 2.7).
 * Updated `libpulse-sys` version dependency (1.5 → 1.6).
 * Updated `libpulse-simple-sys` version dependency (1.5 → 1.6).

# 2.6.1 (April 5th, 2019)

 * Enabled `Send`+`Sync` for `Simple`, thanks to @MOZGIII for the patch.

# 2.6.0 (March 10th, 2019)

 * Updated `libpulse-binding` version dependency (2.5 → 2.6).

# 2.5.0 (December 22nd, 2018)

 * Added the new `latest_pa_compatibility` and `pa_v12_compatibility` feature flags, and deprecated
   `pa_encoding_from_string` in favour of `pa_v12_compatibility`.
 * Updated `libpulse-binding` version dependency (2.4 → 2.5).
 * Updated `libpulse-sys` version dependency (1.4 → 1.5).
 * Updated `libpulse-simple-sys` version dependency (1.4 → 1.5).

# 2.4.0 (November 28th, 2018)

 * Updated `libpulse-binding` version dependency (2.3 → 2.4).

# 2.3.0 (November 4th, 2018)

 * Clarified `pa_encoding_from_string` feature purpose.
 * Updated `libpulse-binding` version dependency (2.2 → 2.3).
 * Updated `libpulse-sys` version dependency (1.3 → 1.4).
 * Updated `libpulse-simple-sys` version dependency (1.3 → 1.4).

# 2.2.5 (October 8th, 2018)

 * Fixed broken attempt to include license file in bundled package.

# 2.2.4 (October 8th, 2018)

 * Added dedicated changelog, split off from the old single project overview one.
 * Included copy of license file in bundled package and excluded the `.gitignore` and `README.md`
   files.

# 2.2.3 (September 20th, 2018)

 * Fixed feature control.

# 2.2.2 (September 3rd, 2018)

 * Added homepage and repo links, thanks to @berkus on github for the patch.
 * Expanded the `pa_encoding_from_string` feature to properly control it across dependencies. Thanks
   to @thejpster on github for reporting.

Note, version number 2.2.1 skipped, bumping number into line with the main binding crate.

# 2.2 (August 21st, 2018)

 * Updated `libpulse-binding` version dependency (2.1 → 2.2).

# 2.1 (July 17th, 2018)

 * Updated with respect to having renamed the `timeval` mod to `time` in the main binding.
 * Updated `libpulse-binding` version dependency (2.0 → 2.1).
 * Updated `libpulse-sys` version dependency (1.2 → 1.3).
 * Updated `libpulse-simple-sys` version dependency (1.2 → 1.3).

# 2.0.1 (June 26th, 2018)

 * Updated version in `README` usage example.

# 2.0 (June 16th, 2018)

 * Updated `libpulse-binding` version dependency (1.2 → 2.0).
 * Updated `libpulse-sys` version dependency (1.1 → 1.2).
 * Updated `libpulse-simple-sys` version dependency (1.1 → 1.2).

# 1.2 (June 1st, 2018)

 * Tidied up error code handling, per version 1.2 of `libpulse-binding`.
 * Improved time handling, per version 1.2 of `libpulse-binding`.
 * Updated `libpulse-binding` version dependency (1.1 → 1.2).

# 1.1 (May 27th, 2018)

 * Privatised `SimpleInternal`.
 * Updated `libpulse-binding` version dependency (1.0 → 1.1).
 * Updated `libpulse-sys` version dependency (1.0 → 1.1).
 * Updated `libpulse-simple-sys` version dependency (1.0 → 1.1).

# 1.0.5 (May 27th, 2018)

 * Enabled doc test.

Note, some version numbers skipped, bumping number into line with the other crates.

# 1.0.2 (February 9th, 2018)

 * Added travis badge.

# 1.0.1 (February 2nd, 2018)

 * Fixed toml file license string `LGPL-2.1` → `LGPL-2.1+`.
 * Fixed toml file missing author email address.
 * Removed obsolete readme doc links.

# 1.0 (January 24th, 2018)

 * Original release.

[issue26]: https://github.com/jnqnfe/pulse-binding-rust/issues/26
