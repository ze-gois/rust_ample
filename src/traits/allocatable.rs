pub mod layout;
pub use layout::Layout;

/// Trait that `ample` containers use to allocate memory
/// Must be implemented by each dependent crate
pub trait Allocatable {
    /// Allocate `layout` bytes; return null on failure
    fn allocate(&self, layout: Layout) -> *mut u8;

    /// Deallocate memory previously allocated
    fn deallocate(&self, ptr: *mut u8, layout: Layout);

    /// Optional reallocate; fallback can be done in container
    fn reallocate(&self, ptr: *mut u8, old_layout: Layout, new_layout: Layout) -> *mut u8 {
        // naive fallback: allocate new, copy, deallocate old
        let new_ptr = self.allocate(new_layout.clone());

        if new_ptr.is_null() {
            return core::ptr::null_mut();
        }

        unsafe {
            core::ptr::copy_nonoverlapping(
                ptr,
                new_ptr,
                core::cmp::min(old_layout.size, new_layout.size),
            );
        }

        self.deallocate(ptr, old_layout);
        new_ptr
    }
}
