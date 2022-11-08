use std::fs::File;
use std::{fs, io};
use std::io::Read;


// ? 的传播
// 可以自动进行类型提升（转换）
pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 更短的写法
pub fn short_handle() -> Result<String, io::Error>{
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Result 通过 ? 返回错误，那么 Option 就通过 ? 返回 None
pub fn first(arr: &[i32]) -> Option<i32>{
    let v = arr.get(0)?;
    Some(*v)
}

// 在链式调用中使用 ? 提前返回 None 的用法， .next 方法返回的是 Option 类型：
// 如果返回 Some(&str)，那么继续调用 chars 方法,如果返回 None，则直接从整个函数中返回 None，不再继续进行链式调用。
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// 这段代码无法通过编译，切记：? 操作符需要一个变量来承载正确的值，这个函数只会返回 Some(&i32) 或者 None，
// 只有错误值能直接返回，正确的值不行，所以如果数组中存在 0 号元素，那么函数第二行使用 ? 后的返回类型为 &i32 而不是 Some(&i32)。
// 因此 ? 只能用于以下形式：
// let v = xxx()?;
// xxx()?.yyy()?;

// fn first(arr: &[i32]) -> Option<&i32> {
//    arr.get(0)?
// }

pub fn try_demo() {
    //  `?`
    // let x = function_with_error()?; // 若返回 Err, 则立刻返回；若返回 Ok(255)，则将 x 的值设置为 255

    // `try!()`
    // let x = try!(function_with_error());
}

