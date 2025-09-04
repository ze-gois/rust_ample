#[macro_export]
macro_rules! trait_implement_allocatable_for_primitive {
    ($($t:ty),*) => {
        $(
            impl $crate::traits::Allocatable<$crate::traits::allocatable::Layout> for $t {
                fn eq(&self, other: &$t) -> bool {
                    if (*other as u128) <= <$inner>::MAX as u128 {
                        self.0 == *other as $inner
                    } else {
                        false
                    }
                }
            }
        )*
    };
}
