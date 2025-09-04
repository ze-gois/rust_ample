//! Imagetic types and traits for working with image data
//! This module defines the core interfaces for image processing functionality
//! that will be implemented in consuming crates.

/// Macro for defining the ImageLoader trait
#[macro_export]
macro_rules! trait_place_imageloader {
    () => {
        /// Core trait for image loading functionality that will be implemented elsewhere
        pub trait ImageLoader<Observer, Reference> {
            /// Loads image data from a raw byte pointer
            ///
            /// # Safety
            ///
            /// This function is unsafe because it dereferences a raw pointer.
            /// The caller must ensure that the pointer is valid and properly aligned.
            unsafe fn load_from_bytes(
                data: *const u8,
                size: usize,
            ) -> Option<$crate::imagetic::Image<Observer, Reference>>;

            /// Loads image data from a file path
            fn load_from_file(
                filepath: &str,
            ) -> Option<$crate::imagetic::Image<Observer, Reference>>;
        }
    };
}

// Define the trait
crate::trait_place_imageloader!();

/// Represents a basic image structure
pub struct Image<Observer, Reference> {
    /// Width of the image in pixels
    pub width: usize,

    /// Height of the image in pixels
    pub height: usize,

    /// Raw pixel data pointer
    pub data: *const u8,

    /// Number of bytes per pixel
    pub bytes_per_pixel: u8,

    /// Phantom data for type parameters
    _phantom: core::marker::PhantomData<(Observer, Reference)>,
}

impl<Observer, Reference> Image<Observer, Reference> {
    /// Creates a new image instance
    ///
    /// # Safety
    ///
    /// This function is unsafe because it stores a raw pointer.
    /// The caller must ensure that the pointer is valid and has proper lifetime.
    pub unsafe fn new(width: usize, height: usize, data: *const u8, bytes_per_pixel: u8) -> Self {
        Self {
            width,
            height,
            data,
            bytes_per_pixel,
            _phantom: core::marker::PhantomData,
        }
    }

    /// Gets the total size of the image data in bytes
    pub fn size_bytes(&self) -> usize {
        self.width * self.height * self.bytes_per_pixel as usize
    }

    /// Creates a null image (no data)
    pub fn null() -> Self {
        Self {
            width: 0,
            height: 0,
            data: core::ptr::null(),
            bytes_per_pixel: 0,
            _phantom: core::marker::PhantomData,
        }
    }
}

/// Loads image data using the appropriate loader implementation
pub fn load_image<Observer, Reference>(filepath: &str) -> Option<Image<Observer, Reference>>
where
    Observer: 'static,
    Reference: 'static,
{
    // This is a stub implementation that delegates to ImageHandle
    // The actual implementation will be provided by the consuming crate
    // that implements the necessary functionality

    // In practice, we'd convert between concrete and generic image types
    // but for the example we'll just return None
    None
}

/// A marker type that serves as a handler for image loading operations
pub struct ImageHandle;

impl ImageHandle {
    /// Loads an image from a file path
    pub fn load_from_file(_filepath: *const u8) -> Option<Image<crate::Origin, crate::Origin>> {
        // This implementation will be provided externally by consuming crates
        None
    }
}

/// Free image resources
pub fn free_image<Observer, Reference>(image: &mut Image<Observer, Reference>) {
    if !image.data.is_null() {
        // This will be implemented by the consuming crate
        ImageHandle::free_image_data(image);

        image.data = core::ptr::null();
        image.width = 0;
        image.height = 0;
        image.bytes_per_pixel = 0;
    }
}

impl ImageHandle {
    /// Frees image data
    pub fn free_image_data<Observer, Reference>(_image: &mut Image<Observer, Reference>) {
        // This implementation will be provided externally by consuming crates
    }
}
