#[macro_export]
macro_rules! trait_implement_primitive_unit_bytes {
    () => {
        impl crate::traits::Bytes<crate::Origin> for () {
            const BYTES_SIZE: usize = core::mem::size_of::<()>();
            const BYTES_ALIGN: usize = core::mem::align_of::<()>();

            fn from_bytes(
                _bytes: [u8; <Self as crate::traits::Bytes<crate::Origin>>::BYTES_SIZE],
                _endianness: bool,
            ) -> Self {
                ()
            }

            fn to_bytes(
                &self,
                _endianness: bool,
            ) -> [u8; <Self as crate::traits::Bytes<crate::Origin>>::BYTES_SIZE] {
                [0u8; 0]
            }
        }
    };
}
