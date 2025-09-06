#![no_std]
#![allow(incomplete_features)]
#![allow(unused_assignments)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub struct Origin {}

// pub mod imagetic;
// pub mod list;
pub mod macros;
// pub mod node;
// pub mod queue;
// pub mod result;
// pub mod string;
pub mod traits;

// pub use queue::Queue;

trait_implement_primitives!();
