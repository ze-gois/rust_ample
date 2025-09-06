// #[macro_export]
// macro_rules! traits_implement_tuple_bytes {
//     ($($($ordinal_type:tt)::*),*) => {
//         impl $crate::traits::Bytes<crate::Origin, crate::Origin> for ($($($ordinal_type)::*),*) {
//             const BYTES_SIZE: usize = $(<$($ordinal_type)::* as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE + )* 0;
//             fn to_bytes(
//                 &self,
//                 endianness: bool,
//             ) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]
//             where
//                 [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
//             {
//                 let mut pair_bytes =
//                     [0u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];

//                 if endianness {
//                     let mut o = 0;
//                     let mut l = <Pcu as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
//                     pair_bytes[o..l].copy_from_slice(&self.0.to_le_bytes());
//                     o = l;
//                     l = l + <Pci8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
//                     pair_bytes[o..l].copy_from_slice(&self.1.to_le_bytes());
//                 } else {
//                     let mut o = 0;
//                     let mut l = <Pcu as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
//                     pair_bytes[o..l].copy_from_slice(&self.0.to_be_bytes());
//                     o = l;
//                     l = l + <Pci8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
//                     pair_bytes[o..l].copy_from_slice(&self.1.to_be_bytes());
//                 }

//                 pair_bytes
//             }

//             fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self
//             where
//                 [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
//             {
//                 let mut left_bytes =
//                     [0u8; <Pcu as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
//                 let mut right_bytes =
//                     [0u8; <Pci8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];

//                 let mut o = 0;
//                 let mut l = <Pcu as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;

//                 left_bytes.copy_from_slice(&bytes[o..l]);

//                 o = l;
//                 l = l + <Pci8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;

//                 right_bytes.copy_from_slice(&bytes[o..l]);

//                 if endianness {
//                     let left = <Pcu as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(
//                         left_bytes,
//                     );
//                     let right =
//                         <Pci8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(
//                             right_bytes,
//                         );
//                     (left, right)
//                 } else {
//                     let left = <Pcu as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_be_bytes(
//                         left_bytes,
//                     );
//                     let right =
//                         <Pci8 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_be_bytes(
//                             right_bytes,
//                         );
//                     (left, right)
//                 }
//             }
//         }
//     };
// }
// // pub use trait_implement_primitive_bytes;
// pub use traits_implement_bytes_tuple;
