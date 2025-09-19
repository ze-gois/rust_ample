#[macro_export]
macro_rules! trait_bytes_naturalize {
    ($($donnor:tt)::*, $($receiver:tt)::*) => {
        impl<A> $($receiver)::*::traits::Bytes<$($receiver)::*::Origin> for A
        where
            A: $($donnor)::*::traits::Bytes<$($donnor)::*::Origin>,
            [u8; <A as $($donnor)::*::traits::Bytes<$($donnor)::*::Origin>>::BYTES_SIZE]:,
        {
            const BYTES_SIZE: usize =
                <A as $($donnor)::*::traits::Bytes<$($donnor)::*::Origin>>::BYTES_SIZE;

            const BYTES_ALIGN: usize =
                <A as $($donnor)::*::traits::Bytes<$($donnor)::*::Origin>>::BYTES_ALIGN;

            fn to_bytes(
                &self,
                endianness: bool,
            ) -> [u8; <Self as $($receiver)::*::traits::Bytes<$($receiver)::*::Origin>>::BYTES_SIZE] {
                <A as $($donnor)::*::traits::Bytes<$($donnor)::*::Origin>>::to_bytes(self, endianness)
            }

            fn from_bytes(
                bytes: [u8; <Self as $($receiver)::*::traits::Bytes<$($receiver)::*::Origin>>::BYTES_SIZE],
                endianness: bool,
            ) -> Self {
                <A as $($donnor)::*::traits::Bytes<$($donnor)::*::Origin>>::from_bytes(bytes, endianness)
            }
        }
    };
}
