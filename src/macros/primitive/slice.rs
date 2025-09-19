#[macro_export]
macro_rules! trait_implement_primitive_array_bytes {
    ($($($t:tt)::* $(<$($type_generics:tt),*>)? ),*) => {
        $(
            impl <const N: usize> crate::traits::Bytes<crate::Origin> for [$($t)::*; N]
            {
                const BYTES_SIZE: usize =
                    N * <$($t)::* as crate::traits::Bytes<crate::Origin>>::BYTES_SIZE;
                const BYTES_ALIGN: usize = <$($t)::* as crate::traits::Bytes<crate::Origin>>::BYTES_ALIGN;

                fn to_bytes(
                    &self,
                    endianness: bool,
                ) -> [u8; <Self as crate::traits::Bytes<crate::Origin>>::BYTES_SIZE]
                where
                    [(); <Self as crate::traits::Bytes<crate::Origin>>::BYTES_SIZE]:,
                {
                    let mut bytes = [0u8; <Self as crate::traits::Bytes<crate::Origin>>::BYTES_SIZE];
                    let item_size = <$($t)::* as crate::traits::Bytes<crate::Origin>>::BYTES_SIZE;

                    for (i, item) in self.iter().enumerate() {
                        let item_bytes = <$($t)::* as crate::traits::Bytes<crate::Origin>>::to_bytes(
                            item,
                            endianness
                        );
                        let start = i * item_size;
                        bytes[start..start + item_size].copy_from_slice(&item_bytes);
                    }
                    bytes
                }

                fn from_bytes(bytes: [u8; <Self as crate::traits::Bytes<crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self
                where
                    [(); <Self as crate::traits::Bytes<crate::Origin>>::BYTES_SIZE]:,
                {
                    const NN: usize = <$($t)::* as crate::traits::Bytes<crate::Origin>>::BYTES_SIZE;
                    let defaulta = <$($t)::* as crate::traits::Bytes<crate::Origin>>::from_bytes([0u8; NN], endianness);
                    let mut arr: [$($t)::*; N] = [defaulta; N];
                    for (i, chunk) in bytes.chunks_exact(<$($t)::* as crate::traits::Bytes<crate::Origin>>::BYTES_SIZE).enumerate() {
                        let mut buf = [0u8; <$($t)::* as crate::traits::Bytes<crate::Origin>>::BYTES_SIZE];
                        buf.copy_from_slice(chunk);
                        arr[i] = <$($t)::* as crate::traits::Bytes<crate::Origin>>::from_bytes(buf, endianness);
                    }
                    arr

                }
            }
        )+
    };
}
