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
    (#[cfg($($cfg_meta:meta)*)] {
        $($it:tt)*
    }) => {{
        #[cfg($($cfg_meta)*)]
        {
            $($it)*
        }
    }};
}

#[cfg(test)]
mod tests {
    //
    #[test]
    fn it_works() {
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
    }
}
