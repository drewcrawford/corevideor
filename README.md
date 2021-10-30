# corevideor

Provides select Rust bindings for Apple [Core Video](https://developer.apple.com/documentation/corevideos) framework.  May be compared to [corevideo-sys](https://crates.io/crates/corevideo-sys) or [display-link](https://crates.io/crates/display-link).

Part of the [objr expanded universe](https://github.com/drewcrawford/objr#objr-expanded-universe), distinctive features of this library:

* Zero-cost abstractions.  Calling this library should perform identically to calling CoreGraphics from Swift/ObjC applications.
    * Most of the magic happens in [objr](https://github.com/drewcrawford/objr)
      which provide cutting-edge high-performance primitives which are used here extensively.
* Safe APIs.  Where possible APIs are designed with safe abstractions to provide familiar guarantees to Rust developers
* Low-level.  These bindings assume familiarity with bound APIs and are not documented separately.
* Free for noncommercial or "small commercial" use.
