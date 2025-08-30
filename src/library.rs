#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![allow(unused_assignments)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub use userspace::info;
pub use userspace::target;

pub fn x() {
    info!("number {}", 9);
}
