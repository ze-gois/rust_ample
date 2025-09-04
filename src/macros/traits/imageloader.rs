// The trait_place_imageloader macro is already defined in imagetic.rs
// This module only provides the implementation macro

#[macro_export]
macro_rules! trait_implement_imageloader {
    () => {
        impl crate::ImageLoader<ample::Origin, ample::Origin> for ample::imagetic::ImageHandle {
            unsafe fn load_from_bytes(
                data: *const u8,
                size: usize,
            ) -> Option<ample::imagetic::Image<ample::Origin, ample::Origin>> {
                // Read header information
                if size < 13 {
                    return None;
                }

                // Verify magic number
                let magic = core::slice::from_raw_parts(data, 4);
                if magic != [b'I', b'M', b'G', 0] {
                    return None;
                }

                // Read width (little-endian u32)
                let width_bytes = core::slice::from_raw_parts(data.add(4), 4);
                let width = u32::from_le_bytes([
                    width_bytes[0],
                    width_bytes[1],
                    width_bytes[2],
                    width_bytes[3],
                ]) as usize;

                // Read height (little-endian u32)
                let height_bytes = core::slice::from_raw_parts(data.add(8), 4);
                let height = u32::from_le_bytes([
                    height_bytes[0],
                    height_bytes[1],
                    height_bytes[2],
                    height_bytes[3],
                ]) as usize;

                // Read bytes per pixel
                let bytes_per_pixel = *data.add(12);

                // Calculate expected data size
                let expected_size = 13 + (width * height * bytes_per_pixel as usize);
                if size < expected_size {
                    return None;
                }

                // Allocate memory for the image data
                let pixel_data_size = width * height * bytes_per_pixel as usize;
                let layout = ample::traits::allocatable::Layout {
                    size: pixel_data_size,
                    align: core::mem::align_of::<u8>(),
                };

                let image_data =
                    <u8 as crate::traits::Allocatable<ample::Origin, ample::Origin>>::allocate(
                        layout,
                    );
                if image_data.is_null() {
                    return None;
                }

                // Copy the pixel data
                core::ptr::copy_nonoverlapping(data.add(13), image_data, pixel_data_size);

                // Create and return the image
                Some(ample::imagetic::Image::new(
                    width,
                    height,
                    image_data,
                    bytes_per_pixel,
                ))
            }

            fn load_from_file(
                filepath: &str,
            ) -> Option<ample::imagetic::Image<ample::Origin, ample::Origin>> {
                // Use the existing file loading functionality
                let result = crate::file::load(filepath);

                match result {
                    Some((fd, stat, data)) => {
                        // Parse the image data
                        let image = unsafe { Self::load_from_bytes(data, stat.st_size as usize) };

                        // Close the file regardless of whether we successfully loaded the image
                        let _ = crate::target::os::syscall::close(fd as i32);

                        // Free the file mapping memory if image loading was successful
                        if image.is_some() {
                            let _ = crate::target::os::syscall::munmap(
                                data as *mut core::ffi::c_void,
                                stat.st_size as usize,
                            );
                        }

                        image
                    }
                    None => None,
                }
            }
        }
    };
}

pub use trait_implement_imageloader;
// We don't export trait_place_imageloader from here
// as it's already defined in imagetic.rs
