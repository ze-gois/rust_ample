pub mod queue;

use crate::traits::Allocatable;

#[derive(Debug, Clone, Copy)]
pub struct DoubleLinkedNode<Load, Observer, Reference>
where
    Load: Allocatable<Observer, Reference>,
{
    pub value: Load,
    pub ancestor: *const DoubleLinkedNode<Load, Observer, Reference>,
    pub sucessor: *const DoubleLinkedNode<Load, Observer, Reference>,
}

impl<Load> DoubleLinkedNode<Load, crate::Origin, crate::Origin>
where
    Load: Allocatable<crate::Origin, crate::Origin>,
{
    pub fn set_sucessor(&mut self, node: *const Self) {
        self.sucessor = node;
    }

    pub fn set_ancestor(&mut self, node: *const Self) {
        self.ancestor = node;
    }
}

// impl<Load: Allocatable> Loading<Load> for DoubleLinkedNode<Load> {
//     fn new(value: Load) -> Self {
//         Self {
//             value,
//             ancestor: core::ptr::null(),
//             sucessor: core::ptr::null(),
//         }
//     }
// }
