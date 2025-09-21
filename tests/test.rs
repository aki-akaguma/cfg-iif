#![allow(unused_mut)]
#![allow(unused_assignments)]

use cfg_iif::cfg_iif;

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
    let a_iif = cfg_iif!(#[cfg(target_family = "windows")] { "windows" } else { "not windows" });
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

#[allow(unexpected_cfgs)]
#[test]
fn it_works_03() {
    // Test with target_os
    let os_name = cfg_iif!(#[cfg(target_os = "linux")] { "Linux" } else { "Other OS" });
    #[cfg(target_os = "linux")]
    assert_eq!(os_name, "Linux");
    #[cfg(not(target_os = "linux"))]
    assert_eq!(os_name, "Other OS");

    // Test with debug_assertions
    let debug_mode = cfg_iif!(#[cfg(debug_assertions)] { true } else { false });
    #[cfg(debug_assertions)]
    assert!(debug_mode);
    #[cfg(not(debug_assertions))]
    assert!(!debug_mode);

    // Test if-only syntax with target_os
    let mut message = String::from("Initial");
    cfg_iif!(
        #[cfg(target_os = "macos")]
        {
            message = String::from("macOS detected");
        }
    );
    #[cfg(target_os = "macos")]
    assert_eq!(message, "macOS detected");
    #[cfg(not(target_os = "macos"))]
    assert_eq!(message, "Initial");

    // Test shorthand with target_os
    let os_shorthand = cfg_iif!(target_os = "windows" { "Windows" } else { "Not Windows" });
    #[cfg(target_os = "windows")]
    assert_eq!(os_shorthand, "Windows");
    #[cfg(not(target_os = "windows"))]
    assert_eq!(os_shorthand, "Not Windows");

    // Test shorthand if-only with target_os
    let mut shorthand_message = String::from("Start");
    cfg_iif!(target_os = "android" {
        shorthand_message = String::from("Android detected");
    });
    #[cfg(target_os = "android")]
    assert_eq!(shorthand_message, "Android detected");
    #[cfg(not(target_os = "android"))]
    assert_eq!(shorthand_message, "Start");

    // Test with different types (numbers)
    let value = cfg_iif!(#[cfg(feature = "some_feature")] { 100 } else { 200 });
    #[cfg(feature = "some_feature")]
    assert_eq!(value, 100);
    #[cfg(not(feature = "some_feature"))]
    assert_eq!(value, 200);

    // Test with different types (structs/enums - simple example)
    #[allow(dead_code)]
    enum Config {
        Debug,
        Release,
    }
    let config_type = cfg_iif!(#[cfg(debug_assertions)] { Config::Debug } else { Config::Release });
    #[cfg(debug_assertions)]
    match config_type {
        Config::Debug => (),
        _ => unreachable!(),
    }
    #[cfg(not(debug_assertions))]
    match config_type {
        Config::Release => (),
        _ => unreachable!(),
    }
}

#[allow(unexpected_cfgs)]
#[test]
fn it_works_04() {
    // Test with multiple cfg attributes (AND condition)
    let complex_cfg = cfg_iif!(#[cfg(all(target_family = "unix", debug_assertions))] { "Unix Debug" } else { "Other" });
    #[cfg(all(target_family = "unix", debug_assertions))]
    assert_eq!(complex_cfg, "Unix Debug");
    #[cfg(not(all(target_family = "unix", debug_assertions)))]
    assert_eq!(complex_cfg, "Other");

    // Test with multiple cfg attributes (OR condition)
    let complex_cfg_or = cfg_iif!(#[cfg(any(target_os = "windows", feature = "another_feature"))] { "Windows or Feature" } else { "Neither" });
    #[cfg(any(target_os = "windows", feature = "another_feature"))]
    assert_eq!(complex_cfg_or, "Windows or Feature");
    #[cfg(not(any(target_os = "windows", feature = "another_feature")))]
    assert_eq!(complex_cfg_or, "Neither");

    // Test with not()
    let not_cfg = cfg_iif!(#[cfg(not(target_family = "windows"))] { "Not Windows Family" } else { "Windows Family" });
    #[cfg(not(target_family = "windows"))]
    assert_eq!(not_cfg, "Not Windows Family");
    #[cfg(target_family = "windows")]
    assert_eq!(not_cfg, "Windows Family");
}

#[allow(unexpected_cfgs)]
#[test]
fn it_works_05() {
    // Test with complex expressions in branches
    let result_str = cfg_iif!(#[cfg(target_pointer_width = "64")] {
        format!("Pointer width is {}", 64)
    } else {
        String::from("Pointer width is not 64")
    });
    #[cfg(target_pointer_width = "64")]
    assert_eq!(result_str, "Pointer width is 64");
    #[cfg(not(target_pointer_width = "64"))]
    assert_eq!(result_str, "Pointer width is not 64");

    // Test with multiple statements in a block
    let mut x = 0;
    let y = cfg_iif!(#[cfg(unix)] {
        x += 1;
        x + 10
    } else {
        x += 2;
        x + 20
    });
    #[cfg(unix)]
    {
        assert_eq!(x, 1);
        assert_eq!(y, 11);
    }
    #[cfg(not(unix))]
    {
        assert_eq!(x, 2);
        assert_eq!(y, 22);
    }

    // Test variable shadowing (should not be an issue as macro expands to cfg blocks)
    let shadow_var = 10;
    let shadowed_result = cfg_iif!(#[cfg(target_endian = "little")] {
        let shadow_var = 20;
        shadow_var + 1
    } else {
        shadow_var + 2
    });
    #[cfg(target_endian = "little")]
    assert_eq!(shadowed_result, 21);
    #[cfg(not(target_endian = "little"))]
    assert_eq!(shadowed_result, 12);
    assert_eq!(shadow_var, 10); // Original variable should be unchanged

    // Test with function calls in branches
    #[allow(dead_code)]
    fn get_value_a() -> i32 {
        100
    }
    #[allow(dead_code)]
    fn get_value_b() -> i32 {
        200
    }
    let func_call_result = cfg_iif!(#[cfg(feature = "test_feature_a")] {
        get_value_a() * 2
    } else {
        get_value_b() / 2
    });
    #[cfg(feature = "test_feature_a")]
    assert_eq!(func_call_result, 200);
    #[cfg(not(feature = "test_feature_a"))]
    assert_eq!(func_call_result, 100);
}

#[test]
fn it_works_06() {
    // Test with different cfg attributes: target_vendor
    let vendor = cfg_iif!(#[cfg(target_vendor = "apple")] { "Apple" } else { "Non-Apple" });
    #[cfg(target_vendor = "apple")]
    assert_eq!(vendor, "Apple");
    #[cfg(not(target_vendor = "apple"))]
    assert_eq!(vendor, "Non-Apple");

    // Test with different cfg attributes: target_env
    let env_type = cfg_iif!(#[cfg(target_env = "msvc")] { "MSVC" } else { "GNU or other" });
    #[cfg(target_env = "msvc")]
    assert_eq!(env_type, "MSVC");
    #[cfg(not(target_env = "msvc"))]
    assert_eq!(env_type, "GNU or other");

    // Test with different cfg attributes: target_arch
    let arch = cfg_iif!(#[cfg(target_arch = "x86_64")] { "x86_64" } else { "Other Arch" });
    #[cfg(target_arch = "x86_64")]
    assert_eq!(arch, "x86_64");
    #[cfg(not(target_arch = "x86_64"))]
    assert_eq!(arch, "Other Arch");
}

#[allow(unexpected_cfgs)]
#[test]
fn it_works_07() {
    // Test with character literals
    let char_val = cfg_iif!(#[cfg(target_os = "freebsd")] { 'f' } else { 'o' });
    #[cfg(target_os = "freebsd")]
    assert_eq!(char_val, 'f');
    #[cfg(not(target_os = "freebsd"))]
    assert_eq!(char_val, 'o');

    // Test with tuple literals
    let tuple_val = cfg_iif!(#[cfg(target_arch = "arm")] { (1, "arm") } else { (0, "other") });
    #[cfg(target_arch = "arm")]
    assert_eq!(tuple_val, (1, "arm"));
    #[cfg(not(target_arch = "arm"))]
    assert_eq!(tuple_val, (0, "other"));

    // Test with array literals
    let array_val = cfg_iif!(#[cfg(target_family = "wasm")] { [1, 2, 3] } else { [4, 5, 6] });
    #[cfg(target_family = "wasm")]
    assert_eq!(array_val, [1, 2, 3]);
    #[cfg(not(target_family = "wasm"))]
    assert_eq!(array_val, [4, 5, 6]);

    // Test with nested cfg attributes in the condition
    let nested_cfg_condition =
        cfg_iif!(#[cfg(all(unix, target_pointer_width = "32"))] { "Unix 32-bit" } else { "Other" });
    #[cfg(all(unix, target_pointer_width = "32"))]
    assert_eq!(nested_cfg_condition, "Unix 32-bit");
    #[cfg(not(all(unix, target_pointer_width = "32")))]
    assert_eq!(nested_cfg_condition, "Other");

    // Test with empty blocks for if-only (should compile and do nothing)
    let mut flag = false;
    cfg_iif!(
        #[cfg(feature = "non_existent_feature")]
        {}
    );
    assert!(!flag); // Should remain false as feature is not enabled

    // Test with empty blocks for if-else (should compile if both branches are empty, but usually not useful)
    #[allow(clippy::let_unit_value)]
    let result_empty = cfg_iif!(#[cfg(feature = "another_non_existent_feature")] {} else {});
    // This will result in a unit type `()` which is fine.
    assert_eq!(result_empty, ());
}

#[test]
fn it_works_08() {
    // Test with raw string literals
    let raw_string = cfg_iif!(#[cfg(target_os = "ios")] { r"iOS path" } else { r"Other path" });
    #[cfg(target_os = "ios")]
    assert_eq!(raw_string, "iOS path");
    #[cfg(not(target_os = "ios"))]
    assert_eq!(raw_string, "Other path");

    // Test with byte string literals
    let byte_string = cfg_iif!(#[cfg(target_os = "android")] { b"android" } else { b"other" });
    #[cfg(target_os = "android")]
    assert_eq!(byte_string, b"android");
    #[cfg(not(target_os = "android"))]
    assert_eq!(byte_string, b"other");

    // Test with a more complex expression that returns a struct
    struct MyData {
        id: u32,
        name: &'static str,
    }
    let data = cfg_iif!(#[cfg(target_endian = "big")] {
        MyData { id: 1, name: "Big Endian" }
    } else {
        MyData { id: 2, name: "Little Endian" }
    });
    #[cfg(target_endian = "big")]
    {
        assert_eq!(data.id, 1);
        assert_eq!(data.name, "Big Endian");
    }
    #[cfg(not(target_endian = "big"))]
    {
        assert_eq!(data.id, 2);
        assert_eq!(data.name, "Little Endian");
    }
}

#[test]
fn it_works_09() {
    // Test with different visibility modifiers (within a function, so not directly applicable to macro output, but context)
    // The macro itself expands to code within the current scope, so visibility is determined by the surrounding code.
    // This test primarily ensures the macro doesn't break when used in a function.
    let public_val = cfg_iif!(#[cfg(target_os = "freebsd")] { "FreeBSD" } else { "Not FreeBSD" });
    #[cfg(target_os = "freebsd")]
    assert_eq!(public_val, "FreeBSD");
    #[cfg(not(target_os = "freebsd"))]
    assert_eq!(public_val, "Not FreeBSD");

    // Test within a nested module (simulated by a block here)
    {
        let module_val = cfg_iif!(#[cfg(target_arch = "powerpc")] { 123 } else { 456 });
        #[cfg(target_arch = "powerpc")]
        assert_eq!(module_val, 123);
        #[cfg(not(target_arch = "powerpc"))]
        assert_eq!(module_val, 456);
    }

    // Test with a more complex custom type (Vec)
    let vec_val = cfg_iif!(#[cfg(target_endian = "big")] {
        vec![1, 2, 3]
    } else {
        vec![4, 5, 6]
    });
    #[cfg(target_endian = "big")]
    assert_eq!(vec_val, vec![1, 2, 3]);
    #[cfg(not(target_endian = "big"))]
    assert_eq!(vec_val, vec![4, 5, 6]);

    // Test with different types in if/else branches (should cause a compile error if uncommented)
    // let type_mismatch = cfg_iif!(#[cfg(feature = "some_other_feature")] { 1 } else { "string" });
    // This would result in a compile-time error: expected integer, found `&str`
    // The macro itself should not prevent this error, but rather let the Rust compiler catch it.
}

#[test]
fn it_works_10() {
    // Test with const items
    const CONST_VAL_A: &str =
        cfg_iif!(#[cfg(target_os = "linux")] { "Linux Const" } else { "Other Const" });
    #[cfg(target_os = "linux")]
    assert_eq!(CONST_VAL_A, "Linux Const");
    #[cfg(not(target_os = "linux"))]
    assert_eq!(CONST_VAL_A, "Other Const");

    // Test with static items
    static STATIC_VAL_A: &str =
        cfg_iif!(#[cfg(target_os = "windows")] { "Windows Static" } else { "Other Static" });
    #[cfg(target_os = "windows")]
    assert_eq!(STATIC_VAL_A, "Windows Static");
    #[cfg(not(target_os = "windows"))]
    assert_eq!(STATIC_VAL_A, "Other Static");

    // Test with a more complex cfg attribute and a struct
    struct Point {
        x: i32,
        y: i32,
    }
    let point = cfg_iif!(#[cfg(all(unix, target_pointer_width = "64"))] {
        Point { x: 10, y: 20 }
    } else {
        Point { x: 30, y: 40 }
    });
    #[cfg(all(unix, target_pointer_width = "64"))]
    {
        assert_eq!(point.x, 10);
        assert_eq!(point.y, 20);
    }
    #[cfg(not(all(unix, target_pointer_width = "64")))]
    {
        assert_eq!(point.x, 30);
        assert_eq!(point.y, 40);
    }
}
