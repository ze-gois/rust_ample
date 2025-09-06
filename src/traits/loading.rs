use crate::traits::Allocatable;

pub trait Loading<Load: Allocatable> {
    fn new(value: Load) -> Self;
}
