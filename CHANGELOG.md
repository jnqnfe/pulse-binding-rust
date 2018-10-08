This changelog now only covers changes that are not specific to the Rust crates that live in this
repository, i.e. project changes only, and maintains an overall project version number. See the
changelog files of the individual crates for crate specific changelog details.

# 2.2.4 (October 8th, 2018)

 * Travis: Simplified and now avoids sudo
 * Workspace: Simplified - specification of an actual `[package]` section it turns out is redundant
   alongside a `[workspace]` section.
 * Split the changelog file into separate changelogs for each crate and this one for the overall
   project for greater clarity. The overall project version number with associated git tags is still
   a potential source of confusion however.

Note, intermediate version numbers were crate specific only

# 2.1 (July 17th, 2018)

 * Renamed `NEWS` file to `CHANGELOG.md` and reformatted to markdown

# 2.0.1 (June 26th, 2018)

 * Updated copy of C includes from PA (version 11.0 → 12.0)

Note, intermediate version numbers were crate specific only

# 1.2 (June 1st, 2018)

 * Further licensing clarification

Note, intermediate version numbers were crate specific only

# 1.0.4d (February 15th, 2018)

 * Fixed Travis testing. Tests were not actually being run, giving a misleading positive result, and
   fixing that then highlighted problems with the test environment which took a few iterations to
   resolve (hence the 'd' in the version number from premature assumptions of resolving the
   problem).
    - Fixed tests not actually being run. I had incorrectly presumed that tests for all sub-crate
      dependencies of the workspace crate would be run automatically, but reviewing the logs
      highlighted that this was not the case; an explicit test script passing the `--all` flag to
      `cargo test` was needed.
    - Added missing `libpulse-mainloop-glib0` library to test environment
    - Actually added `pulseaudio` to the test environment and added command to start it

Note, intermediate version numbers were crate specific only

# 1.0 (January 24th, 2018)

 * Original release
