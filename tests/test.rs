#![allow(unused_mut)]
#![allow(unused_assignments)]

use cfg_iif::cfg_iif;

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
