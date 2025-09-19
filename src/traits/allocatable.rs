pub trait AllocatableResult {
    fn as_ptr(&self) -> *mut u8;
    fn from_raw(raw: *mut u8) -> Self;
}

pub trait Allocatable<Origin> {
    type Ok: AllocatableResult;
    type Error: AllocatableResult;

    /// Allocates memory according to the provided layout
    fn allocate(numerosity: usize) -> core::result::Result<Self::Ok, Self::Error>;

    /// Deallocates memory at the given pointer with the specified layout
    fn deallocate(ptr: *mut Self, numerosity: usize)
    -> core::result::Result<Self::Ok, Self::Error>;

    /// Allocates zeroed memory according to the provided layout
    fn allocate_zeroed(numerosity: usize) -> Result<Self::Ok, Self::Error> {
        let pointer = Self::allocate(numerosity)?.as_ptr();
        for p in 0..numerosity {
            unsafe {
                *(pointer.add(p)) = 0;
            }
        }
        core::result::Result::Ok(Self::Ok::from_raw(pointer))
    }
}
