#[macro_export]
macro_rules! trait_implement_primitive_array_bytes {
    ($($($t:tt)::* $(<$($type_generics:tt),*>)? ),*) => {
        $(
            impl <const N: usize> $crate::traits::Bytes<crate::Origin, crate::Origin> for [$($t)::*; N]
            {
                const BYTES_SIZE: usize =
                    N * <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                const BYTES_ALIGN: usize = <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_ALIGN;

                fn to_bytes(
                    &self,
                    endianness: bool,
                ) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]
                where
                    [(); <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
                {
                    let mut bytes = [0u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                    let item_size = <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;

                    for (i, item) in self.iter().enumerate() {
                        let item_bytes = <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(
                            item,
                            endianness
                        );
                        let start = i * item_size;
                        bytes[start..start + item_size].copy_from_slice(&item_bytes);
                    }
                    bytes
                }

                fn from_bytes(bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self
                where
                    [(); <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
                {
                    const NN: usize = <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                    let defaulta = <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes([0u8; NN], endianness);
                    let mut arr: [$($t)::*; N] = [defaulta; N];
                    for (i, chunk) in bytes.chunks_exact(<$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE).enumerate() {
                        let mut buf = [0u8; <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                        buf.copy_from_slice(chunk);
                        arr[i] = <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(buf, endianness);
                    }
                    arr

                }


                fn from_bytes_pointer(bytes_pointer: *const u8, endianness: bool) -> Self {
                    const NN: usize = <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                    let defaulta = <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes([0u8; NN], endianness);
                    let mut arr: [$($t)::*; N] = [defaulta; N];
                    for i in 0..N {
                        let mut buf = [0u8; <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                        unsafe { core::ptr::copy_nonoverlapping(bytes_pointer.add(i*NN), buf.as_mut_ptr(), NN) };
                        // buf.copy_from_slice(unsafe { std::slice::from_raw_parts(bytes_pointer.add(i * NN), NN) });
                        arr[i] = <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(buf, endianness);
                    }
                    // for (i, chunk) in bytes.chunks_exact(<$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE).enumerate() {
                    //     let mut buf = [0u8; <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                    //     buf.copy_from_slice(chunk);
                    //     arr[i] = <$($t)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(buf, endianness);
                    // }
                    arr

                    // let mut _o = 0;
                    // $(
                    //     let mut field_bytes = [0u8; <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                    //     unsafe {
                    //         core::ptr::copy_nonoverlapping(bytes_pointer.add(_o), field_bytes.as_mut_ptr(), <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE);
                    //     }
                    //     let $field_identifier = <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(field_bytes,endianness);
                    //     _o = _o + <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                    // )*
                    // Self {
                    //     $($field_identifier,)*
                    // }
                }
            }
        )+
    };
}
