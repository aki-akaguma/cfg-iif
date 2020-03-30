#![no_std]

#[macro_export]
macro_rules! cfg_iif {
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
    (#[cfg($($cfg_meta:meta),*)] {
        $($it:tt)*
    }) => {{
        #[cfg($($cfg_meta),*)]
        {
            $($it)*
        }
    }};
    //
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
    #[test]
    fn it_works_02() {
    /* deprecated from rustc 1.42.0
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
        let a_iif = cfg_iif!(Unix { "unix" } else { "not unix" });
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
        let a_iif = cfg_iif!(Unix { "windows" } else { "not windows" });
        assert_eq!(a_iif, a);
    */
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
