use super::DoubleLinkedNode;
use crate::Queue;
use crate::traits::{Allocatable, Queueing};

impl<Load> Queueing<DoubleLinkedNode<Load>, Load> for DoubleLinkedNode<Load>
where
    Load: Allocatable,
{
    fn enqueue(&self, queue: &mut Queue<DoubleLinkedNode<Load>, Load>) -> bool {
        queue.enqueue(self as *const DoubleLinkedNode<Load>);
        true
    }
}
