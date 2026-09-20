use ample::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn grows_and_preserves_values() {
    let mut values = Vec::with_capacity(1);
    for value in 0..32 {
        values.push(value);
    }

    assert_eq!(values.len(), 32);
    assert!(values.capacity() >= 32);
    assert_eq!(values[0], 0);
    assert_eq!(values[31], 31);
}

#[test]
fn collect_clone_and_owned_iteration_work() {
    let values: Vec<_> = (0..8).collect();
    let clone = values.clone();

    assert_eq!(values, clone);
    assert_eq!(clone.into_iter().sum::<i32>(), 28);
}

#[test]
fn insert_remove_and_pop_keep_order() {
    let mut values: Vec<_> = [1, 3, 4].into_iter().collect();
    values.insert(1, 2);

    assert_eq!(values.as_slice(), &[1, 2, 3, 4]);
    assert_eq!(values.remove(2), 3);
    assert_eq!(values.pop(), Some(4));
    assert_eq!(values.as_slice(), &[1, 2]);
}

#[test]
fn nested_vectors_are_supported() {
    let rows: Vec<Vec<i32>> = (0..3)
        .map(|row| (0..2).map(|column| row * 10 + column).collect())
        .collect();

    assert_eq!(rows[2].as_slice(), &[20, 21]);
}

struct DroppableZst;

static DROPS: AtomicUsize = AtomicUsize::new(0);

impl Drop for DroppableZst {
    fn drop(&mut self) {
        DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn zero_sized_values_are_dropped_once_per_element() {
    DROPS.store(0, Ordering::SeqCst);

    {
        let mut values = Vec::new();
        values.push(DroppableZst);
        values.push(DroppableZst);
        values.push(DroppableZst);
        assert_eq!(values.len(), 3);
    }

    assert_eq!(DROPS.load(Ordering::SeqCst), 3);
}
