pub mod queue;

use crate::traits::Allocatable;
use crate::traits::Loading;

#[derive(Debug, Clone, Copy)]
pub struct DoubleLinkedNode<Load>
where
    Load: Allocatable,
{
    pub value: Load,
    pub ancestor: *const DoubleLinkedNode<Load>,
    pub sucessor: *const DoubleLinkedNode<Load>,
}

impl<Load> DoubleLinkedNode<Load>
where
    Load: Allocatable,
{
    pub fn set_sucessor(&mut self, node: *const Self) {
        self.sucessor = node;
    }

    pub fn set_ancestor(&mut self, node: *const Self) {
        self.ancestor = node;
    }
}

impl<Load: Allocatable> Loading<Load> for DoubleLinkedNode<Load> {
    fn new(value: Load) -> Self {
        Self {
            value,
            ancestor: core::ptr::null(),
            sucessor: core::ptr::null(),
        }
    }
}
