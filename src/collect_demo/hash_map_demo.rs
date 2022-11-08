use std::collections::HashMap;

pub fn create_hash_map() {
    let mut my_gems = HashMap::new();

    // 将宝石类型和对应的数量写入表中
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);

    // 定长
    // let mut _know_cap_map = HashMap::with_capacity(3);

    // list 转 map
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];
    let team_map: HashMap<_, _> = teams_list.into_iter().collect();
    println!("{:?}", team_map);
}

// 若类型实现 Copy 特征，该类型会被复制进 HashMap，因此无所谓所有权
// 若没实现 Copy 特征，所有权将被转移给 HashMap 中
pub fn hash_map_life_time() {
    let name = String::from("Sun face");
    let age = 18;
    let mut human_boy = HashMap::new();
    human_boy.insert(name, age);

    // name的所有权已被转移到hash map中
    // println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    println!("还有，他的真实年龄远远不止{}岁", age);

    // 如果你使用引用类型放入 HashMap 中，请确保该引用的生命周期至少跟 HashMap 活得一样久
    let name2 = String::from("Sun face");
    let mut human_boy2 = HashMap::new();
    human_boy2.insert(&name2, age);
    std::mem::drop(name2);
    // 通过 drop 函数手动将 name 字符串从内存中移除
    // println!("因为过于无耻，{:?}已经被除名", handsome_boys2);
    println!("还有，他的真实年龄远远不止{}岁", age);
}


pub fn get_from_hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);
    println!("{:?}", score);
    match score {
        Some(a) => println!("{}", a),
        None => println!("无数据"),
    }

    // 循环
    for (k, v) in &scores {
        println!("{} {}", k, v);
    }
}


pub fn update_hash_map() {
    let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入
}