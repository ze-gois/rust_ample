pub fn terminate(head: &str) -> *const u8 {
    let layout = crate::traits::allocatable::Layout {
        size: head.len().checked_add(1).unwrap(),
        align: core::mem::align_of::<u8>(),
    };

    let tailed = unsafe {
        <u8 as crate::traits::Allocatable<crate::Origin, crate::Origin>>::allocate_zeroed(layout)
    };

    if tailed.is_null() {
        panic!("allocation failed");
    }

    unsafe {
        core::ptr::copy_nonoverlapping(head.as_ptr(), tailed, head.len());
        *tailed.add(head.len()) = 0;
    };

    tailed as *const u8
}
