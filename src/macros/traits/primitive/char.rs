#[macro_export]
macro_rules! trait_implement_primitive_char_bytes {
    () => {
        impl $crate::traits::Bytes<crate::Origin, crate::Origin> for char {
            const BYTES_SIZE: usize = 4;

            fn to_bytes(&self, endianness: bool) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
                if endianness {
                    <u32 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_le_bytes(&(*self as u32))
                } else {
                    <u32 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_be_bytes(&(*self as u32))
                }
            }
            fn from_bytes(bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
                let val = if endianness {
                    <u32 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(bytes)
                } else {
                    <u32 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_be_bytes(bytes)
                };
                core::char::from_u32(val).unwrap()
            }
        }
    };
}
