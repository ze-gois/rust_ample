use super::LinkedNode;
use crate::Queue;
use crate::traits::{Allocatable, Queueing};

impl<Load> Queueing<LinkedNode<Load>, Load> for LinkedNode<Load>
where
    Load: Allocatable,
{
    fn enqueue(&self, queue: &mut Queue<LinkedNode<Load>, Load>) -> bool {
        queue.enqueue(self as *const LinkedNode<Load>);
        true
    }
}
