# Simple Heartbeats Rust Bindings

The `hbs-sys` crate provides declarations and linkage for the `hbs` C
library.
Following the *-sys package conventions, this crate does not define
higher-level abstractions over the native library functions.

The latest `heartbeats-simple` C libraries can be found at
[https://github.com/libheartbeats/heartbeats-simple](https://github.com/libheartbeats/heartbeats-simple).

## Dependencies

In order to use this crate, you should have the `heartbeats-simple` libraries
installed to the system where they can be found by `pkg-config`.

If the libraries are not found, the build process will try to compile them.

## Usage
Add `hbs-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies]
hbs-sys = "0.2"
```
