#![no_std]

extern crate ukrust;
use ukrust::ENOMEM;

#[no_mangle]
fn get_random_number() -> i32 {
    4
}
