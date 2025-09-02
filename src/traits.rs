pub mod bytes;
pub mod enums;
pub mod primitive;

pub use bytes::Bytes;
pub use primitive::Primitive;

use core::alloc::Layout;

/// Trait that `ample` containers use to allocate memory
/// Must be implemented by each dependent crate
pub unsafe trait AmpleAlloc {
    /// Allocate `layout` bytes; return null on failure
    unsafe fn allocate(&self, layout: Layout) -> *mut u8;

    /// Deallocate memory previously allocated
    unsafe fn deallocate(&self, ptr: *mut u8, layout: Layout);

    /// Optional reallocate; fallback can be done in container
    unsafe fn reallocate(&self, ptr: *mut u8, old_layout: Layout, new_layout: Layout) -> *mut u8 {
        // naive fallback: allocate new, copy, deallocate old
        unsafe {
            let new_ptr = self.allocate(new_layout);
            if new_ptr.is_null() {
                return core::ptr::null_mut();
            }
            core::ptr::copy_nonoverlapping(
                ptr,
                new_ptr,
                core::cmp::min(old_layout.size(), new_layout.size()),
            );
            self.deallocate(ptr, old_layout);
            new_ptr
        }
    }
}
