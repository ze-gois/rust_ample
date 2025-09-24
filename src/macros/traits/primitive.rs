#[macro_export]
macro_rules! trait_primitive_place {
    () => {
        pub trait Primitive<Origin> {
            const IS_PRIMITIVE: bool;
        }

        pub trait NonPrimitive<Origin> {
            const IS_PRIMITIVE: bool = false;
        }
    };
}

#[macro_export]
macro_rules! trait_implement_primitive {
    ($tv:expr, $($t:ty),*) => {
        $(
            impl $crate::traits::Primitive<crate::Origin> for $t {
                const IS_PRIMITIVE: bool = $tv;
            }
        )*
    };
}

pub use trait_implement_primitive;
pub use trait_primitive_place;
