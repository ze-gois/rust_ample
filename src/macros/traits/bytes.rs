#[macro_use]
pub mod implement;

#[macro_use]
pub mod blanket;

#[macro_use]
pub mod naturalize;

#[macro_use]
pub mod option;

#[macro_use]
pub mod phantom;

#[macro_use]
pub mod pointer;

#[macro_use]
pub mod slice;

#[macro_use]
pub mod tuple;

#[macro_use]
pub mod primitive;

#[macro_export]
macro_rules! trait_bytes_place {
    () => {
        pub trait Bytes<Origin, Destination> {
            const BYTES_SIZE: usize;
            const BYTES_ALIGN: usize;

            fn primitive_offset_size(&self) -> usize {
                Self::BYTES_SIZE
            }

            fn to_bytes(
                &self,
                endianness: bool,
            ) -> [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE]
            where
                [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE]:;

            fn to_le_bytes(
                &self,
            ) -> [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE]
            where
                [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE]:,
            {
                self.to_bytes(true)
            }

            fn to_be_bytes(
                &self,
            ) -> [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE]
            where
                [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE]:,
            {
                self.to_bytes(false)
            }

            fn from_bytes(
                bytes: [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE],
                endianness: bool,
            ) -> Self
            where
                Self: Sized,
                [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE]:;

            fn from_bytes_pointer(pointer: *const u8, endianness: bool) -> Self
            where
                Self: Sized;

            fn from_le_bytes(
                bytes: [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE],
            ) -> Self
            where
                Self: Sized,
                [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE]:,
            {
                Self::from_bytes(bytes, true)
            }

            fn from_be_bytes(
                bytes: [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE],
            ) -> Self
            where
                Self: Sized,
                [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE]:,
            {
                Self::from_bytes(bytes, false)
            }
        }
    };
}
