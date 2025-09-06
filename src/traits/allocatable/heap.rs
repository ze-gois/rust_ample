use crate::traits::Allocatable;
use core::alloc::{GlobalAlloc, Layout};

// A marker trait for heap allocated types
pub trait HeapAllocatable: Allocatable {}

// A utility struct to enable heap allocation for implementing types
pub struct HeapAllocator;

// This is a simplified version. In a real implementation, you would use
// proper heap allocation mechanisms like System allocator or a custom one.
#[cfg(not(test))]
unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // This is a placeholder - in a real implementation you would
        // use the system allocator or other mechanisms
        unsafe extern "C" {
            fn malloc(size: usize) -> *mut u8;
        }
        unsafe { malloc(layout.size()) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe extern "C" {
            fn free(ptr: *mut u8);
        }
        unsafe { free(ptr) }
    }
}

// For tests, provide a mock allocator
#[cfg(test)]
unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // Simplified test implementation
        let mut v = Vec::<u8>::with_capacity(size + align);
        let ptr = v.as_mut_ptr();
        core::mem::forget(v);

        // Align the pointer
        let offset = align - (ptr as usize % align);
        if offset < align {
            unsafe { ptr.add(offset) }
        } else {
            ptr
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // In a test environment, we might choose not to deallocate
        // to simplify the testing
    }
}

// Static allocator instance
#[global_allocator]
pub static HEAP_ALLOCATOR: HeapAllocator = HeapAllocator;

// A generic implementation that can be used to wrap any type
// and provide heap allocation capabilities
pub struct HeapAllocated<T>(pub T);

impl<T: Clone> Allocatable for HeapAllocated<T> {
    fn allocate(n: usize) -> *const Self {
        unsafe {
            let layout = Layout::array::<Self>(n).unwrap_unchecked();
            HEAP_ALLOCATOR.alloc(layout) as *const Self
        }
    }

    fn deallocate(ptr: *const Self, n: usize) -> bool {
        unsafe {
            let layout = Layout::array::<Self>(n).unwrap_unchecked();
            HEAP_ALLOCATOR.dealloc(ptr as *mut u8, layout);
            true
        }
    }
}

impl<T: Clone> HeapAllocatable for HeapAllocated<T> {}
