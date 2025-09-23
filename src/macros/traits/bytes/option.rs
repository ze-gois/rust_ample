#[macro_export]
macro_rules! trait_implement_bytes_option {
    (
        $(
            $($type:tt)::* $(<$($type_generics:tt),*>)?
            $(where
                $($where_alias:ty : $($where_boundary:tt)::* $(<$($($where_boundary_generics:tt)::*),*>)?),*
            )?
        ),*
    ) => {
        $(
            impl $crate::traits::Bytes<crate::Origin, crate::Origin> for Option<$($type)::*>
            $(where
                $($where_alias : $($where_boundary)::* $(<$($($where_boundary_generics)::*),*>)?),*
            )?
            {
                const BYTES_SIZE: usize = core::mem::size_of::<u8>() + <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                const BYTES_ALIGN: usize = <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_ALIGN;

                fn from_bytes(bytes: [u8; <Option<$($type)::*> as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
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
                        l = l + <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                        let mut value_bytes = [0u8; <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                        value_bytes.copy_from_slice(&bytes[o..l]);
                        if endianness {
                            Some(<$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(value_bytes))
                        } else {
                            Some(<$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_be_bytes(value_bytes))
                        }
                    }
                }

                fn from_bytes_pointer(bytes_pointer: *const u8, endianness: bool) -> Self {
                    let mut option_bytes = [0u8; <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                    let mut o = 0;
                    let mut l = <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                    unsafe { core::ptr::copy_nonoverlapping(bytes_pointer.add(o), option_bytes.as_mut_ptr(), <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE) };
                    let option = <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(option_bytes, endianness);

                    if option == 0 {
                        None
                    } else {
                        o = l;
                        l = l + <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                        let mut value_bytes = [0u8; <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                        unsafe {core::ptr::copy_nonoverlapping(bytes_pointer.add(o), value_bytes.as_mut_ptr(), <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE) };
                        Some(<$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(value_bytes, endianness))
                    }
                }

                fn to_bytes(&self, endianness: bool) -> [u8; <Option<$($type)::*> as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
                    let mut bytes = [0u8; <Option<$($type)::*> as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                    if let Some(v) = self {
                        let mut o = 0;
                        let mut l = <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                        bytes[o..l].copy_from_slice(&1u8.to_le_bytes());
                        o = l;
                        l = l + <$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
                        if endianness {
                            bytes[o..l].copy_from_slice(&<$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_le_bytes(v));
                        } else {
                            bytes[o..l].copy_from_slice(&<$($type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_be_bytes(v));
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

// #[macro_export]
// macro_rules! trait_implement_bytes_option {
//     () => {
//         impl<B> $crate::traits::Bytes<crate::Origin, crate::Origin> for Option<B>
//         where
//             B: $crate::traits::Bytes<crate::Origin, crate::Origin>,
//             // [();B::BYTES_SIZE]:
//         {
//             const BYTES_SIZE: usize = core::mem::size_of::<u8>() + B::BYTES_SIZE;
//             const BYTES_ALIGN: usize = B::BYTES_ALIGN;

//             fn from_bytes(bytes: [u8; <Option<B> as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
//                 let mut option_bytes = [0u8; <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
//                 let mut o = 0;
//                 let mut l = <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
//                 option_bytes.copy_from_slice(&bytes[o..l]);
//                 let option = if endianness {
//                     <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(option_bytes)
//                 } else {
//                     <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_be_bytes(option_bytes)
//                 };
//                 if option == 0 {
//                     None
//                 } else {
//                     o = l;
//                     l = l + B::BYTES_SIZE;
//                     let mut value_bytes = [0u8; B::BYTES_SIZE];
//                     value_bytes.copy_from_slice(&bytes[o..l]);
//                     Some(B::from_bytes(value_bytes,endianness))
//                 }
//             }

//             fn from_bytes_pointer(bytes_pointer: *const u8, endianness: bool) -> Self {
//                 let mut option_bytes = [0u8; <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
//                 let mut o = 0;
//                 let mut l = <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
//                 unsafe { core::ptr::copy_nonoverlapping(bytes_pointer.add(o), option_bytes.as_mut_ptr(), <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE) };
//                 let option = <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(option_bytes, endianness);

//                 if option == 0 {
//                     None
//                 } else {
//                     o = l;
//                     l = l + B::BYTES_SIZE;
//                     let mut value_bytes = [0u8; B::BYTES_SIZE];
//                     unsafe {core::ptr::copy_nonoverlapping(bytes_pointer.add(o), value_bytes.as_mut_ptr(), B::BYTES_SIZE) };
//                     Some(B::from_bytes(value_bytes, endianness))
//                 }
//             }

//             fn to_bytes(&self, endianness: bool) -> [u8; <Option<B> as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
//                 let mut bytes = [0u8; <Option<B> as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
//                 if let Some(v) = self {
//                     let mut o = 0;
//                     let mut l = <u8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
//                     bytes[o..l].copy_from_slice(&1u8.to_le_bytes());
//                     o = l;
//                     l = l + B::BYTES_SIZE;
//                     bytes[o..l].copy_from_slice(&B::to_bytes(v,endianness));
//                     bytes
//                 } else {
//                     bytes
//                 }
//             }
//         }
//     };
// }
