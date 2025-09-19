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
