#[macro_export]
macro_rules! make_default_or_given {
    ($default:ty, $given:ty) => {
        macro_rules! default_or_given {
            () => {
                $default
            };
            ($t:ty) => {
                $t
            };
        }
    };
}
