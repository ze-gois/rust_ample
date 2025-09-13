use crate::traits::{Allocatable, Bytes};

pub fn terminate<Origin, A: Allocatable<Origin>>(head: &str) -> *const u8
where
    u8: Bytes<Origin>,
{
    let tailed: *mut u8 =
        A::allocate_zeroed(head.len() * core::mem::size_of::<u8>() + 1) as *mut u8;

    if tailed.is_null() {
        panic!("allocation failed");
    }

    unsafe {
        core::ptr::copy_nonoverlapping(head.as_ptr(), tailed, head.len());
        *tailed.add(head.len()) = 0;
    };

    tailed as *const u8
}
