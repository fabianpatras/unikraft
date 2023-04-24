#![no_std]

extern crate ukrust_sys as ukrust;
// use ukrust::bindings::ENOMEM;

#[no_mangle]
fn get_random_number() -> i32 {
    4
}
