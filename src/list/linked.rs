use crate::node::LinkedNode;
use crate::traits::Allocating;
use core::marker::PhantomData;

#[derive(Debug)]
pub struct LinkedList<T, A>
where
    A: Allocating,
{
    head: Option<*mut LinkedNode<T>>,
    tail: Option<*mut LinkedNode<T>>,
    length: usize,
    allocator: PhantomData<A>,
}

impl<T, A> LinkedList<T, A>
where
    A: Allocating,
{
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
            allocator: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn push_front(&mut self, value: T) -> bool {
        let node = LinkedNode::allocate::<A>(value);
        if node.is_null() {
            return false;
        }

        unsafe {
            (*node).next = self.head;
        }

        if self.tail.is_none() {
            self.tail = Some(node);
        }

        self.head = Some(node);
        self.length += 1;
        true
    }

    pub fn push_back(&mut self, value: T) -> bool {
        let node = LinkedNode::allocate::<A>(value);
        if node.is_null() {
            return false;
        }

        if let Some(tail) = self.tail {
            unsafe {
                (*tail).next = Some(node);
            }
        } else {
            self.head = Some(node);
        }

        self.tail = Some(node);
        self.length += 1;
        true
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node = self.head?;

        unsafe {
            self.head = (*node).next;
            if self.head.is_none() {
                self.tail = None;
            }

            self.length -= 1;
            let value = core::ptr::read(core::ptr::addr_of!((*node).value));
            let _ = LinkedNode::deallocate::<A>(node);
            Some(value)
        }
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.head.map(|node| (*node).value()) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.map(|node| (*node).value()) }
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head,
            lifetime: PhantomData,
        }
    }
}

impl<T, A> Default for LinkedList<T, A>
where
    A: Allocating,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A> Drop for LinkedList<T, A>
where
    A: Allocating,
{
    fn drop(&mut self) {
        self.clear();
    }
}

pub struct Iter<'list, T> {
    current: Option<*mut LinkedNode<T>>,
    lifetime: PhantomData<&'list T>,
}

impl<'list, T> Iterator for Iter<'list, T> {
    type Item = &'list T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let current = self.current?;
            let current_ref = &*current;
            self.current = current_ref.next;
            Some(current_ref.value())
        }
    }
}
