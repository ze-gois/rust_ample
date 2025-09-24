#[macro_export]
macro_rules! r#enum {
    (
        $enum_discriminant_type:ty;
        $(#[$($struct_doc:meta),*])*
        $enum_vis:vis enum $enum_identifier:ident {
            $(
                $variant_identifier:ident = $variant_discriminant:tt
            ),* $(,)?
        }
    ) => {

        // pub mod constants {
        //     $(pub const $variant_identifier: $enum_discriminant_type = $variant_discriminant as $enum_discriminant_type;)*
        // }

        $(#[$($struct_doc),*])*
        #[repr(u64)]
        $enum_vis enum $enum_identifier {
            $(
                $variant_identifier = $variant_discriminant
            ),*
        }

        impl $enum_identifier {
            pub fn discriminant(&self) -> $enum_discriminant_type {
                match self {
                    $(
                        $enum_identifier::$variant_identifier => $variant_discriminant
                    ),*
                }
            }

            pub fn from_discriminant(discriminant: $enum_discriminant_type) -> Option<Self> {
                match discriminant {
                    $(
                        $variant_discriminant => Some($enum_identifier::$variant_identifier),
                    )*
                    _ => None,
                }
            }
        }

        impl $crate::traits::Bytes<crate::Origin, crate::Origin> for $enum_identifier {
            const BYTES_SIZE : usize = <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
            const BYTES_ALIGN : usize = <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_ALIGN;

            fn primitive_load_size(&self) -> usize {
                <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE
            }

            fn to_bytes(&self, endianness: bool) -> [u8;<Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
                let mut bytes = [0u8;<Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];

                match self {
                    $(
                        Self::$variant_identifier => {
                            let discriminant = self.discriminant();

                            let mut o = 0;
                            bytes[o..(o+<$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)].copy_from_slice(
                                &<$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(&discriminant,endianness)
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
                            Self::$variant_identifier
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
                            Self::$variant_identifier
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
pub use r#enum;
