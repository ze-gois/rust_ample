#[macro_export]
macro_rules! trait_implement_primitive_phantom_bytes {
    () => {
        impl<T> $crate::traits::Bytes<crate::Origin, crate::Origin> for core::marker::PhantomData<T> {
            const BYTES_SIZE: usize = 0;
            const BYTES_ALIGN: usize = 0;

            fn to_bytes(
                &self,
                _endianness: bool,
            ) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
                [0u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]
            }

            fn from_bytes(
                _bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE],
                _endianness: bool,
            ) -> Self {
                core::marker::PhantomData {}
            }
        }
    };
}
