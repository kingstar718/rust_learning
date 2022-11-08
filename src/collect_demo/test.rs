use crate::collect_demo::{batch_use, create_hash_map, create_vec, get_from_hash_map, hash_map_life_time, read_vec, trait_diff_vec, update_hash_map};

#[test]
pub fn create_vec_test() {
    create_vec();
}

#[test]
pub fn read_vec_test() {
    read_vec();
}

#[test]
pub fn batch_use_test() {
    batch_use();
}

#[test]
pub fn trait_diff_vec_test() {
    trait_diff_vec();
}

#[test]
pub fn create_hash_map_test() {
    create_hash_map();
}

#[test]
pub fn hash_map_life_time_test() {
    hash_map_life_time();
}

#[test]
pub fn get_from_hash_map_test() {
    get_from_hash_map();
}

#[test]
pub fn update_hash_map_test() {
    update_hash_map();
}
