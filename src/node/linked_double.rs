pub mod queue;

// use crate::traits::Allocatable;

// #[derive(Debug, Clone, Copy)]
// pub struct DoubleLinkedNode<Load, Observer, Reference>
// where
//     Load: Allocatable<Observer, Reference>,
// {
//     pub value: Load,
//     pub ancestor: *const DoubleLinkedNode<Load, Observer, Reference>,
//     pub sucessor: *const DoubleLinkedNode<Load, Observer, Reference>,
// }

// impl<Load> DoubleLinkedNode<Load, crate::Origin, crate::Origin>
// where
//     Load: Allocatable<crate::Origin, crate::Origin>,
// {
//     pub fn set_sucessor(&mut self, node: *const Self) {
//         self.sucessor = node;
//     }

//     pub fn set_ancestor(&mut self, node: *const Self) {
//         self.ancestor = node;
//     }
// }

// impl<Load: Allocatable> Loading<Load> for DoubleLinkedNode<Load> {
//     fn new(value: Load) -> Self {
//         Self {
//             value,
//             ancestor: core::ptr::null(),
//             sucessor: core::ptr::null(),
//         }
//     }
// }

use crate::traits::Allocatable;
use crate::traits::Bytes;
use core::marker::PhantomData;

#[derive(Debug)]
pub struct DoubleLinkedNode<Origin, AllocatorOrigin, T>
where
    T: Bytes<Origin>,
{
    pub value: T,
    pub ancestor: Option<*mut Self>,
    pub sucessor: Option<*mut Self>,
    _phantom_o: PhantomData<Origin>,
    _phantom_a: PhantomData<AllocatorOrigin>,
}

impl<Origin, AllocatorOrigin, T> DoubleLinkedNode<Origin, AllocatorOrigin, T>
where
    T: Bytes<Origin>,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            ancestor: None,
            sucessor: None,
            _phantom_o: PhantomData,
            _phantom_a: PhantomData,
        }
    }

    /// Create a node with the given value and allocate it using the provided allocator type
    pub fn allocate_node<A>(value: T) -> *mut Self
    where
        A: Allocatable<AllocatorOrigin>,
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
        A: Allocatable<AllocatorOrigin>,
    {
        // Cast the pointer to the allocator's expected type
        A::deallocate(ptr as *mut A, 1)
    }

    /// Safely get a reference to the next node
    pub fn ancestor(&self) -> Option<&Self> {
        unsafe { self.ancestor.map(|ptr| &*ptr) }
    }

    /// Safely get a mutable reference to the ancestor node
    pub fn ancestor_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.ancestor.map(|ptr| &mut *ptr) }
    }

    /// Set the ancestor node
    pub fn set_ancestor(&mut self, ancestor: Option<*mut Self>) {
        self.ancestor = ancestor;
    }

    /// Safely get a reference to the next node
    pub fn sucessor(&self) -> Option<&Self> {
        unsafe { self.sucessor.map(|ptr| &*ptr) }
    }

    /// Safely get a mutable reference to the sucessor node
    pub fn sucessor_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.sucessor.map(|ptr| &mut *ptr) }
    }

    /// Set the sucessor node
    pub fn set_sucessor(&mut self, sucessor: Option<*mut Self>) {
        self.sucessor = sucessor;
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

impl<Origin, AllocatorOrigin, T> Clone for DoubleLinkedNode<Origin, AllocatorOrigin, T>
where
    T: Bytes<Origin> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            ancestor: self.ancestor,
            sucessor: self.sucessor,
            _phantom_o: PhantomData,
            _phantom_a: PhantomData,
        }
    }
}
