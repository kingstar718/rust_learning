mod string_demo;
mod string_operate;

fn main() {
    str_demo();
    str_operate();
}

fn str_demo() {
    println!("==> 字符串相关 ==>");
    string_demo::slice_demo_1();
    string_demo::string_slice_1();
    string_demo::str_and_string();
    string_demo::string_index();
    string_demo::string_slice_2();
    println!("==> 字符串相关 ==>");
}

fn str_operate() {
    println!("==> 字符串操作相关 ==>");
    string_operate::string_push();
    string_operate::string_insert();
    string_operate::string_replace();
    string_operate::string_delete();
    string_operate::string_concatenate();
    println!("==> 字符串操作相关 ==>");
}
