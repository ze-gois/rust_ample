pub fn terminate(head: &str) -> *const u8 {
    let layout = core::alloc::Layout::array::<u8>(head.len() + 1).unwrap();
    let tailed = unsafe { alloc::alloc::alloc_zeroed(layout) };

    if tailed.is_null() {
        panic!("allocation failed");
    }

    unsafe {
        core::ptr::copy_nonoverlapping(head.as_ptr(), tailed, head.len());
        *tailed.add(head.len()) = 0;
    };

    tailed as *const u8
}
