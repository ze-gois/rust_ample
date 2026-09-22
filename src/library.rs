#![no_std]
#![allow(incomplete_features)]
#![allow(unused_assignments)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]
#![feature(generic_const_parameter_types)]

extern crate alloc;

pub struct Origin {}

#[macro_use]
pub mod macros;
pub mod math;
pub mod result;
pub mod r#struct;
pub mod traits;

pub use result::{Error, Ok, Result};
pub use r#struct::{BTreeMap, BTreeSet, BinaryHeap, Box, LinkedList, List, Rc, String, Vec, VecDeque};

trait_implement_primitives!();
