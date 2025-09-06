pub mod heap;
pub mod layout;
pub mod stack;

pub use layout::Layout;

use crate::traits::Bytes;

// crate::trait_place_allocatable!();

pub trait Allocatable<Reference, Observed>: Bytes<Reference, Observed> {
    fn allocate(layout: Layout) -> *const Self;
    fn deallocate(ptr: *const Self, layout: Layout) -> bool;

    fn allocate_zeroed(layout: Layout) -> *const Self {
        let ptr = Self::allocate(layout);
        unsafe {
            core::ptr::write_bytes(ptr as *mut u8, 0, layout.size);
        }
        ptr
    }
}
