use crate::traits::Allocating;
use core::alloc::Layout;

pub fn terminate<A: Allocating>(head: &str) -> *mut u8 {
    let Ok(layout) = Layout::array::<u8>(head.len().saturating_add(1)) else {
        return core::ptr::null_mut();
    };

    let tailed = A::allocate_zeroed(layout);
    if tailed.is_null() {
        return tailed;
    }

    unsafe {
        core::ptr::copy_nonoverlapping(head.as_ptr(), tailed, head.len());
        *tailed.add(head.len()) = 0;
    }

    tailed
}
