/// Define a lossless set of flags over an integer representation.
///
/// The generated type preserves every raw bit, including bits unknown to this
/// version of the crate. Descriptive Rust constants name known values while the
/// generated raw constants retain the spelling supplied by the external
/// standard.
#[macro_export]
#[rustfmt::skip]
macro_rules! flags {
    (
        $bits_type:ty;
        $label:expr;
        $(#[$($meta:meta),*])*
        $vis:vis struct $identifier:ident {
            $(
                [
                    $bits:expr;
                    $name:ident;
                    $standard_name:ident;
                    $acronym:expr;
                    $description:expr
                ]
            ),* $(,)?
        }
    ) => {
        pub mod constants {
            $(
                pub const $standard_name: $bits_type = $bits;
            )*
        }

        pub use constants::*;

        $(#[$($meta),*])*
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        $vis struct $identifier($bits_type);

        impl $identifier {
            $(
                pub const $name: Self = Self($bits);
            )*

            pub const KNOWN_BITS: $bits_type = 0 $(| $bits)*;

            pub const fn from_bits(bits: $bits_type) -> Self {
                Self(bits)
            }

            pub const fn bits(self) -> $bits_type {
                self.0
            }

            pub const fn known_bits(self) -> $bits_type {
                self.0 & Self::KNOWN_BITS
            }

            pub const fn unknown_bits(self) -> $bits_type {
                self.0 & !Self::KNOWN_BITS
            }

            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }

            pub const fn contains(self, other: Self) -> bool {
                if other.0 == 0 {
                    self.0 == 0
                } else {
                    (self.0 & other.0) == other.0
                }
            }

            pub fn acronym(self) -> &'static str {
                match self.0 {
                    $(
                        $bits => $acronym,
                    )*
                    _ => "unknown",
                }
            }

            pub fn description(self) -> &'static str {
                match self.0 {
                    $(
                        $bits => $description,
                    )*
                    _ => $label,
                }
            }
        }

        impl Default for $identifier {
            fn default() -> Self {
                Self(0)
            }
        }

        impl From<$bits_type> for $identifier {
            fn from(bits: $bits_type) -> Self {
                Self::from_bits(bits)
            }
        }

        impl From<$identifier> for $bits_type {
            fn from(flags: $identifier) -> Self {
                flags.bits()
            }
        }

        impl core::ops::BitOr for $identifier {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl core::ops::BitOrAssign for $identifier {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl core::ops::BitAnd for $identifier {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl core::ops::BitAndAssign for $identifier {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl core::ops::BitXor for $identifier {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl core::ops::Not for $identifier {
            type Output = Self;

            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl core::fmt::Display for $identifier {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "{}({:#x})", $label, self.0)
            }
        }

        impl core::fmt::Debug for $identifier {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "{}({:#x}, {})", $label, self.0, self.acronym())
            }
        }

        impl $crate::traits::Bytes<crate::Origin, crate::Origin> for $identifier {
            const REPRESENTATION_SIZE: usize =
                <$bits_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::REPRESENTATION_SIZE;

            fn to_bytes(
                &self,
                endianness: bool,
            ) -> [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::REPRESENTATION_SIZE] {
                <$bits_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(
                    &self.0,
                    endianness,
                )
            }

            fn from_bytes(
                bytes: [u8; <Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::REPRESENTATION_SIZE],
                endianness: bool,
            ) -> Self {
                Self(
                    <$bits_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(
                        bytes,
                        endianness,
                    ),
                )
            }

            fn from_bytes_pointer(bytes_pointer: *const u8, endianness: bool) -> Self {
                Self(
                    <$bits_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes_pointer(
                        bytes_pointer,
                        endianness,
                    ),
                )
            }
        }

        impl $crate::traits::BytesDefault<crate::Origin> for $identifier {}
    };
}

pub use flags;
