use crate::tuple_demo::{attach_tuple, match_tuple, use_tuple_return};

#[test]
pub fn match_tuple_test() {
    match_tuple();
}

#[test]
pub fn attach_tuple_test() {
    attach_tuple();
}

#[test]
pub fn use_tuple_return_test() {
    use_tuple_return();
}