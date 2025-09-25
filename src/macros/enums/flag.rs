/// A macro that defines an error type and error handling for syscalls.
///
/// This macro generates:
/// 1. An Error enum with the specified variants and their associated discriminant values
/// 2. ErrorTrait implementation for the Error type with proper discriminant mapping
/// 3. An discriminant module with standard Linux error constants
/// 4. Into<isize> implementation for the Error type
/// 5. A handle_result function that maps arch errors to syscall errors
///
/// # Arguments
///
/// * `$enum_identifier` - The name of the error enum (usually Error)
/// * `$enum_discriminant_type` - The variant name in the crate::result::Error enum (e.g., Open, Read, Write)
/// * `$enum_label` - String slice with the syscall name
/// * A list of error variants with their descriptions, discriminant values and Linux standard constant names
///   [VariantName, discriminant_value, "description", "LINUX_CONSTANT"]
///
/// # Example
///
/// ```
/// $enum_identifier:ident,
/// $enum_discriminant_type:ty,
/// $enum_label:expr,
/// [
///     $(
///         [
///             $variant_discriminant:expr;
///             $variant_identifier:ident;
///             $variant_constant_identifier:ident;
///             $variant_acronym:expr;
///             $variant_description:expr
///         ]
///     ),* $(,)?
/// ]
/// ```
#[macro_export]
#[rustfmt::skip]
macro_rules! enum_flag {
    (
        $enum_discriminant_type:ty;
        $enum_label:expr;
        $(#[$($enum_meta:meta),*])*
        $enum_vis:vis enum $enum_identifier:ident {
            $(
                [
                    $variant_discriminant:expr;
                    $variant_identifier:ident;
                    $variant_constant_identifier:ident;
                    $variant_acronym:expr;
                    $variant_description:expr
                ]
            ),* $(,)?
        }
    ) => {
        // Define Linux standard error constants in an discriminant module with standard names
        pub mod constants {
            $(
                pub const $variant_constant_identifier: $enum_discriminant_type = $variant_discriminant;
            )*
        }


        // pub enum $enum_identifier {
        //     $($variant_identifier = $variant_discriminant,)*
        //     TODO,
        // }

        // $crate::r#enum!(
        //     $enum_discriminant_type;
        //     $(#[$($enum_meta),*])*
        //     #[repr(C)]
        //     #[derive(Copy, Clone, Eq, PartialEq)]
        //     $enum_vis enum $enum_identifier {
        //         $(
        //             $variant_identifier = $variant_discriminant,
        //         )*
        //         TODO = <$enum_discriminant_type>::MAX,
        //     }
        // );


        $(#[$($enum_meta),*])*
        #[repr(C)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        $enum_vis enum $enum_identifier {
            $(
                $variant_identifier,
            )*
            TODO,
        }

        impl $enum_identifier {
            pub fn discriminant(&self) -> $enum_discriminant_type {
                match self {
                    $(
                        $enum_identifier::$variant_identifier => $variant_discriminant,
                    )*
                    $enum_identifier::TODO => <$enum_discriminant_type>::MAX,
                }
            }
        }

        impl $crate::traits::Bytes<crate::Origin, crate::Origin> for $enum_identifier {
            const BYTES_SIZE : usize = <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
            const BYTES_ALIGN : usize = <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_ALIGN;

            fn to_bytes(&self, endianness: bool) -> [u8;<Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
                let mut bytes = [0u8;<Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                let discriminant = self.discriminant();
                match self {
                    $(
                        Self::$variant_identifier => {

                            let mut o = 0;
                            bytes[o..(o+<$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)].copy_from_slice(
                                &<$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(&discriminant,endianness)
                            );
                            bytes
                        },
                    )*
                    Self::TODO => <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::to_bytes(&discriminant, endianness)
                }
            }

            fn from_bytes(bytes: [u8;<Self as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
                let mut o = 0;
                let mut discriminant_bytes = [0u8; <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                discriminant_bytes.copy_from_slice(&bytes[o..(o+<$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE)]);
                let discriminant = <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(discriminant_bytes, endianness);
                match discriminant {
                    $(
                        $variant_discriminant => {
                            Self::$variant_identifier
                        },
                    )*
                    _ => Self::TODO
                }
            }


            fn from_bytes_pointer(bytes_pointer: *const u8, endianness: bool) -> Self {
                let o = 0;
                let mut discriminant_bytes = [0u8; <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
                unsafe {
                    core::ptr::copy_nonoverlapping(bytes_pointer.add(o), discriminant_bytes.as_mut_ptr(), <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE);
                }
                let discriminant = <$enum_discriminant_type as $crate::traits::Bytes<crate::Origin, crate::Origin>>::from_bytes(discriminant_bytes, endianness);
                match discriminant {
                    $(
                        $variant_discriminant => {
                            Self::$variant_identifier
                        },
                    )*
                    _ => Self::TODO
                }
            }
        }

        impl $enum_identifier {
            pub fn from(discriminant: $enum_discriminant_type) -> Self {
                match discriminant {
                    $($variant_discriminant => Self::$variant_identifier,)*
                    _ => Self::TODO,
                }
            }

            pub fn to(&self) -> $enum_discriminant_type {
                match *self {
                    $(Self::$variant_identifier => $variant_discriminant,)*
                    _ => <$enum_discriminant_type>::MAX
                }
            }

            pub fn str(&self) -> &str {
                match self {
                    $(Self::$variant_identifier => $variant_description,)*
                    _ => "TODO"
                }
            }

            pub fn acronym(&self) -> &str {
                match *self {
                    $(Self::$variant_identifier => $variant_acronym,)*
                    _ => "Unknown error",
                }
            }
        }

        impl core::ops::BitOr for $enum_identifier {
            type Output = $enum_discriminant_type;
            fn bitor(self, rhs: Self) -> Self::Output {
                self.to() | rhs.to()
            }
        }

        impl core::ops::BitAnd for $enum_identifier {
            type Output = $enum_discriminant_type;
            fn bitand(self, rhs: Self) -> Self::Output {
                self.to() & rhs.to()
            }
        }

        impl core::ops::Add for $enum_identifier {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                $enum_identifier::from(self.to() | rhs.to())
            }
        }

        impl core::ops::Sub for $enum_identifier {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                $enum_identifier::from(self.to() & !rhs.to())
            }
        }

        impl core::fmt::Display for $enum_identifier {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}:{}",self.to(), self.str())
            }
        }

        impl core::fmt::Debug for $enum_identifier {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}:{}",self.to(), self.acronym())
            }
        }
    };
}

pub use enum_flag;
