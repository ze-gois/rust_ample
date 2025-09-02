use crate::traits::AmpleAlloc;

pub struct ListNode<T> {
    pub next: *mut ListNode<T>,
    pub elem: T,
}

pub struct List<T, A: AmpleAlloc> {
    head: *mut ListNode<T>,
    alloc: A,
}

impl<T, A: AmpleAlloc + Copy> List<T, A> {
    pub const fn new(alloc: A) -> Self {
        Self {
            head: core::ptr::null_mut(),
            alloc,
        }
    }

    pub fn push_front(&mut self, value: T) -> Result<(), ()> {
        let layout = core::alloc::Layout::new::<ListNode<T>>();
        let ptr = unsafe { self.alloc.allocate(layout) };
        if ptr.is_null() {
            return Err(());
        }
        unsafe {
            let node = ptr as *mut ListNode<T>;
            node.write(ListNode {
                next: self.head,
                elem: value,
            });
            self.head = node;
        }
        Ok(())
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }
        unsafe {
            let node = self.head;
            self.head = (*node).next;
            let value = core::ptr::read(&(*node).elem);
            let layout = core::alloc::Layout::new::<ListNode<T>>();
            self.alloc.deallocate(node as *mut u8, layout);
            Some(value)
        }
    }
}
