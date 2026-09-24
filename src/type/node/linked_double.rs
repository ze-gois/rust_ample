use crate::traits::Allocating;
use core::alloc::Layout;

#[derive(Debug)]
pub struct DoubleLinkedNode<T> {
    pub value: T,
    pub previous: Option<*mut Self>,
    pub next: Option<*mut Self>,
}

impl<T> DoubleLinkedNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            previous: None,
            next: None,
        }
    }

    pub fn allocate<A: Allocating>(value: T) -> *mut Self {
        let layout = Layout::new::<Self>();
        let pointer = A::allocate(layout) as *mut Self;

        if pointer.is_null() {
            return pointer;
        }

        unsafe {
            pointer.write(Self::new(value));
        }

        pointer
    }

    /// # Safety
    ///
    /// `pointer` must designate a node allocated by `A` with this node's
    /// layout, and its value must already have been moved or dropped.
    pub unsafe fn deallocate<A: Allocating>(pointer: *mut Self) -> bool {
        if pointer.is_null() {
            return false;
        }

        unsafe { A::deallocate(pointer as *mut u8, Layout::new::<Self>()) }
    }

    pub fn previous(&self) -> Option<&Self> {
        unsafe { self.previous.map(|pointer| &*pointer) }
    }

    pub fn previous_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.previous.map(|pointer| &mut *pointer) }
    }

    pub fn set_previous(&mut self, previous: Option<*mut Self>) {
        self.previous = previous;
    }

    pub fn next(&self) -> Option<&Self> {
        unsafe { self.next.map(|pointer| &*pointer) }
    }

    pub fn next_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.next.map(|pointer| &mut *pointer) }
    }

    pub fn set_next(&mut self, next: Option<*mut Self>) {
        self.next = next;
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: Clone> Clone for DoubleLinkedNode<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            previous: self.previous,
            next: self.next,
        }
    }
}
