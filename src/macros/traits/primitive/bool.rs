#[macro_export]
macro_rules! trait_implement_primitive_bool_bytes {
    () => {
        impl $crate::traits::Bytes<crate::Origin, crate::Origin> for bool {
            const BYTES_SIZE: usize = 1;
            fn to_bytes(&self, _endianness: bool) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]
            where
                [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
            {
                <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_le_bytes(&(*self as u8))
            }
            fn from_bytes(bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], _endianness: bool) -> Self
            where
                [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
            {
                bytes[0] != 0
            }
        }
    };
}
