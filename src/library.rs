#![no_std]
#![allow(incomplete_features)]
#![allow(unused_assignments)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]
#![feature(const_trait_impl)]
#![feature(fundamental)]

pub struct Origin {}

pub mod list;
pub mod macros;
pub mod node;
pub mod string;
pub mod traits;

trait_implement_primitives!();
