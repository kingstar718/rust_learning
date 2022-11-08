mod collect_demo;
mod test;

pub use collect_demo::create_vec;
pub use collect_demo::read_vec;
pub use collect_demo::batch_use;
pub use collect_demo::trait_diff_vec;


mod hash_map_demo;

pub use hash_map_demo::create_hash_map;
pub use hash_map_demo::hash_map_life_time;
pub use hash_map_demo::get_from_hash_map;
pub use hash_map_demo::update_hash_map;