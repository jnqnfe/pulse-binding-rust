# <unreleased>

 * Added a new `latest_pa_common_compatibility` feature flag, used by default now instead of
   `latest_pa_compatibility`.

# 2.6.1 (April 5th, 2019)

 * Enabled `Send`+`Sync` for `Simple`, thanks to @MOZGIII for the patch

# 2.6.0 (March 10th, 2019)

 * Updated `libpulse-binding` version dependency (2.5 → 2.6)

# 2.5.0 (December 22nd, 2018)

 * Added the new `latest_pa_compatibility` and `pa_v12_compatibility` feature flags, and deprecated
   `pa_encoding_from_string` in favour of `pa_v12_compatibility`.
 * Updated `libpulse-binding` version dependency (2.4 → 2.5)
 * Updated `libpulse-sys` version dependency (1.4 → 1.5)
 * Updated `libpulse-simple-sys` version dependency (1.4 → 1.5)

# 2.4.0 (November 28th, 2018)

 * Updated `libpulse-binding` version dependency (2.3 → 2.4)

# 2.3.0 (November 4th, 2018)

 * Clarified `pa_encoding_from_string` feature purpose
 * Updated `libpulse-binding` version dependency (2.2 → 2.3)
 * Updated `libpulse-sys` version dependency (1.3 → 1.4)
 * Updated `libpulse-simple-sys` version dependency (1.3 → 1.4)

# 2.2.5 (October 8th, 2018)

 * Fixed broken attempt to include license file in bundled package

# 2.2.4 (October 8th, 2018)

 * Added dedicated changelog, split off from the old single project overview one
 * Included copy of license file in bundled package and excluded the `.gitignore` and `README.md`
   files.

# 2.2.3 (September 20th, 2018)

 * Fixed feature control

# 2.2.2 (September 3rd, 2018)

 * Added homepage and repo links, thanks to @berkus on github for the patch
 * Expanded the `pa_encoding_from_string` feature to properly control it across dependencies. Thanks
   to @thejpster on github for reporting.

Note, version number 2.2.1 skipped, bumping number into line with the main binding crate

# 2.2 (August 21st, 2018)

 * Updated `libpulse-binding` version dependency (2.1 → 2.2)

# 2.1 (July 17th, 2018)

 * Updated with respect to having renamed the `timeval` mod to `time` in the main binding
 * Updated `libpulse-binding` version dependency (2.0 → 2.1)
 * Updated `libpulse-sys` version dependency (1.2 → 1.3)
 * Updated `libpulse-simple-sys` version dependency (1.2 → 1.3)

# 2.0.1 (June 26th, 2018)

 * Updated version in `README` usage example

# 2.0 (June 16th, 2018)

 * Updated `libpulse-binding` version dependency (1.2 → 2.0)
 * Updated `libpulse-sys` version dependency (1.1 → 1.2)
 * Updated `libpulse-simple-sys` version dependency (1.1 → 1.2)

# 1.2 (June 1st, 2018)

 * Tidied up error code handling, per version 1.2 of `libpulse-binding`
 * Improved time handling, per version 1.2 of `libpulse-binding`
 * Updated `libpulse-binding` version dependency (1.1 → 1.2)

# 1.1 (May 27th, 2018)

 * Privatised `SimpleInternal`
 * Updated `libpulse-binding` version dependency (1.0 → 1.1)
 * Updated `libpulse-sys` version dependency (1.0 → 1.1)
 * Updated `libpulse-simple-sys` version dependency (1.0 → 1.1)

# 1.0.5 (May 27th, 2018)

 * Enabled doc test

Note, some version numbers skipped, bumping number into line with the other crates

# 1.0.2 (February 9th, 2018)

 * Added travis badge

# 1.0.1 (February 2nd, 2018)

 * Fixed toml file license string `LGPL-2.1` → `LGPL-2.1+`
 * Fixed toml file missing author email address
 * Removed obsolete readme doc links

# 1.0 (January 24th, 2018)

 * Original release
