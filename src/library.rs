#![no_std]
#![allow(incomplete_features)]
#![allow(unused_assignments)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]
#![feature(const_trait_impl)]
#![feature(fundamental)]
#![feature(generic_const_parameter_types)]

pub struct Origin {}

#[macro_use]
pub mod macros;
pub mod list;
pub mod node;
pub mod result;
pub mod string;
pub mod traits;
pub use string::String;

pub use result::{Error, Ok, Result};

trait_implement_primitives!();
