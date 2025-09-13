pub trait Primitive<Origin> {
    const IS_PRIMITIVE: bool;
}

pub trait NonPrimitive<Origin> {
    const IS_PRIMITIVE: bool = false;
}
