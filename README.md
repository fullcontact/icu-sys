# icu-sys

This crate provides direct Rust bindings to the majority of the ICU C API.

This is partially derived from https://github.com/servo/rust-icu.

## Building

`cargo build` will build everything, including a static build of ICU without
symbol renaming. You need `make`, `gcc`, and `g++` for this.

## Updating ICU

The `icu` directory contains an unmodified copy of the ICU sources. They can be
upgraded as follows:

- Replace the `icu` directory with the new sources.

- Run `cargo build`. There's a decent chance this will fail; you don't need to
  address the errors immediately.

- Run `./create-sources.sh` to rebuild the bindings. You need to pass the path
  to the install prefix created by the prior step (see the script for an
  example). You need [bindgen](https://crates.io/crates/bindgen) installed for
  this step.

- Run `cargo build`. If any errors occur, address them by modifying
  `create-sources.sh` as needed, then go back to the previous step.

- Commit your changes, including the new generated sources.

## License

The bindings themselves are placed under the usual MIT/Apache-2.0 license. This
crate also includes the unmodified ICU library itself, which remains under the
ICU license (see [icu/license.html](icu/license.html) for details).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
