pub mod queue;

use crate::traits::Allocatable;
use crate::traits::Loading;

#[derive(Debug, Clone, Copy)]
pub struct LinkedNode<Load>
where
    Load: Allocatable,
{
    pub value: Load,
    pub sucessor: *const Self,
}

impl<Load: Allocatable> Loading<Load> for LinkedNode<Load> {
    fn new(value: Load) -> Self {
        Self {
            value,
            sucessor: core::ptr::null(),
        }
    }
}

impl<Load> LinkedNode<Load>
where
    Load: Allocatable,
{
    pub fn new(value: Load) -> Self {
        Self {
            value,
            sucessor: core::ptr::null(),
        }
    }

    pub fn set_sucessor(&mut self, node: *const Self) {
        self.sucessor = node;
    }
}
