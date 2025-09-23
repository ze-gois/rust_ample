#[macro_export]
macro_rules! trait_implement_bytes_pointer {
    () => {
        impl<B> $crate::traits::Bytes<crate::Origin, crate::Origin> for *const B
        where
            B: $crate::traits::Bytes<crate::Origin, crate::Origin>,
        {
            const BYTES_SIZE: usize =
                <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
            const BYTES_ALIGN: usize =
                <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_ALIGN;

            fn to_bytes(
                &self,
                endianness: bool,
            ) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]
            {
                let mut bytes =
                    [0u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                let pointer_as_usize = &(*self as usize);
                let usize_bytes = if endianness {
                    <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_le_bytes(
                        pointer_as_usize,
                    )
                } else {
                    <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_be_bytes(
                        pointer_as_usize,
                    )
                };
                bytes.copy_from_slice(&usize_bytes);
                bytes
            }

            fn from_bytes(
                bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE],
                endianness: bool,
            ) -> Self {
                let mut usize_bytes = [0u8; <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                usize_bytes.copy_from_slice(&bytes);
                <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(usize_bytes, endianness) as Self
            }

            fn from_bytes_pointer(bytes_pointer: *const u8, endianness: bool) -> Self {
                let mut usize_bytes = [0u8; <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                unsafe { core::ptr::copy_nonoverlapping(bytes_pointer, usize_bytes.as_mut_ptr(), <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE) };
                <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(usize_bytes, endianness) as Self
            }
        }

        impl<B> $crate::traits::Bytes<crate::Origin, crate::Origin> for *mut B
        where
            B: $crate::traits::Bytes<crate::Origin, crate::Origin>,
        {
            const BYTES_SIZE: usize =
                <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
            const BYTES_ALIGN: usize =
                <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_ALIGN;

            fn to_bytes(
                &self,
                endianness: bool,
            ) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]
            {
                let mut bytes = [0u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                let usize_bytes = <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(&(*self as usize), endianness);
                bytes.copy_from_slice(&usize_bytes);
                bytes
            }

            fn from_bytes(
                bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE],
                endianness: bool,
            ) -> Self {
                let mut usize_bytes = [0u8; <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                usize_bytes.copy_from_slice(&bytes);
                <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(usize_bytes,endianness) as Self
            }

            fn from_bytes_pointer(bytes_pointer: *const u8, endianness: bool) -> Self {
                let mut usize_bytes = [0u8; <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                unsafe { core::ptr::copy_nonoverlapping(bytes_pointer, usize_bytes.as_mut_ptr(), <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE) };
                <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(usize_bytes, endianness) as Self
            }
        }
    };
}

// #[macro_export]
// macro_rules! trait_implement_primitive_pointer_bytes2 {
//     ($($($type:tt)::* $(<$($type_generics:tt),*>)? ),*) => {
//         $(
//             impl $crate::traits::Bytes<crate::Origin, crate::Origin> for *const $($type)::* {
//                 const BYTES_SIZE: usize = <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;

//                 fn to_bytes(&self, endianness: bool) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
//                     let pointer_as_usize = &(*self as usize);
//                     if endianness {
//                         <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_le_bytes(pointer_as_usize)
//                     } else {
//                         <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_be_bytes(pointer_as_usize)
//                     }
//                 }

//                 fn from_bytes(bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
//                     if endianness {
//                         <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(bytes) as Self
//                     } else {
//                         <usize as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_be_bytes(bytes) as Self
//                     }
//                 }
//             }

//             impl $crate::traits::Bytes<crate::Origin, crate::Origin> for *mut $($type)::* {
//                 const BYTES_SIZE: usize = core::mem::size_of::<usize>();

//                 fn to_bytes(&self, endianness: bool) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
//                     if endianness {
//                         usize::to_le_bytes(*self as usize)
//                     } else {
//                         usize::to_be_bytes(*self as usize)
//                     }
//                 }

//                 fn from_bytes(bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
//                     if endianness {
//                         usize::from_le_bytes(bytes) as Self
//                     } else {
//                         usize::from_be_bytes(bytes) as Self
//                     }
//                 }
//             }
//         )*
//     };
// }

// #[macro_export]
// macro_rules! trait_implement_primitive_pointer_bytes {
//     ($($($type:tt)::*),*) => {
//         $(
//             impl $crate::traits::Bytes<crate::Origin, crate::Origin> for *const $($type)::* {
//                 const BYTES_SIZE: usize = core::mem::size_of::<Self>();

//                 fn to_bytes(&self, endianness: bool) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
//                     if endianness {
//                         usize::to_le_bytes(*self as usize)
//                     } else {
//                         usize::to_be_bytes(*self as usize)
//                     }
//                 }

//                 fn from_bytes(bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
//                     if endianness {
//                         usize::from_le_bytes(bytes) as Self
//                     } else {
//                         usize::from_be_bytes(bytes) as Self
//                     }
//                 }
//             }

//         )*
//     };
// }
