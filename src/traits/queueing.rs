use crate::queue::Queue;
use crate::traits::{Allocatable, Loading};

pub trait Queueing<Node, Load>
where
    Node: Loading<Load>,
    Load: Allocatable,
{
    fn enqueue(&self, queue: &mut Queue<Node, Load>) -> bool;
}
