pub mod terminate;
pub use terminate::terminate;

crate::r#struct!(
    #[derive(Debug)]
    pub struct String {
        content: [char; 256],
    }
);

impl String {
    pub fn from_pointer(_: *const u8) -> Self {
        String {
            content: ['\0'; 256],
        }
    }
}
