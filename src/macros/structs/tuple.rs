#[macro_export]
macro_rules! struct_tuple {
    (
        $(#[$($tuple_doc:meta),*])*
        $struct_visualization:vis struct $struct_identifier:ident ($($ordinal_no:tt : $ordinal_visibility:vis $ordinal_type:ty),*)
    ) => {
        $(#[$($tuple_doc),*])*
        $struct_visualization struct $struct_identifier($($ordinal_visibility $ordinal_type),*);

        impl $crate::traits::Bytes<crate::Origin, crate::Origin> for $struct_identifier {
            const BYTES_SIZE : usize = $(<$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE + )* 0;
            const BYTES_ALIGN : usize = $crate::expressions_upperbound!($(<$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_ALIGN ),*);


            fn to_bytes(&self, endianness: bool) -> [u8; <$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
                let mut b = [0u8; <$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                let mut o = 0;
                $(
                    b[o..(o+<$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)].copy_from_slice(
                        &<$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(&self.$ordinal_no,endianness)
                    );
                    o = o + <$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                )*
                b
            }

            fn from_bytes(bytes : [u8; <$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> $struct_identifier {
                let mut o = 0;
                $struct_identifier (
                    $(
                        {
                            let mut field_bytes = [0u8; <$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                            field_bytes.copy_from_slice(&bytes[o..(o+<$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)]);
                            let ordinal = if endianness {
                                <$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(field_bytes)
                            } else {
                                <$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_be_bytes(field_bytes)
                            };
                            o = o + <$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                            ordinal
                        }
                    ),*
                )
            }

            fn from_bytes_pointer(bytes_pointer: *const u8, endianness: bool) -> Self {
                let mut _o = 0;
                $struct_identifier (
                    $(
                        {
                            let mut field_bytes = [0u8; <$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                            unsafe {
                                core::ptr::copy_nonoverlapping(bytes_pointer.add(_o), field_bytes.as_mut_ptr(), <$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE);
                            }
                            let ordinal = <$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(field_bytes, endianness);
                            _o = _o + <$ordinal_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                            ordinal
                        }
                    ),*
                )
            }
        }

        $crate::trait_implement_bytes_option!(
            $struct_identifier
        );


        $crate::trait_implement_bytes_slice!(
            $struct_identifier
        );

        impl $crate::traits::BytesDefault<crate::Origin> for $struct_identifier {}

        impl Clone for $struct_identifier

        {
            fn clone(&self) -> Self {
                let bytes = <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_le_bytes(self);
                <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(bytes)
            }
        }

        impl Copy for $struct_identifier
        {
        }


        impl Default for $struct_identifier
        {
            fn default() -> Self {
                <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes([0u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE])
            }
        }


        // impl $crate::traits::BytesDefault<crate::Origin> for $struct_identifier
        // $(where
        //     $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
        // )?
        // {
        // }
    };
}

pub use struct_tuple;
