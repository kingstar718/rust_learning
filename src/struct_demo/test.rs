use crate::struct_demo::{create_file, create_user, struct_display};

#[test]
pub fn create_user_test() {
    create_user();
}

#[test]
pub fn create_file_test() {
    create_file();
}

#[test]
pub fn struct_display_test() {
    struct_display();
}