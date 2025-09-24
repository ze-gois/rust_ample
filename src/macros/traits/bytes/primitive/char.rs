#[macro_export]
macro_rules! trait_implement_primitive_char_bytes {
    () => {
        impl $crate::traits::Bytes<crate::Origin, crate::Origin> for char {
            const BYTES_SIZE: usize = 4;
            const BYTES_ALIGN: usize = 4;

            fn to_bytes(
                &self,
                endianness: bool,
            ) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]
            {
                <u32 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(&(*self as u32),endianness)
            }

            fn from_bytes(
                bytes: [u8;
                    <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE],
                endianness: bool,
            ) -> Self {
                let val = <u32 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(bytes, endianness);
                core::char::from_u32(val).unwrap()
            }

            fn from_bytes_pointer(bytes_pointer: *const u8, endianness: bool) -> Self {
                let o = 0;
                let mut char_bytes = [0u8; <u32 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                unsafe { core::ptr::copy_nonoverlapping(bytes_pointer.add(o), char_bytes.as_mut_ptr(), <u32 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE) };
                let val = <u32 as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(char_bytes, endianness);
                core::char::from_u32(val).unwrap()
            }
        }
    };
}
