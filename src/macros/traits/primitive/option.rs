#[macro_export]
macro_rules! trait_implement_primitive_option_bytes {
    ($($($t:tt)::*),*) => {
        $(
            impl $crate::traits::Bytes<crate::Origin,crate::Origin> for Option<$($t)::*> {
                const BYTES_SIZE: usize = core::mem::size_of::<u8>() + <$($t)::* as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;

                fn from_bytes(bytes: [u8; <Option<$($t)::*> as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
                    let mut option_bytes = [0u8; <u8 as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE];
                    let mut o = 0;
                    let mut l = <u8 as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                    option_bytes.copy_from_slice(&bytes[o..l]);
                    let option = if endianness {
                        <u8 as $crate::traits::Bytes<crate::Origin,crate::Origin>>::from_le_bytes(option_bytes)
                    } else {
                        <u8 as $crate::traits::Bytes<crate::Origin,crate::Origin>>::from_be_bytes(option_bytes)
                    };
                    if option == 0 {
                        None
                    } else {
                        o = l;
                        l = l + <$($t)::* as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                        let mut value_bytes = [0u8; <$($t)::* as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE];
                        value_bytes.copy_from_slice(&bytes[o..l]);
                        if endianness {
                            Some(<$($t)::* as $crate::traits::Bytes<crate::Origin,crate::Origin>>::from_le_bytes(value_bytes))
                        } else {
                            Some(<$($t)::* as $crate::traits::Bytes<crate::Origin,crate::Origin>>::from_be_bytes(value_bytes))
                        }
                    }
                }

                fn to_bytes(&self, endianness: bool) -> [u8; <Option<$($t)::*> as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE] {
                    let mut bytes = [0u8; <Option<$($t)::*> as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE];
                    if let Some(v) = self {
                        let mut o = 0;
                        let mut l = <u8 as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                        bytes[o..l].copy_from_slice(&1u8.to_le_bytes());
                        o = l;
                        l = l + <$($t)::* as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                        if endianness {
                            bytes[o..l].copy_from_slice(&<$($t)::* as $crate::traits::Bytes<crate::Origin,crate::Origin>>::to_le_bytes(v));
                        } else {
                            bytes[o..l].copy_from_slice(&<$($t)::* as $crate::traits::Bytes<crate::Origin,crate::Origin>>::to_be_bytes(v));
                        }
                        bytes
                    } else {
                        bytes
                    }
                }
            }
        )*
    };
}
