#[macro_export]
macro_rules! traits_impl_blanket_bytes {
    ($identifier:ident) => {
        impl $crate::traits::Bytes<crate::Origin> for $identifier {
            const BYTES_ALIGN: usize = 0;
            const BYTES_SIZE: usize = 0;
            fn from_bytes(
                _bytes: [u8; <Self as ample::traits::Bytes<ample::Origin>>::BYTES_SIZE],
                _endianness: bool,
            ) -> Self
            where
                Self: Sized,
                [u8; <Self as ample::traits::Bytes<ample::Origin>>::BYTES_SIZE]:,
            {
                $identifier {}
            }
            fn to_bytes(
                &self,
                _endianness: bool,
            ) -> [u8; <Self as ample::traits::Bytes<ample::Origin>>::BYTES_SIZE]
            where
                Self: Sized,
                [u8; <Self as ample::traits::Bytes<ample::Origin>>::BYTES_SIZE]:,
            {
                [0; <Self as ample::traits::Bytes<ample::Origin>>::BYTES_SIZE]
            }
        }
    };
}
