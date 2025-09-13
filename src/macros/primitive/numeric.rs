#[macro_export]
macro_rules! trait_implement_primitive_numeric_bytes {
    ($($($t:tt)::*),*) => {
        $(
            impl $crate::traits::Bytes<crate::Origin> for $($t)* {
                const BYTES_SIZE: usize = core::mem::size_of::<$($t)*>();
                const BYTES_ALIGN: usize = core::mem::align_of::<$($t)*>();

                fn to_bytes(&self, endianness: bool) -> [u8; <$($t)* as $crate::traits::Bytes<crate::Origin>>::BYTES_SIZE] {
                    if endianness {
                        core::primitive::$($t)*::to_le_bytes(*self)
                    } else {
                        core::primitive::$($t)*::to_be_bytes(*self)
                    }
                }

                fn from_bytes(bytes: [u8; <$($t)* as $crate::traits::Bytes<crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
                    if endianness {
                        core::primitive::$($t)*::from_le_bytes(bytes)
                    } else {
                        core::primitive::$($t)*::from_be_bytes(bytes)
                    }
                }
            }
        )*
    };
}
