/*!
A macro for defining `#[cfg]` `if-else` functions.

The macro provided by this crate.
Unlike [`cfg_if`](https://crates.io/crates/cfg-if),
`cfg_iif` can be used as a function, and can be used in a function.

# Features

- minimum support rustc 1.60.0 (7737e0b5c 2022-04-04)
- support `else if` chains
- support multiple `cfg` predicates (implicitly combined with `all()`)

# Example

## Example 1: `#[cfg()]`

- `a_iif` is "unix" when a os is Unix at compile time:
```
use cfg_iif::cfg_iif;
let a_iif = cfg_iif!(#[cfg(target_family = "unix")] { "unix" } else { "not unix" });
```

- `a_iif` is "abc" when a feature is "has_abc" at compile time:
```
use cfg_iif::cfg_iif;
let a_iif = cfg_iif!(#[cfg(feature = "has_abc")] { "abc" } else { "not abc" });
```

- Using `else if` chains:
```
use cfg_iif::cfg_iif;
let result = cfg_iif!(
    #[cfg(target_os = "linux")] { "linux" }
    else if #[cfg(target_os = "windows")] { "windows" }
    else { "other" }
);
```

- Using multiple predicates:
```
use cfg_iif::cfg_iif;
let result = cfg_iif!(
    #[cfg(unix, target_pointer_width = "64")] { "64-bit unix" }
    else { "other" }
);
```

## Example 2: a short hand for a firendly `cargo fmt`

- `a_iif` is "abc" when a feature is "has_abc" at compile time:
```
use cfg_iif::cfg_iif;
let a_iif = cfg_iif!(feature = "has_abc" { "abc" } else { "not abc" });
```

- Using `else if` chains with shorthand:
```
use cfg_iif::cfg_iif;
let result = cfg_iif!(
    target_os = "linux" { "linux" }
    else if target_os = "windows" { "windows" }
    else { "other" }
);
```
*/
#![no_std]

/// This macro provided by this crate. See crate documentation for more information.
#[macro_export]
macro_rules! cfg_iif {
    // --- Internal arms ---

    // Final else
    (@inner ( $($prev:meta),+ ) else { $($it:tt)* }) => {
        #[cfg(not(any($($prev),*)))]
        { $($it)* }
    };

    // --- Standard Syntax ---

    // Else if
    (@inner ( $($prev:meta),+ ) else if #[cfg($($m:meta),*)] { $($it:tt)* } $($rest:tt)* ) => {
        #[cfg(all(not(any($($prev),*)), $($m),*))]
        { $($it)* }
        $crate::cfg_iif! { @inner ( $($prev,)* all($($m),*) ) $($rest)* }
    };

    // Initial if
    (@inner () #[cfg($($m:meta),*)] { $($it:tt)* } $($rest:tt)* ) => {
        #[cfg(all($($m),*))]
        { $($it)* }
        $crate::cfg_iif! { @inner (all($($m),*)) $($rest)* }
    };

    // --- Shorthand Syntax ---

    // Else if (Shorthand)
    (@inner ( $($prev:meta),+ ) else if $($m:meta),+ { $($it:tt)* } $($rest:tt)* ) => {
        #[cfg(all(not(any($($prev),*)), $($m),*))]
        { $($it)* }
        $crate::cfg_iif! { @inner ( $($prev,)* all($($m),*) ) $($rest)* }
    };

    // Initial if (Shorthand)
    (@inner () $($m:meta),+ { $($it:tt)* } $($rest:tt)* ) => {
        #[cfg(all($($m),*))]
        { $($it)* }
        $crate::cfg_iif! { @inner (all($($m),*)) $($rest)* }
    };

    // End
    (@inner ( $($prev:meta),* )) => {};

    // --- Entry Point ---
    ( $($t:tt)* ) => {{
        $crate::cfg_iif! { @inner () $($t)* }
    }};
}

#[cfg(test)]
mod tests {
    #![allow(unused_mut)]
    #![allow(unused_assignments)]
    //
    #[test]
    fn it_works_01() {
        let a = {
            #[cfg(target_family = "unix")]
            {
                "unix"
            }
            #[cfg(not(target_family = "unix"))]
            {
                "not unix"
            }
        };
        let a_iif = cfg_iif!(#[cfg(target_family = "unix")] { "unix" } else { "not unix" });
        assert_eq!(a_iif, a);
        //
        let a = {
            #[cfg(target_family = "windows")]
            {
                "windows"
            }
            #[cfg(not(target_family = "windows"))]
            {
                "not windows"
            }
        };
        let a_iif =
            cfg_iif!(#[cfg(target_family = "windows")] { "windows" } else { "not windows" });
        assert_eq!(a_iif, a);
        //
        let a = {
            #[cfg(feature = "has_abc")]
            {
                "abc"
            }
            #[cfg(not(feature = "has_abc"))]
            {
                "not abc"
            }
        };
        let a_iif = cfg_iif!(#[cfg(feature = "has_abc")] { "abc" } else { "not abc" });
        assert_eq!(a_iif, a);
        //
        let mut a = "";
        let mut a_iif = "";
        #[cfg(feature = "has_abc")]
        {
            a = "abc";
        };
        cfg_iif!(
            #[cfg(feature = "has_abc")]
            {
                a_iif = "abc";
            }
        );
        assert_eq!(a_iif, a);
    }
    //
    #[test]
    fn it_works_02() {
        let a = {
            #[cfg(feature = "has_abc")]
            {
                "abc"
            }
            #[cfg(not(feature = "has_abc"))]
            {
                "not abc"
            }
        };
        let a_iif = cfg_iif!(feature = "has_abc" { "abc" } else { "not abc" });
        assert_eq!(a_iif, a);
        //
        let mut a = "";
        let mut a_iif = "";
        #[cfg(feature = "has_abc")]
        {
            a = "abc";
        };
        cfg_iif!(feature = "has_abc" {
            a_iif = "abc";
        });
        assert_eq!(a_iif, a);
    }

    #[test]
    fn test_else_if_standard() {
        let result = cfg_iif!(
            #[cfg(target_os = "linux")] { "linux" }
            else if #[cfg(target_os = "windows")] { "windows" }
            else { "other" }
        );
        #[cfg(target_os = "linux")]
        assert_eq!(result, "linux");
        #[cfg(target_os = "windows")]
        assert_eq!(result, "windows");
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        assert_eq!(result, "other");
    }

    #[test]
    fn test_else_if_shorthand() {
        let result = cfg_iif!(
            target_os = "linux" { "linux" }
            else if target_os = "windows" { "windows" }
            else { "other" }
        );
        #[cfg(target_os = "linux")]
        assert_eq!(result, "linux");
        #[cfg(target_os = "windows")]
        assert_eq!(result, "windows");
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        assert_eq!(result, "other");
    }

    #[test]
    fn test_multiple_meta() {
        let result = cfg_iif!(#[cfg(unix, target_pointer_width = "64")] { "64-bit unix" } else { "other" });
        #[cfg(all(unix, target_pointer_width = "64"))]
        assert_eq!(result, "64-bit unix");
        #[cfg(not(all(unix, target_pointer_width = "64")))]
        assert_eq!(result, "other");
    }
}
