use crate::node::LinkedNode;
use crate::traits::Allocatable;
use crate::traits::Bytes;
use crate::traits::allocatable::AllocatableResult;
// use crate::traits::allocatable::HeapAllocatable;
use core::marker::PhantomData;

/// A singly linked list implementation that can work with different allocation strategies
#[derive(Debug)]
pub struct LinkedList<BytesOrigin, AllocatorOrigin, B, A>
where
    B: Bytes<BytesOrigin>,
    A: Allocatable<AllocatorOrigin>,
{
    meta: Option<*mut B>,
    former: Option<*mut LinkedNode<BytesOrigin, AllocatorOrigin, B>>,
    latter: Option<*mut LinkedNode<BytesOrigin, AllocatorOrigin, B>>,
    numerosity: usize,
    _phantom_o: PhantomData<BytesOrigin>,
    _phantom_p: PhantomData<AllocatorOrigin>,
    _phantom_a: PhantomData<A>,
}

impl<BytesOrigin, AllocatorOrigin, B, A> LinkedList<BytesOrigin, AllocatorOrigin, B, A>
where
    B: Bytes<BytesOrigin>,
    A: Allocatable<AllocatorOrigin>,
{
    /// Create a new empty linked list
    pub fn new() -> Self {
        Self {
            meta: None,
            former: None,
            latter: None,
            numerosity: 0,
            _phantom_o: PhantomData,
            _phantom_p: PhantomData,
            _phantom_a: PhantomData,
        }
    }

    /// Returns the number of elements in the list
    pub fn meta(&self) -> Option<*mut B> {
        self.meta
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
    pub fn push_front(&mut self, value: B) -> core::result::Result<bool, A::Error> {
        let new_node = LinkedNode::allocate_node::<A>(value)?.as_ptr()
            as *mut LinkedNode<BytesOrigin, AllocatorOrigin, B>;

        // <A::Ok as crate::traits::allocatable::Pointer>::from_raw(new_node);

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
        core::result::Result::Ok(true)
    }

    /// Add an element to the back of the list
    pub fn push_back(&mut self, value: B) -> core::result::Result<bool, A::Error> {
        let new_node = LinkedNode::allocate_node::<A>(value)?.as_ptr()
            as *mut LinkedNode<BytesOrigin, AllocatorOrigin, B>;

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
        core::result::Result::Ok(true)
    }

    /// Remove and return the element at the front of the list
    pub fn pop_front(&mut self) -> core::result::Result<Option<B>, A::Error> {
        let former = self.former.unwrap();

        self.former = unsafe { (*former).next };

        if self.former.is_none() {
            self.latter = None;
        }

        self.numerosity -= 1;

        LinkedNode::deallocate_node::<A>(former)?;

        core::result::Result::Ok(None)
    }

    /// Get a reference to the first element
    pub fn front(&self) -> Option<&B> {
        unsafe { self.former.map(|node| (*node).value()) }
    }

    /// Get a reference to the last element
    pub fn back(&self) -> Option<&B> {
        unsafe { self.latter.map(|node| (*node).value()) }
    }

    /// Clear all nodes and deallocate memory
    pub fn clear(&mut self) -> core::result::Result<(), A::Error> {
        while self.pop_front()?.is_some() {}
        core::result::Result::Ok(())
    }

    /// Get an iterator over the values in the list
    pub fn iter(&self) -> Iter<'_, BytesOrigin, AllocatorOrigin, B> {
        Iter {
            current: self.former,
            _phantom_o: PhantomData,
            _phantom_p: PhantomData,
        }
    }
}

// Implement Drop to ensure memory is freed when the list is dropped
impl<Origin, AllocatorOrigin, B, A> Drop for LinkedList<Origin, AllocatorOrigin, B, A>
where
    B: Bytes<Origin>,
    A: Allocatable<AllocatorOrigin>,
{
    fn drop(&mut self) {
        let _ = self.clear();
    }
}

// Iterator for LinkedList
pub struct Iter<'a, Origin: 'a, AllocatorOrigin: 'a, B: 'a>
where
    B: Bytes<Origin>,
{
    current: Option<*mut LinkedNode<Origin, AllocatorOrigin, B>>,
    _phantom_o: PhantomData<&'a Origin>,
    _phantom_p: PhantomData<&'a AllocatorOrigin>,
}

impl<'a, Origin: 'a, AllocatorOrigin: 'a, B: 'a> Iterator for Iter<'a, Origin, AllocatorOrigin, B>
where
    B: Bytes<Origin>,
{
    type Item = &'a B;

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
// impl<Origin, B> LinkedList<Origin, B, B>
// where
//     B: Bytes<Origin> + HeapAllocatable<Origin>,
// {
//     /// Creates a new LinkedList that uses the default heap allocation strategy
//     pub fn with_heap_allocation() -> Self {
//         Self::new()
//     }
// }
