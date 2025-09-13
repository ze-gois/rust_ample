pub trait Allocatable<Origin> {
    /// Allocates memory according to the provided layout
    fn allocate(numerosity: usize) -> *mut Self;

    /// Deallocates memory at the given pointer with the specified layout
    fn deallocate(ptr: *mut Self, numerosity: usize) -> bool;

    /// Allocates zeroed memory according to the provided layout
    fn allocate_zeroed(numerosity: usize) -> *mut Self {
        let ptr = Self::allocate(numerosity);
        unsafe {
            core::ptr::write_bytes(ptr as *mut u8, 0, numerosity);
        }
        ptr
    }
}
