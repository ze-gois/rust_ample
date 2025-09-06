use crate::traits::Allocatable;
use crate::traits::Loading;
use core::fmt::Debug;
use core::marker::PhantomData;

pub struct Queue<Node, Load>
where
    Node: Loading<Load>,
    Load: Allocatable,
{
    pub former: *const Node,
    pub latter: *const Node,
    _phantom: PhantomData<Load>,
}

impl<Node, Load> Queue<Node, Load>
where
    Node: Loading<Load>,
    Load: Allocatable,
{
    pub fn new() -> Queue<Node, Load> {
        Queue {
            former: core::ptr::null_mut(),
            latter: core::ptr::null_mut(),
            _phantom: PhantomData,
        }
    }

    pub fn enqueue(&mut self, node: *const Node) {
        if self.former.is_null() {
            self.former = node;
            self.latter = node;
        } else {
            // This is a simplification - in a real implementation,
            // you'd need to set the successor/ancestor based on node type
            self.latter = node;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.former.is_null()
    }

    pub fn new_5(loads: &[Load; 5]) -> Queue<Node, Load>
    where
        Load: Clone,
    {
        let mut queue = Queue::new();
        for l in 0..5 {
            // Create a new node with a clone of the load
            let _node = queue.enqueue(&Node::new(loads[l].clone()));
            // In a real implementation, you would:
            // 1. Properly allocate the node with Allocatable
            // 2. Add it to the queue with enqueue
        }
        queue
    }
}

impl<Node, Load> core::fmt::Debug for Queue<Node, Load>
where
    Node: Loading<Load> + Debug + Copy + Clone,
    Load: Allocatable,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Queue {{\n")?;
        write!(f, "\t Former {:?}\n", self.former)?;
        let mut p = self.former.clone();
        if !p.is_null() {
            unsafe {
                while p != self.latter.add(1) {
                    write!(f, "\t\t Entry {:?}\n", *p)?;
                    p = p.add(1);
                }
            }
        }
        write!(f, "\t Latter {:?}\n", self.latter)?;
        write!(f, "}} Queue\n")
    }
}
