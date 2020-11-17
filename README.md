# cfg-iif

*cfg-iif* is simple macro.

## Examples

```
let a_iif = cfg_iif!(#[cfg(Unix)] { "unix" } else { "not unix" });
```

```
let a_iif = cfg_iif!(#[cfg(feature = "has_abc")] { "abc" } else { "not abc" });
```

```
let a_iif = cfg_iif!(feature = "has_abc" { "abc" } else { "not abc" });
```
