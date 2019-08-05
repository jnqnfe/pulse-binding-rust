Tagging and version numbering confusion
=======================================

The version numbers used as tags within the repo began life reflecting a single overall version
number of the entire repo, being identical within all crates. This was never expected to become a
problem, not expecting so much work to end up being done here, and for *binding* and *sys* crate
versioning to diverge so much.

Since version numbering of crates started to diverge, version tagging remained associated with the
“higher-level” binding crates. Furthermore some times minor versions of the less commonly updated
*simple* and *glib-mainloop* crates are skipped over to keep versioning inline with the main crates,
to try to keep some sanity.

I am aware that this setup is an unfortunate potential source of confusion now.
