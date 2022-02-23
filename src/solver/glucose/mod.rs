mod bindings {
    #![allow(
        unused,
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case,
    )]
    include!("./bindings.rs");
}

pub fn say_hello() {
    unsafe {
        bindings::say_hello();
    }
}

#[test]
fn glucose() {
    unsafe { bindings::say_hello() }
    assert_eq!(2 + 2, 4);
}
