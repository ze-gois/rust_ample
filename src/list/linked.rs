use crate::node::LinkedNode;
use crate::traits::Allocatable;
use crate::traits::Bytes;
// use crate::traits::allocatable::HeapAllocatable;
use core::marker::PhantomData;

/// A singly linked list implementation that can work with different allocation strategies
pub struct LinkedList<Origin, T, A>
where
    T: Bytes<Origin>,
    A: Allocatable<Origin>,
{
    former: Option<*mut LinkedNode<Origin, T>>,
    latter: Option<*mut LinkedNode<Origin, T>>,
    numerosity: usize,
    _phantom_o: PhantomData<Origin>,
    _phantom_a: PhantomData<A>,
}

impl<Origin, T, A> LinkedList<Origin, T, A>
where
    T: Bytes<Origin>,
    A: Allocatable<Origin>,
{
    /// Create a new empty linked list
    pub fn new() -> Self {
        Self {
            former: None,
            latter: None,
            numerosity: 0,
            _phantom_o: PhantomData,
            _phantom_a: PhantomData,
        }
    }

    /// Returns the number of elements in the list
    pub fn numerosity(&self) -> usize {
        self.numerosity
    }

    /// Returns true if the list is empty
    pub fn is_empty(&self) -> bool {
        self.numerosity == 0
    }

    /// Add an element to the front of the list
    pub fn push_front(&mut self, value: T) {
        let new_node = LinkedNode::allocate_node::<A>(value);

        unsafe {
            if let Some(old_former) = self.former {
                (*new_node).next = Some(old_former);
                self.former = Some(new_node);
            } else {
                // Empty list
                self.former = Some(new_node);
                self.latter = Some(new_node);
            }
        }

        self.numerosity += 1;
    }

    /// Add an element to the back of the list
    pub fn push_back(&mut self, value: T) {
        let new_node = LinkedNode::allocate_node::<A>(value);

        unsafe {
            if let Some(old_latter) = self.latter {
                (*old_latter).next = Some(new_node);
                self.latter = Some(new_node);
            } else {
                // Empty list
                self.former = Some(new_node);
                self.latter = Some(new_node);
            }
        }

        self.numerosity += 1;
    }

    /// Remove and return the element at the front of the list
    pub fn pop_front(&mut self) -> Option<T> {
        self.former.map(|former_ptr| unsafe {
            let former = &*former_ptr;
            let value = core::ptr::read(former.value());

            self.former = former.next;

            if self.former.is_none() {
                self.latter = None;
            }

            LinkedNode::deallocate_node::<A>(former_ptr);
            self.numerosity -= 1;

            value
        })
    }

    /// Get a reference to the first element
    pub fn front(&self) -> Option<&T> {
        unsafe { self.former.map(|node| (*node).value()) }
    }

    /// Get a reference to the last element
    pub fn back(&self) -> Option<&T> {
        unsafe { self.latter.map(|node| (*node).value()) }
    }

    /// Clear all nodes and deallocate memory
    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    /// Get an iterator over the values in the list
    pub fn iter(&self) -> Iter<Origin, T> {
        Iter {
            current: self.former,
            _phantom_o: PhantomData,
        }
    }
}

// Implement Drop to ensure memory is freed when the list is dropped
impl<Origin, T, A> Drop for LinkedList<Origin, T, A>
where
    T: Bytes<Origin>,
    A: Allocatable<Origin>,
{
    fn drop(&mut self) {
        self.clear();
    }
}

// Iterator for LinkedList
pub struct Iter<'a, Origin: 'a, T: 'a>
where
    T: Bytes<Origin>,
{
    current: Option<*mut LinkedNode<Origin, T>>,
    _phantom_o: PhantomData<&'a Origin>,
}

impl<'a, Origin: 'a, T: 'a> Iterator for Iter<'a, Origin, T>
where
    T: Bytes<Origin>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let current = self.current?;
            let current_ref = &*current;
            let result = current_ref.value();
            self.current = current_ref.next;
            Some(result)
        }
    }
}

// // Define a default constructor that uses heap allocation
// impl<Origin, T> LinkedList<Origin, T, T>
// where
//     T: Bytes<Origin> + HeapAllocatable<Origin>,
// {
//     /// Creates a new LinkedList that uses the default heap allocation strategy
//     pub fn with_heap_allocation() -> Self {
//         Self::new()
//     }
// }
