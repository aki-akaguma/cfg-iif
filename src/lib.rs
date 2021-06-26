//! A macro for defining `#[cfg]` `if-else` functions.
//!
//! The macro provided by this crate.
//! Unlike [`cfg_if`](https://crates.io/crates/cfg-if),
//! `cfg_iif` can be used as a function, and can be used in a function.
//!
//! # Example
//!
//! ## Example 1: `#[cfg()]`
//!
//! - `a_iif` is "unix" when a os is Unix at compile time:
//! ```
//! use cfg_iif::cfg_iif;
//! let a_iif = cfg_iif!(#[cfg(Unix)] { "unix" } else { "not unix" });
//! ```
//!
//! - `a_iif` is "abc" when a feature is "has_abc" at compile time:
//! ```
//! use cfg_iif::cfg_iif;
//! let a_iif = cfg_iif!(#[cfg(feature = "has_abc")] { "abc" } else { "not abc" });
//! ```
//!
//! - `a_iif` is "abc" when a feature is "has_abc" at compile time:
//! ```
//! use cfg_iif::cfg_iif;
//! let mut a_iif = "";
//! cfg_iif!(
//!     #[cfg(feature = "has_abc")]
//!     {
//!         a_iif = "abc";
//!     }
//! );
//! ```
//!
//! ## Example 2: a short hand for a firendly `cargo fmt`
//!
//! - `a_iif` is "abc" when a feature is "has_abc" at compile time:
//! ```
//! use cfg_iif::cfg_iif;
//! let a_iif = cfg_iif!(feature = "has_abc" { "abc" } else { "not abc" });
//! ```
//!
//! - `a_iif` is "abc" when a feature is "has_abc" at compile time:
//! ```
//! use cfg_iif::cfg_iif;
//! let mut a_iif = "";
//! cfg_iif!(feature = "has_abc" {
//!     a_iif = "abc";
//! });
//! ```
//!
#![no_std]

/// This macro provided by this crate. See crate documentation for more information.
#[macro_export]
macro_rules! cfg_iif {
    // match `if-else` not chains with `else-if`
    (#[cfg($($cfg_meta:meta),*)] {
        $($true_it:tt)*
    } else {
        $($false_it:tt)*
    }) => {{
        #[cfg($($cfg_meta),*)]
        {
            $($true_it)*
        }
        #[cfg(not($($cfg_meta),*))]
        {
            $($false_it)*
        }
    }};
    //
    // match `if` not chains with `else`
    (#[cfg($($cfg_meta:meta),*)] {
        $($it:tt)*
    }) => {{
        #[cfg($($cfg_meta),*)]
        {
            $($it)*
        }
    }};
    //
    // match `if-else` not chains with `else-if`, more short hand
    ($($cfg_meta:meta),* {
        $($true_it:tt)*
    } else {
        $($false_it:tt)*
    }) => {{
        #[cfg($($cfg_meta),*)]
        {
            $($true_it)*
        }
        #[cfg(not($($cfg_meta),*))]
        {
            $($false_it)*
        }
    }};
    //
    // match `if` not chains with `else`, more short hand
    ($($cfg_meta:meta),* {
        $($it:tt)*
    }) => {{
        #[cfg($($cfg_meta),*)]
        {
            $($it)*
        }
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
            #[cfg(Unix)]
            {
                "unix"
            }
            #[cfg(not(Unix))]
            {
                "not unix"
            }
        };
        let a_iif = cfg_iif!(#[cfg(Unix)] { "unix" } else { "not unix" });
        assert_eq!(a_iif, a);
        //
        let a = {
            #[cfg(Windows)]
            {
                "windows"
            }
            #[cfg(not(Windows))]
            {
                "not windows"
            }
        };
        let a_iif = cfg_iif!(#[cfg(Unix)] { "windows" } else { "not windows" });
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
}
