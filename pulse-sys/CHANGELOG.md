# ?? (???? ??, ????)

 * Added dedicated changelog, split off from the old single project overview one

# 1.3.1 (September 3rd, 2018)

 * Added homepage and repo links, thanks to @berkus on github for the patch

# 1.3 (July 17th, 2018)

 * Mainloop API objects now correctly treated as immutable, per related change in version 2.1 of
   `libpulse-binding`.
 * Default-enabled inclusion of the `pa_encoding_from_string` function symbol, which was missing
   from PA's symbol file and thus not available in the client library before v12.

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
