# rsfunge

rsfunge is an interpreter written in Rust for Befunge-93 and the Funge-98 family of esoteric languages.

Note that rsfunge is still under developement!

Some features that are planned (or would be nice but require a lot of work):
* [ ] Finish the implementation of the spec
* [ ] Improve the cli
  * Add various environment options like "Disable concurrent funge", given via settings file or cli maybe
* [ ] Add a commandline debugger
  * Store breakpoints and debugging settings in a debugging file
* [ ] Implement standard fingerprints
* [ ] Actually support Befunge-93 (via constexprs and generics to make it fast)
* [ ] Maybe (really big maybe) implement a simple JIT
