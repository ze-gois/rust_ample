#[macro_export]
macro_rules! trait_place_allocatable {
    () => {
        pub trait Allocatable<Reference, Observer>
        where
            Self: crate::traits::Bytes<Reference, Observer>,
        {
            /// Allocate `layout` bytes; return null on failure
            unsafe fn allocate(layout: $crate::traits::allocatable::Layout) -> *mut u8;

            /// Deallocate memory previously allocated
            unsafe fn deallocate(ptr: *mut u8, layout: $crate::traits::allocatable::Layout);

            /// Optional reallocate; fallback can be done in container
            unsafe fn reallocate(
                ptr: *mut u8,
                old_layout: $crate::traits::allocatable::Layout,
                new_layout: $crate::traits::allocatable::Layout,
            ) -> *mut u8 {
                // naive fallback: allocate new, copy, deallocate old
                let new_ptr = unsafe {
                    <Self as crate::traits::Allocatable<Reference, Observer>>::allocate(
                        new_layout.clone(),
                    )
                };

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

                unsafe {
                    <Self as crate::traits::Allocatable<Reference, Observer>>::deallocate(
                        ptr, old_layout,
                    )
                };
                new_ptr
            }

            unsafe fn allocate_zeroed(layout: $crate::traits::allocatable::Layout) -> *mut u8 {
                let ptr = unsafe {
                    <Self as crate::traits::Allocatable<Reference, Observer>>::allocate(layout)
                };
                if ptr.is_null() {
                    return core::ptr::null_mut();
                }

                unsafe {
                    core::ptr::write_bytes(ptr, 0, layout.size);
                }

                ptr
            }
        }
    };
}

pub use trait_place_allocatable;
