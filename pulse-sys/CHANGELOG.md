# <unreleased>

 * Extended support to older versions of PA, specifically v4

# 1.8.1 (August 17th, 2019)

 * Version: Improved mod documentation

# 1.8.0 (August 15th, 2019)

 * Updated `use` conventions to that of Rust 1.30/1.31
 * Specified edition in toml file
 * Made the following `const` functions:
    - `version::get_compatibility()`
    - `channelmap::pa_channel_position_mask()`
    - `volume::pa_volume_is_valid()`
    - `context::subscribe::pa_subscription_match_flags()`
 * Version: purged deprecated items

Note: version 1.7 skipped, used only for `libpulse-mainloop-glib-sys` crate changes

# 1.6.0 (August 12th, 2019)

 * Replaced use of empty enums for opaque types with a struct based alternative. According to the
   Rust nomicon ([here](https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs))
   the use of the empty enum trick is apparently undefined behaviour.
 * Added a new `latest_pa_common_compatibility` feature flag, used by default now instead of
   `latest_pa_compatibility`.

# 1.5.0 (December 22nd, 2018)

 * Added the new `latest_pa_compatibility` and `pa_v12_compatibility` feature flags, and deprecated
   `pa_encoding_from_string` in favour of `pa_v12_compatibility`.

# 1.4.0 (November 4th, 2018)

 * Switched to using package-config for linking (though retaining the direct linking as a fallback
   for those without the necessary *.pc files installed). Thanks to @JohnAZoidberg on github for
   the original patch.
 * Improved the `version` mod:
    - Constants now vary depending upon backwards compatibility flags, correctly indicating the
      newest supported PA version.
    - Added the `Compatibility` enum and `get_compatibility` function
    - Renamed `LINK_TARGET_VERSION` to `TARGET_VERSION_STRING`
    - Introduced `TARGET_VERSION` and deprecated `PA_MAJOR`, `PA_MINOR` and `PA_MICRO`
    - Deprecated `get_headers_version`
 * Clarified PA version compatibility in `version` mod
 * Clarified `pa_encoding_from_string` feature purpose

# 1.3.4 (October 8th, 2018)

 * Fixed broken attempt to include license file in bundled package

# 1.3.3 (October 8th, 2018)

 * Added dedicated changelog, split off from the old single project overview one
 * Included copy of license file in bundled package and excluded the `.gitignore` file

Note, version number 1.3.2 skipped, bumping number into line with the other crates

# 1.3.1 (September 3rd, 2018)

 * Added homepage and repo links, thanks to @berkus on github for the patch

# 1.3 (July 17th, 2018)

 * Mainloop API objects now correctly treated as immutable, per related change in version 2.1 of
   `libpulse-binding`.
 * Default-enabled inclusion of the `pa_encoding_from_string` function symbol, which was missing
   from PA’s symbol file and thus not available in the client library before v12.

# 1.2.1 (June 26th, 2018)

 * Updated declared PA version compatibility (11.0 → 12.0)

# 1.2 (June 16th, 2018)

 * Context: Handful of functions changed to take `const` pointers.
   In version 1.1 many functions throughout the API were changed to take `const` pointers, with
   respective patches sent in to PA itself (which have since been accepted). Some context related
   functions were skipped over then however due to a complication with an artefact relating to
   validation checks. Additional patches solving this have now been created and sent in to the PA
   project. Discussion with PA devs indicates that this change will be accepted, so pre-emtively
   pushing the change here in our representation of the API; logically they should be immutable, and
   we do not need to propagate this unfortunate artefact).
 * Introspect & subscribe: Purged autoload API (deprecated in PA since 2009)

# 1.1 (May 27th, 2018)

 * Various functions have been changed to take immutable `const` pointers.
   There are numerous functions in the C API which take mutable pointers to objects where there is
   no intention to actually mutate those objects. Patches have been sent in to the PA project to
   correct many of these cases. There was no point in waiting for those to be accepted before
   fixing our representation of the API here, since the change is so obviously correct.

# 1.0.5 (May 27th, 2018)

 * Minor, non-functional consistency fix only

Note, some version numbers skipped, bumping number into line with the other crates

# 1.0.2 (February 9th, 2018)

 * Added travis badge

# 1.0.1 (February 2nd, 2018)

 * Fixed toml file license string `LGPL-2.1` → `LGPL-2.1+`
 * Fixed toml file missing author email address

# 1.0 (January 24th, 2018)

 * Original release
