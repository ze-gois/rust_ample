//! Fundamental types re-exported from `alloc`.
//!
//! `ample` does not reimplement the Rust allocation ecosystem for ordinary
//! use. These names provide one stable type vocabulary for consumers
//! such as `userspace` and `webspace`, while leaving room for experimental
//! structures elsewhere in the crate.

pub use alloc::boxed::Box;
pub use alloc::collections::{
    BTreeMap,
    BTreeSet,
    BinaryHeap,
    LinkedList,
    VecDeque,
};
pub use alloc::rc::Rc;
pub use alloc::string::String;
pub use alloc::vec::Vec;

/// Canonical linked-list structure exposed through the ample vocabulary.
pub type List<T> = LinkedList<T>;


pub mod list;
pub mod node;
pub mod string;
