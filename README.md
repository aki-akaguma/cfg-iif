# cfg-iif

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test][test-image]][test-link]

A macro for defining `#[cfg]` `if-else` functions.

The macro provided by this crate.
Unlike [`cfg_if`](https://crates.io/crates/cfg-if),
`cfg_iif` can be used as a function, and can be used in a function.

## Features

- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

## Example

### Example 1: `#[cfg()]`

- `a_iif` is "unix" when a os is Unix at compile time:
```rust
use cfg_iif::cfg_iif;
let a_iif = cfg_iif!(#[cfg(Unix)] { "unix" } else { "not unix" });
```

- `a_iif` is "abc" when a feature is "has_abc" at compile time:
```rust
use cfg_iif::cfg_iif;
let a_iif = cfg_iif!(#[cfg(feature = "has_abc")] { "abc" } else { "not abc" });
```

- `a_iif` is "abc" when a feature is "has_abc" at compile time:
```rust
use cfg_iif::cfg_iif;
let mut a_iif = "";
cfg_iif!(
    #[cfg(feature = "has_abc")]
    {
        a_iif = "abc";
    }
);
```

### Example 2: a short hand for a firendly `cargo fmt`

- `a_iif` is "abc" when a feature is "has_abc" at compile time:
```rust
use cfg_iif::cfg_iif;
let a_iif = cfg_iif!(feature = "has_abc" { "abc" } else { "not abc" });
```

- `a_iif` is "abc" when a feature is "has_abc" at compile time:
```rust
use cfg_iif::cfg_iif;
let mut a_iif = "";
cfg_iif!(feature = "has_abc" {
    a_iif = "abc";
});
```

[Documentation][docs-link]

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/cfg-iif/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/cfg-iif.svg
[crate-link]: https://crates.io/crates/cfg-iif
[docs-image]: https://docs.rs/cfg-iif/badge.svg
[docs-link]: https://docs.rs/cfg-iif/
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-image]: https://github.com/aki-akaguma/cfg-iif/actions/workflows/test.yml/badge.svg
[test-link]: https://github.com/aki-akaguma/cfg-iif/actions/workflows/test.yml
