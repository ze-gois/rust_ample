#[macro_export]
macro_rules! traits_implement_bytes_tuple {
    ($($($ordinal_type:tt)::*),*) => {
        impl $crate::traits::Bytes<crate::Origin, crate::Origin> for ($($ordinal_index:tt: $($ordinal_type)::*),*) {
            const BYTES_SIZE: usize = $(<$($ordinal_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE + )* 0;
            const BYTES_ALIGN: usize = $crate::expressions_upperbound!($(<$($ordinal_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_ALIGN),*);

            fn to_bytes(
                &self,
                endianness: bool,
            ) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]
            where
                [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
            {
                let mut pair_bytes = [0u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];

                let mut o = 0;
                let mut l = 0;
                $(
                    $(
                        o = l;
                        l = l + <$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                        pair_bytes[o..l].copy_from_slice(<$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(&self.$ordinal_index,endianness));
                    )*
                )*

                pair_bytes
            }

            fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self
            where
                [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
            {
                let mut o = 0;
                let mut l = 0;
                (
                    $(
                        $(
                            let mut ordinal_bytes = [0u8; <$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                            o = l;
                            l = l + <$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                            ordinal_bytes.copy_from_slice(bytes[o..l]);
                            <$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(ordinal_bytes, endianness)
                        )*
                    ),*
                )
            }
        }
    };
}
// pub use trait_implement_primitive_bytes;
pub use traits_implement_bytes_tuple;

// #[macro_export]
// macro_rules! trait_implement_primitive_unit_bytes {
//     () => {
//         impl $crate::traits::Bytes<crate::Origin, crate::Origin> for () {
//             const BYTES_SIZE: usize = core::mem::size_of::<()>();
//             const BYTES_ALIGN: usize = core::mem::align_of::<()>();

//             fn from_bytes(
//                 _bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE],
//                 _endianness: bool,
//             ) -> Self {
//                 ()
//             }

//             fn to_bytes(
//                 &self,
//                 _endianness: bool,
//             ) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
//                 [0u8; 0]
//             }

//             fn from_bytes_pointer(_bytes_pointer: *const u8, _endianness: bool) -> Self {
//                 ()
//             }
//         }
//     };
// }
