#[macro_export]
macro_rules! r#generic_struct {
    ($(#[$($struct_doc:meta),*])* $vis:vis struct $struct_identifier:ident $(where $($where_clause:tt),*)? { $($(#[$field_doc:meta])* $field_visibility:vis $field_identifier:ident: $field_type:ty),*$(,)?}) => {
        // #[repr(C,packed)]
        // #[derive(Debug, Clone, Copy)]
        #[derive(Debug)]
        $(#[$($struct_doc),*])*
        $vis struct $struct_identifier {
            $(
                $(#[$field_doc])*
                $field_visibility $field_identifier: $field_type
            ),*
        }

        impl $crate::traits::Bytes<crate::Origin, crate::Origin> for $struct_identifier {
            const BYTES_SIZE : usize = $(<$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE +)* 0;

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
        }

        // type u8 = u8;
        impl $crate::traits::Bytes<crate::Origin, crate::Origin> for Option<$struct_identifier> {
            const BYTES_SIZE: usize = <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE + <$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;

            fn from_bytes(bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
                let mut option_bytes = [0u8; <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                let mut o = 0;
                let mut l = <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                option_bytes.copy_from_slice(&bytes[o..l]);
                let option = if endianness {
                    <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(option_bytes)
                } else {
                    <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_be_bytes(option_bytes)
                };
                if option == 0 {
                    None
                } else {
                    o = l;
                    l = l + <$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                    let mut value_bytes = [0u8; <$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                    value_bytes.copy_from_slice(&bytes[o..l]);
                    if endianness {
                        Some(<$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(value_bytes))
                    } else {
                        Some(<$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_be_bytes(value_bytes))
                    }
                }
            }

            fn to_bytes(&self, endianness: bool) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
                let mut bytes = [0u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                if let Some(v) = self {
                    let mut o = 0;
                    let mut l = <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                    bytes[o..l].copy_from_slice(&(1 as u8).to_le_bytes());
                    o = l;
                    l = l + <$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                    if endianness {
                        bytes[o..l].copy_from_slice(&<$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_le_bytes(v));
                    } else {
                        bytes[o..l].copy_from_slice(&<$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_be_bytes(v));
                    }
                    bytes
                } else {
                    bytes
                }
            }
        }

        impl Clone for $struct_identifier
        where
            $struct_identifier: $crate::traits::Bytes<crate::Origin, crate::Origin>,
            [u8; <$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
        {
            fn clone(&self) -> $struct_identifier {
                let bytes = <$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_le_bytes(&self);
                <$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(bytes)
            }
        }

        impl Copy for $struct_identifier
        where
            $struct_identifier: $crate::traits::Bytes<crate::Origin, crate::Origin>,
            [u8; <$struct_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
        {
        }

    }
}
pub use r#struct;
