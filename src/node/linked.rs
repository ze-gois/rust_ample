use crate::traits::Allocatable;
use crate::traits::Bytes;
use core::marker::PhantomData;

#[derive(Debug)]
pub struct LinkedNode<Origin, T>
where
    T: Bytes<Origin>,
{
    pub value: T,
    pub next: Option<*mut Self>,
    _phantom_o: PhantomData<Origin>,
}

impl<Origin, T> LinkedNode<Origin, T>
where
    T: Bytes<Origin>,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            _phantom_o: PhantomData,
        }
    }

    /// Create a node with the given value and allocate it using the provided allocator type
    pub fn allocate_node<A>(value: T) -> *mut Self
    where
        A: Allocatable<Origin>,
    {
        // Use the A's allocate method but cast the result to our node type
        let ptr = unsafe {
            // Allocate raw memory
            let raw_ptr = A::allocate(1);

            // Cast to our node type
            let node_ptr = raw_ptr as *mut Self;

            // Initialize the node
            *node_ptr = Self::new(value);

            node_ptr
        };

        ptr
    }

    /// Deallocate a node using the provided allocator type
    pub fn deallocate_node<A>(ptr: *mut Self) -> bool
    where
        A: Allocatable<Origin>,
    {
        // Cast the pointer to the allocator's expected type
        A::deallocate(ptr as *mut A, 1)
    }

    /// Safely get a reference to the next node
    pub fn next(&self) -> Option<&Self> {
        unsafe { self.next.map(|ptr| &*ptr) }
    }

    /// Safely get a mutable reference to the next node
    pub fn next_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.next.map(|ptr| &mut *ptr) }
    }

    /// Set the next node
    pub fn set_next(&mut self, next: Option<*mut Self>) {
        self.next = next;
    }

    /// Get a reference to the value stored in the node
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get a mutable reference to the value stored in the node
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<Origin, T> Clone for LinkedNode<Origin, T>
where
    T: Bytes<Origin> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            next: self.next,
            _phantom_o: PhantomData,
        }
    }
}
