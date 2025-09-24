// #[macro_export]
// macro_rules! union {
//     (
//         $(#[$($union_doc:meta),*])*
//         $union_visualization:vis union $union_identifier:ident ($($field_identifier:ident : $field_vis:vis $field_type:ty),*)
//     ) => {
//         $(#[$($union_doc),*])*
//         $union_visualization union $union_identifier {
//             (
//                 $($field_vis $field_identifier: $field_type),*
//             )
//         };

//         impl $crate::traits::Bytes<crate::Origin, crate::Origin> for $union_identifier {
//             // Union size is the maximum of its fields
//             const BYTES_SIZE : usize = $crate::expressions_upperbound!($(<$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE),*);
//             // Union alignment is the maximum alignment
//             const BYTES_ALIGN : usize = $crate::expressions_upperbound!($(<$field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_ALIGN),*);

//             fn to_bytes(&self, endianness: bool) -> [u8; <$union_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
//                 let mut b = [0u8; <$union_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
//                 unsafe {
//                     // Take the first field as representative
//                     b[..<$($field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)*].copy_from_slice(
//                         &<$($field_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(&self.$field_identifier, endianness)*)
//                     );
//                 }
//                 b
//             }

//             fn from_bytes(bytes: [u8; <$union_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> $union_identifier {
//                 unsafe {
//                     let mut u = core::mem::MaybeUninit::<$union_identifier>::uninit();
//                     let ptr = u.as_mut_ptr() as *mut u8;
//                     core::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, <$union_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE);
//                     u.assume_init()
//                 }
//             }

//             fn from_bytes_pointer(bytes_pointer: *const u8, _endianness: bool) -> Self {
//                 unsafe {
//                     let mut u = core::mem::MaybeUninit::<$union_identifier>::uninit();
//                     core::ptr::copy_nonoverlapping(bytes_pointer, u.as_mut_ptr() as *mut u8, <$union_identifier as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE);
//                     u.assume_init()
//                 }
//             }
//         }
//     };
// }

// pub use union;
