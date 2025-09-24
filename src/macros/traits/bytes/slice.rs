#[macro_export]
macro_rules! trait_implement_bytes_slice {
    (
        $(
            $($type:tt)::* $(<$($type_generics:tt),*>)?
            $(where
                $($where_alias:ty : $($where_boundary:tt)::* $(<$($($where_boundary_generics:tt)::*),*>)?),*
            )?
        ),*
    ) => {
        $(
            impl <const N: usize> $crate::traits::Bytes<crate::Origin, crate::Origin> for [$($type)::*; N]
            $(where
                $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
            )?
            {
                const BYTES_SIZE: usize =
                    N * <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                    const BYTES_ALIGN: usize = <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_ALIGN;

                fn to_bytes(
                    &self,
                    endianness: bool,
                ) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]
                where
                    [(); <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
                {
                    let mut bytes = [0u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                    let item_size = <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;

                    for (i, item) in self.iter().enumerate() {
                        let item_bytes = <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(
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
                    const NN: usize = <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                    let defaulta = <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes([0u8; NN], endianness);
                    let mut arr: [$($type)::*; N] = [defaulta; N];
                    for (i, chunk) in bytes.chunks_exact(<$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE).enumerate() {
                        let mut buf = [0u8; <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                        buf.copy_from_slice(chunk);
                        arr[i] = <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(buf, endianness);
                    }
                    arr

                }


                fn from_bytes_pointer(bytes_pointer: *const u8, endianness: bool) -> Self {
                    const NN: usize = <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                    let defaulta = <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes([0u8; NN], endianness);
                    let mut arr: [$($type)::*; N] = [defaulta; N];
                    for i in 0..N {
                        let mut buf = [0u8; <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                        unsafe { core::ptr::copy_nonoverlapping(bytes_pointer.add(i*NN), buf.as_mut_ptr(), NN) };
                        // buf.copy_from_slice(unsafe { std::slice::from_raw_parts(bytes_pointer.add(i * NN), NN) });
                        arr[i] = <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(buf, endianness);
                    }
                    // for (i, chunk) in bytes.chunks_exact(<$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE).enumerate() {
                    //     let mut buf = [0u8; <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                    //     buf.copy_from_slice(chunk);
                    //     arr[i] = <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(buf, endianness);
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

            impl <const N: usize> $crate::traits::BytesDefault<crate::Origin> for [$($type)::*; N]
            $(where
                $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
            )?
            {
            }
            // impl$(<$($struct_generics),*>)?  for $struct_identifier $(<$($struct_generics),*>)?
            // $(where
            //     $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
            // )?
            // {
            // }
        )+
    };
}
