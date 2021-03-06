# icu-sys

This crate provides direct Rust bindings to the majority of the ICU C API.

This is partially derived from https://github.com/servo/rust-icu.

[Documentation](https://api.fullcontact.com/v3/docs/rustdoc/icu_sys/index.html)

## Getting `icu-sys`

`icu-sys` is not on crates.io because the ICU source exceeds the maximum
permitted size of a crate. Instead, you can add it as git dependency in
`Cargo.toml`:

```
[dependencies.icu-sys]
git = "https://github.com/fullcontact/icu-sys"
rev = "0.1.0"
```

## Available Modules

This crate does not currently include every module in ICU. The ones here are
the ones listed as C APIs under http://icu-project.org/apiref/icu4c/, minus
things so specific to C that they are not useful to bind or could not be bound
automatically, plus other modules as found to be necessary via dependencies
from other modules or by general usage.

The modules included are listed in [create-sources.sh](create-sources.sh).
Adding a new module is usually as simple as adding it to the list there and
regenerating the sources. Please feel free to make a PR if you find an addition
you need.

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
