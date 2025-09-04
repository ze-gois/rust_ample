pub mod double_linked;

// use crate::traits::Allocatable;
// use crate::traits::allocatable::Layout;

// pub struct ListNode<T> {
//     pub next: *mut ListNode<T>,
//     pub elem: T,
// }

// pub struct List<A: Copy, T: Allocatable<A>> {
//     head: *mut ListNode<T>,
//     alloc: A,
// }

// impl<T, A: Allocatable + Copy> List<T, A> {
//     pub const fn new(alloc: A) -> Self {
//         Self {
//             head: core::ptr::null_mut(),
//             alloc,
//         }
//     }

//     pub fn push_front(&mut self, value: T) -> Result<(), ()> {
//         let layout = Layout { size: 3, align: 4 };
//         let ptr = self.alloc.allocate(layout);
//         if ptr.is_null() {
//             return Err(());
//         }
//         unsafe {
//             let node = ptr as *mut ListNode<T>;
//             node.write(ListNode {
//                 next: self.head,
//                 elem: value,
//             });
//             self.head = node;
//         }
//         Ok(())
//     }

//     pub fn pop_front(&mut self) -> Option<T> {
//         if self.head.is_null() {
//             return None;
//         }
//         unsafe {
//             let node = self.head;
//             self.head = (*node).next;
//             let value = core::ptr::read(&(*node).elem);
//             let layout = Layout { size: 3, align: 4 };
//             self.alloc.deallocate(node as *mut u8, layout);
//             Some(value)
//         }
//     }
// }
