//! A small growable contiguous collection for `no_std` environments.
//!
//! `Vec<T>` owns its buffer and uses Rust's global allocator through the
//! `alloc` crate. The container itself does not depend on `std`; bare-metal
//! consumers only need to provide an appropriate `#[global_allocator]`.

use alloc::alloc::{alloc, dealloc, handle_alloc_error, realloc};
use core::alloc::Layout;
use core::fmt;
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::mem::{self, ManuallyDrop};
use core::ops::{Deref, DerefMut};
use core::ptr::{self, NonNull};
use core::slice;

pub struct Vec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> Vec<T> {
    pub const fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: if mem::size_of::<T>() == 0 {
                usize::MAX
            } else {
                0
            },
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut vec = Self::new();
        vec.reserve_exact(capacity);
        vec
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn capacity(&self) -> usize {
        self.cap
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.reserve(1);
        }

        let index = self.len;
        self.len = self
            .len
            .checked_add(1)
            .unwrap_or_else(|| capacity_overflow());

        unsafe {
            self.element_ptr(index).write(value);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        unsafe { Some(self.element_ptr(self.len).read()) }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.len, "insertion index out of bounds");

        self.reserve(1);

        unsafe {
            let ptr = self.element_ptr(index);
            if mem::size_of::<T>() != 0 {
                ptr::copy(ptr, ptr.add(1), self.len - index);
            }
            ptr.write(value);
        }

        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "removal index out of bounds");

        unsafe {
            let ptr = self.element_ptr(index);
            let value = ptr.read();

            if mem::size_of::<T>() != 0 {
                ptr::copy(ptr.add(1), ptr, self.len - index - 1);
            }

            self.len -= 1;
            value
        }
    }

    pub fn clear(&mut self) {
        self.truncate(0);
    }

    pub fn truncate(&mut self, len: usize) {
        while self.len > len {
            self.len -= 1;
            unsafe {
                ptr::drop_in_place(self.element_ptr(self.len));
            }
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        let required = self
            .len
            .checked_add(additional)
            .unwrap_or_else(|| capacity_overflow());

        if required <= self.cap {
            return;
        }

        let doubled = self.cap.saturating_mul(2);
        let new_capacity = required.max(doubled).max(4);
        self.grow_to(new_capacity);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        let required = self
            .len
            .checked_add(additional)
            .unwrap_or_else(|| capacity_overflow());

        if required > self.cap {
            self.grow_to(required);
        }
    }

    fn grow_to(&mut self, new_capacity: usize) {
        if mem::size_of::<T>() == 0 {
            return;
        }

        assert!(new_capacity >= self.len);
        let new_layout = layout_for::<T>(new_capacity);

        let raw = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = layout_for::<T>(self.cap);
            unsafe {
                realloc(
                    self.ptr.cast::<u8>().as_ptr(),
                    old_layout,
                    new_layout.size(),
                )
            }
        };

        self.ptr = match NonNull::new(raw.cast::<T>()) {
            Some(ptr) => ptr,
            None => handle_alloc_error(new_layout),
        };
        self.cap = new_capacity;
    }

    unsafe fn element_ptr(&self, index: usize) -> *mut T {
        if mem::size_of::<T>() == 0 {
            self.ptr.as_ptr()
        } else {
            unsafe { self.ptr.as_ptr().add(index) }
        }
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        self.truncate(0);
        deallocate_buffer(self.ptr, self.cap);
    }
}

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut clone = Self::with_capacity(self.len);
        clone.extend(self.iter().cloned());
        clone
    }
}

impl<T: fmt::Debug> fmt::Debug for Vec<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl<T: PartialEq> PartialEq for Vec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq> Eq for Vec<T> {}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T> AsRef<[T]> for Vec<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<[T]> for Vec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> Extend<T> for Vec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iterator = iter.into_iter();
        let (lower, _) = iterator.size_hint();
        self.reserve(lower);

        for value in iterator {
            self.push(value);
        }
    }
}

impl<T> core::iter::FromIterator<T> for Vec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iterator = iter.into_iter();
        let (lower, _) = iterator.size_hint();
        let mut vec = Self::with_capacity(lower);
        vec.extend(iterator);
        vec
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct IntoIter<T> {
    ptr: NonNull<T>,
    cap: usize,
    front: usize,
    back: usize,
    marker: PhantomData<T>,
}

impl<T> IntoIter<T> {
    unsafe fn element_ptr(&self, index: usize) -> *mut T {
        if mem::size_of::<T>() == 0 {
            self.ptr.as_ptr()
        } else {
            unsafe { self.ptr.as_ptr().add(index) }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }

        let index = self.front;
        self.front += 1;
        unsafe { Some(self.element_ptr(index).read()) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.back - self.front;
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }

        self.back -= 1;
        unsafe { Some(self.element_ptr(self.back).read()) }
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}
impl<T> FusedIterator for IntoIter<T> {}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        while self.front != self.back {
            unsafe {
                ptr::drop_in_place(self.element_ptr(self.front));
            }
            self.front += 1;
        }

        deallocate_buffer(self.ptr, self.cap);
    }
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let this = ManuallyDrop::new(self);

        IntoIter {
            ptr: this.ptr,
            cap: this.cap,
            front: 0,
            back: this.len,
            marker: PhantomData,
        }
    }
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

fn layout_for<T>(capacity: usize) -> Layout {
    let layout = Layout::array::<T>(capacity).unwrap_or_else(|_| capacity_overflow());

    if layout.size() > isize::MAX as usize {
        capacity_overflow();
    }

    layout
}

fn deallocate_buffer<T>(ptr: NonNull<T>, capacity: usize) {
    if capacity == 0 || mem::size_of::<T>() == 0 {
        return;
    }

    unsafe {
        dealloc(ptr.cast::<u8>().as_ptr(), layout_for::<T>(capacity));
    }
}

#[cold]
#[inline(never)]
fn capacity_overflow() -> ! {
    panic!("capacity overflow")
}


