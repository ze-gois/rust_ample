#[macro_use]
pub mod r#bool;

#[macro_use]
pub mod r#char;

#[macro_use]
pub mod numeric;

#[macro_use]
pub mod primitive;

#[macro_use]
pub mod unit;

#[rustfmt::skip]
#[macro_export]
macro_rules! trait_implement_primitives {
    ($($t:ty),*) => {

        $crate::trait_implement_primitive_phantom_bytes!();

        $crate::trait_implement_primitive!(
            true, (), bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );

        $crate::trait_implement_primitive_unit_bytes!();
        $crate::trait_implement_primitive_bool_bytes!();
        $crate::trait_implement_primitive_char_bytes!();
        $crate::trait_implement_primitive_numeric_bytes!(
            f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );

        $crate::trait_implement_bytes_slice!((), bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
        // $crate::trait_implement_primitive_option_bytes!((), bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
        $crate::trait_implement_bytes_option!((), bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
        $crate::trait_implement_bytes_pointer!();
        // impl<T: $crate::traits::Bytes<crate::Origin,crate::Origin>> Clone for T {
        //     fn clone(&self) -> Self {
        //         <T>::from_bytes(self.to_bytes(true),true)
        //     }
        // }
        //

        // $crate::trait_implement_primitive_slice_bytes!((), bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
        // $crate::trait_implement_primitive_array_bytes!(bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
        // $crate::trait_implement_primitive_array_reference_bytes!(bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
    };
}
