#[macro_export]
macro_rules! trait_implement_primitive_slice_bytes {
    ($($($t:tt)::*),*) => {
        // $(
        //     impl $crate::traits::Bytes<crate::Origin, crate::Origin> for [&$($t)::*] {
        //         const BYTES_SIZE: usize = Self::len() * <$($t)* as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;

        //         fn to_bytes(&self, endianness: bool) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]
        //         where
        //             [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
        //         {
        //             if endianness {
        //                 usize::to_le_bytes(*self as usize)
        //             } else {
        //                 usize::to_be_bytes(*self as usize)
        //             }
        //         }

        //         fn from_bytes(bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self
        //         where
        //             [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
        //         {
        //             if endianness {
        //                 usize::from_le_bytes(bytes) as Self
        //             } else {
        //                 usize::from_be_bytes(bytes) as Self
        //             }
        //         }
        //     }

        // )*
    };
}
