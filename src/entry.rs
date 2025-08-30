#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![allow(unused_assignments)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

use ample;

#[unsafe(no_mangle)]
pub extern "C" fn entry(_stack_pointer: userspace::target::arch::PointerType) -> ! {
    userspace::file::print("Cargo.toml");
    ample::x();
    userspace::target::os::syscall::exit(30);
}
