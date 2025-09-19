use crate::traits::{Allocatable, AllocatableResult, Bytes};

pub fn terminate<Origin, A: Allocatable<Origin>>(
    head: &str,
) -> core::result::Result<A::Ok, A::Error>
where
    u8: Bytes<Origin>,
{
    let tailed = A::allocate_zeroed(head.len() * core::mem::size_of::<u8>() + 1)?;
    let tailed = tailed.as_ptr() as *mut u8;
    if tailed.is_null() {
        panic!("allocation failed");
    }

    unsafe {
        core::ptr::copy_nonoverlapping(head.as_ptr(), tailed, head.len());
        *tailed.add(head.len()) = 0;
    };

    core::result::Result::Ok(A::Ok::from_raw(tailed as *mut u8))
}
