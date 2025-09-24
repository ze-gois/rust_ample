#[macro_export]
macro_rules! r#enum_typed {
    (
        $enum_discriminant_type:ty;
        $(#[$($struct_doc:meta),*])*
        $enum_vis:vis enum $enum_identifier:ident {
            $(
                $variant_identifier:ident($($variant_type:tt)::*) = $variant_discriminant:expr
            ),* $(,)?
        }
    ) => {

        $(#[$($struct_doc),*])*
        #[repr(u64)]
        $enum_vis enum $enum_identifier {
            $(
                $variant_identifier($($variant_type)::*) = $variant_discriminant
            ),*
        }

        impl $enum_identifier {
            pub fn discriminant(&self) -> $enum_discriminant_type {
                match self {
                    $(
                        $enum_identifier::$variant_identifier(_) => $variant_discriminant
                    ),*
                }
            }
        }

        impl $crate::traits::Bytes<crate::Origin, crate::Origin> for $enum_identifier {
            const BYTES_SIZE : usize = <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE + $crate::expressions_upperbound!($(<$($variant_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE),*);
            const BYTES_ALIGN : usize = $crate::expressions_upperbound!($(<$($variant_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_ALIGN),*);

            fn primitive_load_size(&self) -> usize {
                match self {
                    $(
                        Self::$variant_identifier(variant_value) => {
                            <$($variant_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::primitive_load_size(&variant_value)
                        }
                    ),*
                }
            }

            fn to_bytes(&self, endianness: bool) -> [u8;<Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
                let mut bytes = [0u8;<Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];

                match self {
                    $(
                        Self::$variant_identifier(payload) => {
                            let discriminant = self.discriminant();

                            let mut o = 0;
                            bytes[o..(o+<$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)].copy_from_slice(
                                &<$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(&discriminant,endianness)
                            );
                            o = o + <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                            bytes[o..(o+<$($variant_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)].copy_from_slice(
                                &<$($variant_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(payload,endianness)
                            );
                            bytes
                        }
                    ),*
                }
            }

            fn from_bytes(bytes: [u8;<Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
                let mut o = 0;
                let mut discriminant_bytes = [0u8; <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                discriminant_bytes.copy_from_slice(&bytes[o..(o+<$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)]);
                let discriminant = <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(discriminant_bytes, endianness);
                o = o + <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                match discriminant {
                    $(
                        $variant_discriminant => {
                            Self::$variant_identifier({
                                let mut payload = [0u8; <$($variant_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                                payload.copy_from_slice(&bytes[o..(o+<$($variant_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)]);
                                <$($variant_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(payload,endianness)
                            })
                        },
                    )*
                    _ => unreachable!()
                }
            }

            fn from_bytes_pointer(bytes_pointer: *const u8, endianness: bool) -> Self {
                let mut o = 0;
                let mut discriminant_bytes = [0u8; <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                unsafe { core::ptr::copy_nonoverlapping(bytes_pointer.add(o), discriminant_bytes.as_mut_ptr(), <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE) };
                let discriminant = <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(discriminant_bytes, endianness);
                o = o + <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                match discriminant {
                    $(
                        $variant_discriminant => {
                            Self::$variant_identifier({
                                let mut payload_bytes = [0u8; <$($variant_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                                unsafe { core::ptr::copy_nonoverlapping(bytes_pointer.add(o), payload_bytes.as_mut_ptr(), <$($variant_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE) };
                                <$($variant_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(payload_bytes, endianness)
                            })
                        },
                    )*
                    _ => unreachable!()
                }
            }
        }

        $crate::trait_implement_bytes_option!(
            $enum_identifier
        );


        $crate::trait_implement_bytes_slice!(
            $enum_identifier
        );

        impl Clone for $enum_identifier {
            fn clone(&self) -> Self {
                let bytes = <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_le_bytes(self);
                <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(bytes)
            }
        }

        impl Copy for $enum_identifier { }

        impl $crate::traits::BytesDefault<crate::Origin> for $enum_identifier {}

        // impl$(<$($struct_generics),*>)? $crate::traits::BytesDefault<crate::Origin> for $struct_identifier $(<$($struct_generics),*>)?
        // $(where
        //     $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
        // )?
        // {
        // }

    };
}
pub use r#enum_typed;
