// 切片 slice: 对集合的部分引用
pub fn slice_demo_1() {

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello world 0..5: {}", hello);
    println!("hello world 6..11: {}", world);

    // 其他类型的切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

}

pub fn string_slice_1() {
    // 字符串字面量的类型是切片
    let s = "Hello World";
    println!("字符串字面量: {}", s)
}

// rust的字符是unicode类型，每个字符占据4个字节内存空间
// 字符串中，是utf-8编码，字符串中所占的字节是变化的（1-4）
// str是硬编码进可执行文件也无法被修改，通常以引用类型出现：&str
// String是一个可增长、可改变且具有所有权的utf-8编码字符串
// &str、String类型都是utf-8编码
pub fn str_and_string() {
    let string_1 = String::from("hello world");
    let string_2 = "hello world".to_string();
    let str_1 = &string_1;
    let str_2 = &string_1[..];
    let str_3 = string_1.as_str();
    println!("{}", str_1);
    println!("{}", str_2);
    println!("{}", str_3);

    say_hello(str_1);

    fn say_hello(s: &str) {
        println!("{}", s);
    }
}


// 字符串索引
pub fn string_index() {
    let s1 = String::from("hello");
    // `String` cannot be indexed by `{integer}`
    // let h = s1[0];
    let s2 = String::from("中国人");
    // 大部分汉字在utf-8是3个字节，所以长度是9
    // 对于String类型来说，取索引无法保证是O(1)操作，可能要从0开始去遍历
    println!("中国人: {}", s2.len())
}


pub fn string_slice_2() {
    // 字符串切片是非常危险的操作
    // 因为切片索引是通过字节来进行，但是字符串又是utf-8编码，无法保证索引的字节刚好落在字符的边界上
    let hello = "中国人";
    // 以下程序会崩溃
    // let s = &hello[0..2];
    println!("中国人 [0..3]: {}", &hello[0..3]);
}

