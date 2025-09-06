use crate::traits::Allocatable;
use core::cell::Cell;
use core::mem;
use core::ptr;

// A marker trait for stack-allocated types
pub trait StackAllocatable: Allocatable {}

// A fixed-size stack buffer that can be used for allocation
pub struct StackBuffer<const N: usize> {
    buffer: [u8; N],
    offset: Cell<usize>,
}

impl<const N: usize> StackBuffer<N> {
    pub const fn new() -> Self {
        Self {
            buffer: [0; N],
            offset: Cell::new(0),
        }
    }

    pub fn allocate<T>(&self, count: usize) -> *const T {
        let type_size = mem::size_of::<T>();
        let type_align = mem::align_of::<T>();
        let size_bytes = type_size * count;

        // Calculate the aligned offset
        let current_offset = self.offset.get();
        let align_padding = (type_align - (current_offset % type_align)) % type_align;
        let new_offset = current_offset + align_padding;

        // Check if we have enough space
        if new_offset + size_bytes > N {
            return ptr::null();
        }

        // Update the offset and return the pointer
        let ptr = unsafe { self.buffer.as_ptr().add(new_offset) } as *const T;
        self.offset.set(new_offset + size_bytes);

        ptr
    }

    pub fn reset(&self) {
        self.offset.set(0);
    }
}

// A wrapper for stack-allocated types
pub struct StackAllocated<T, const N: usize> {
    value: T,
    _buffer: &'static StackBuffer<N>, // Renamed with underscore to indicate intentionally unused
}

impl<T, const N: usize> StackAllocated<T, N> {
    pub fn new(value: T, buffer: &'static StackBuffer<N>) -> Self {
        Self {
            value,
            _buffer: buffer,
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T, const N: usize> Allocatable for StackAllocated<T, N> {
    fn allocate(_n: usize) -> *const Self {
        // This is a placeholder - real implementation would use the buffer
        // For now, we just return null to indicate this isn't a typical heap allocation
        ptr::null()
    }

    fn deallocate(_ptr: *const Self, _n: usize) -> bool {
        // Stack deallocation happens automatically when going out of scope
        // so we don't need to do anything here
        true
    }
}

impl<T, const N: usize> StackAllocatable for StackAllocated<T, N> {}

// Helper macro to create a static stack buffer
#[macro_export]
macro_rules! define_stack_buffer {
    ($name:ident, $size:expr) => {
        static $name: $crate::traits::allocatable::stack::StackBuffer<$size> =
            $crate::traits::allocatable::stack::StackBuffer::new();
    };
}
