#[macro_export]
macro_rules! trait_implement_primitive_bool_bytes {
    () => {
        impl $crate::traits::Bytes<crate::Origin> for bool {
            const BYTES_SIZE: usize = <u8 as $crate::traits::Bytes<crate::Origin>>::BYTES_SIZE;
            const BYTES_ALIGN: usize = <u8 as $crate::traits::Bytes<crate::Origin>>::BYTES_ALIGN;

            fn to_bytes(
                &self,
                _endianness: bool,
            ) -> [u8; <Self as $crate::traits::Bytes<crate::Origin>>::BYTES_SIZE] {
                <u8 as $crate::traits::Bytes<crate::Origin>>::to_le_bytes(&(*self as u8))
            }

            fn from_bytes(
                bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin>>::BYTES_SIZE],
                _endianness: bool,
            ) -> Self {
                bytes[0] != 0
            }
        }
    };
}
