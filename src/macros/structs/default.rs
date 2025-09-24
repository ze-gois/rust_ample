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

        $crate::trait_implement_bytes_option!(
            $struct_identifier $(<$($struct_generics),*>)?
            $(where
                $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
            )?
        );


        $crate::trait_implement_bytes_slice!(
            $struct_identifier $(<$($struct_generics),*>)?
            $(where
                $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
            )?
        );

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

            fn primitive_load_size(&self) -> usize {
                $(<$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::primitive_load_size(&self.$field_identifier) +)* 0
            }

            fn to_bytes(&self, endianness: bool) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
                let mut b = [0u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                let mut o = 0;
                // let _ = endianness;
                $(
                    b[o..(o+<$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)].copy_from_slice(
                        &<$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(&self.$field_identifier,endianness)
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
                    let $field_identifier = <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(field_bytes, endianness);
                    _o = _o + <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                )*
                Self {
                    $($field_identifier,)*
                }
            }

            fn from_bytes_pointer(bytes_pointer: *const u8, endianness: bool) -> Self {
                let mut _o = 0;
                $(
                    let mut field_bytes = [0u8; <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                    unsafe {
                        core::ptr::copy_nonoverlapping(bytes_pointer.add(_o), field_bytes.as_mut_ptr(), <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE);
                    }
                    let $field_identifier = <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(field_bytes,endianness);
                    _o = _o + <$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                )*
                Self {
                    $($field_identifier,)*
                }
            }
        }

        impl$(<$($struct_generics),*>)? $crate::traits::BytesDefault<crate::Origin> for $struct_identifier $(<$($struct_generics),*>)?
        $(where
            $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
        )?
        {
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
