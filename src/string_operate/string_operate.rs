// String 可变字符串的操作方法

// Push 追加
pub fn string_push() {
    let mut s = String::from("Hello ");
    s.push('r');
    println!("追加字符串 push() -> {}", s);

    s.push_str("ust!");
    println!("追加字符串 push_str() -> {}", s);
}

pub fn string_insert() {
    let mut s = String::from("hello rust");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);
}

pub fn string_replace() {
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    // &str、String 替换所有匹配到的字符串，返回一个新的字符串
    let new_string_replace = string_replace.replace("rust", "RUST");
    println!("{}", new_string_replace);

    // &str、String 替换的个数
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    println!("{}", new_string_replacen);

    // 仅适用于String
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    println!("{}", string_replace_range);
}

pub fn string_delete() {
    // 直接操作原来的字符串
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    // 删除并返回字符串中指定位置的字符
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字符",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下边的代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);

    // truncate 删除字符串中指定位置开始到结束的全部字符，直接操作原来的字符串
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate); // 输出：测，测占3个字符，后边的全部删除

    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);
}

pub fn string_concatenate() {
    // 适用 + 或 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice）类型
    // + 操作符，相当于调用std::string标准库中的add()方法
    // + += 都是返回一个新的字符串，所以变量声明可以不需要mut关键字修饰
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let mut result = string_append + &string_rust;
    result += "!!!";
    println!("连接字符串 + -> {}", result);

    // add()
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    // 下边会报错
    // println!("{}", s1);

    // format!
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
}

pub fn string_utf_8() {
    for c in "中国人".chars() {
        println!("{}", c);
    }
    for b in "中国人".bytes() {
        println!("{}", b);
    }
}