# cfg-iif

A macro for defining `#[cfg]` `if-else` functions.

The macro provided by this crate.
Unlike [`cfg_if`](https://crates.io/crates/cfg-if),
`cfg_iif` can be used as a function, and can be used in a function.

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


# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.
