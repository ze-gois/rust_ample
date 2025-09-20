#[macro_export]
macro_rules! r#struct {
    (
        $(#[$($struct_doc:meta),*])*
        $struct_vis:vis struct $struct_identifier:ident
        $(<
            $($struct_generics:tt),*
        >)?
        $(where
            $($where_alias:ty : $($where_boundary:tt)::* $(<$($($where_boundary_generics:tt)::*),*>)?),*
        )? $(,)?
        {
            $(
                $(#[$($field_doc:meta),*])*
                $field_vis:vis $field_identifier:ident : $field_type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$($struct_doc),*])*
        $struct_vis struct $struct_identifier
        $(<
            $($struct_generics),*
        >)?
        $(where
            $($where_alias : $($where_boundary)::*  ),*
        )?
        {
            $(
                $(#[$($field_doc),*])*
                $field_vis $field_identifier : $field_type
            ),*
        }

        // crate::trait_implement_primitive_pointer_bytes!($struct_identifier $(<$($struct_generics),*>)?);
        // crate::trait_implement_primitive_array_bytes!($struct_identifier $(<$($struct_generics),*>)?);
        // crate::trait_implement_primitive_option_bytes!($struct_identifier $(<$($struct_generics),*>)?);


        impl$(<$($struct_generics),*>)? $crate::traits::Primitive<crate::Origin> for $struct_identifier $(<$($struct_generics),*>)?
        $(where
            $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
        )?{
            const IS_PRIMITIVE : bool = false;
        }

        impl$(<$($struct_generics),*>)? $crate::traits::NonPrimitive<crate::Origin> for $struct_identifier $(<$($struct_generics),*>)?
        $(where
            $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
        )?{
            const IS_PRIMITIVE : bool = false;
        }

        impl$(<$($struct_generics),*>)? $crate::traits::Bytes<crate::Origin, crate::Origin> for $struct_identifier $(<$($struct_generics),*>)?
        $(where
            $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
        )?{
            const BYTES_SIZE : usize = $(<$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE +)* 0;
            const BYTES_ALIGN : usize = core::mem::align_of::<$struct_identifier $(<$($struct_generics),*>)?>();

            fn to_bytes(&self, endianness: bool) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
                let mut b = [0u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                let mut o = 0;
                // let _ = endianness;
                $(
                    b[o..(o+<$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)].copy_from_slice(
                        &if endianness {
                            <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_le_bytes(&self.$field_identifier)
                        } else {
                            <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_be_bytes(&self.$field_identifier)
                        }
                    );
                    o = o + <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                )*
                b
            }

            fn from_bytes(bytes : [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
                let mut _o = 0;
                let _ = bytes;
                let _ = endianness;
                $(
                    let mut field_bytes = [0u8; <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                    field_bytes.copy_from_slice(&bytes[_o..(_o+<$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)]);
                    let $field_identifier = if endianness {
                        <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(field_bytes)
                    } else {
                        <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_be_bytes(field_bytes)
                    };
                    _o = _o + <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                )*
                Self {
                    $($field_identifier,)*
                }
            }

            // fn from_bytes_pointer(bytes: *const u8, endianness: bool) -> Self {
            //     let mut _o = 0;
            //     let _ = bytes;
            //     let _ = endianness;
            //     $(
            //         let mut field_bytes = [0u8; <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
            //         field_bytes.copy_from_slice(&bytes[_o..(_o+<$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)]);
            //         let $field_identifier = if endianness {
            //             <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(field_bytes)
            //         } else {
            //             <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_be_bytes(field_bytes)
            //         };
            //         _o = _o + <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
            //     )*
            //     Self {
            //         $($field_identifier,)*
            //     }
            // }
        }


        impl$(<$($struct_generics),*>)? Clone for $struct_identifier $(<$($struct_generics),*>)?
        $(where
            $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
            [u8; <B as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
        )?
        {
            fn clone(&self) -> Self {
                let bytes = <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_le_bytes(self);
                <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(bytes)
            }
        }

        impl$(<$($struct_generics),*>)? Copy for $struct_identifier $(<$($struct_generics),*>)?
        $(where
            $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
            [u8; <B as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
        )?
        {
        }


        impl$(<$($struct_generics),*>)? Default for $struct_identifier $(<$($struct_generics),*>)?
        $(where
            $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
            [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
        )?
        {
            fn default() -> Self {
                <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes([0u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE])
            }
        }

        // impl$(<$($struct_generics),*>)? Drop for $struct_identifier $(<$($struct_generics),*>)?
        // $(where
        //     $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
        //     [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
        // )?
        // {
        //     fn drop(&mut self) {
        //         <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes([0u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE])
        //     }
        // }


    }
}

pub use r#struct;
