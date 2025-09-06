pub fn terminate<Observer, Reference>(head: &str) -> *const u8
where
    u8: crate::traits::Allocatable,
{
    let layout = crate::traits::allocatable::Layout {
        size: head.len().checked_add(1).unwrap(),
        align: core::mem::align_of::<u8>(),
    };

    let tailed =
        unsafe { <u8 as crate::traits::Allocatable<Observer, Reference>>::allocate_zeroed(layout) };

    if tailed.is_null() {
        panic!("allocation failed");
    }

    unsafe {
        core::ptr::copy_nonoverlapping(head.as_ptr(), tailed, head.len());
        *tailed.add(head.len()) = 0;
    };

    tailed as *const u8
}
