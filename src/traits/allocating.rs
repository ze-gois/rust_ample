use core::alloc::Layout;

/// Capability implemented by an allocator.
///
/// # Safety
///
/// Implementors must return either a null pointer or storage satisfying the
/// supplied `Layout`. A pointer passed to `deallocate` must have been returned
/// by the same allocator for the same layout and must still designate the
/// original allocation.
pub unsafe trait Allocating {
    fn allocate(layout: Layout) -> *mut u8;

    unsafe fn deallocate(pointer: *mut u8, layout: Layout) -> bool;

    fn allocate_zeroed(layout: Layout) -> *mut u8 {
        let pointer = Self::allocate(layout);
        if pointer.is_null() {
            return pointer;
        }

        unsafe {
            core::ptr::write_bytes(pointer, 0, layout.size());
        }

        pointer
    }
}
