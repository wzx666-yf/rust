#![allow(warnings)]

use std::collections::HashMap;

// 动态数组Vector的使用
// 动态数组只能存储相同类型的元素，
// 如果想存储不同类型的元素，可以使用之前讲过的枚举类型或者特征对象。
fn vector() {
    let v: Vec<i32> = Vec::new();

    let mut v = Vec::new();
    v.push(1); //向数组尾部添加元素

    //如果预先知道要存储的元素个数，可以使用 Vec::with_capacity(capacity) 创建动态数组，
    // 这样可以避免因为插入大量新数据导致频繁的内存分配和拷贝，提升性能
    let v = vec![1, 2, 3]; //在创建同时给予初始化值

    // 读取元素 1、下标访问 2、get方法 => 返回的是Option类型
    let third = &v[2];
    println!("The third element is {}", *third);

    match v.get(2) {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // let does_not_exist = &v[100]; 越界报错
    let does_not_exist = v.get(100);
    println!("The does_not_exist is {does_not_exist:?}");

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    // println!("The first element is: {first}"); //报错，因为元素已经被移除

    // 遍历
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    // 存储不同类型的元素
    // 使用枚举实现
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let i = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];
    for ip in i {
        println!("{:?}", ip);
    }

    // 使用特征对象实现
    trait IpAddr2 {
        fn display(&self);
    }

    struct V4(String);
    impl IpAddr2 for V4 {
        fn display(&self) {
            println!("ipv4: {:?}", self.0);
        }
    }

    struct V6(String);
    impl IpAddr2 for V6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0);
        }
    }

    let i: Vec<Box<dyn IpAddr2>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
    for ip in i {
        ip.display();
    }

    // 初始化数组
    let v = vec![0; 3];
    let v_from = Vec::from([0, 0, 0]);
    assert_eq!(v, v_from);

    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]); //附加数据到v
    println!("Vector length: {}, capacity: {}", v.len(), v.capacity());

    v.reserve(100); // 调整容量，至少要有100的容量
    println!("Vector length: {}, capacity: {}", v.len(), v.capacity());

    v.shrink_to_fit(); // 调整容量，使其等于当前长度
    println!("Vector length: {}, capacity: {}", v.len(), v.capacity());

    // Vector常用方法示例
    let mut v = vec![1, 2, 3];
    assert!(!v.is_empty()); // 判断是否为空

    v.insert(2, 3); // 在指定位置插入元素，索引值不能大于v的长度 v: [1, 2, 3, 3]
    assert_eq!(v.remove(1), 2); //移除指定位置元素并返回 v: [1, 3, 3]
    assert_eq!(v.pop(), Some(3)); //移除最后一个元素并返回 v: [1, 3]
    assert_eq!(v.pop(), Some(3)); //v: [1]
    assert_eq!(v.pop(), Some(1)); //v: []
    assert_eq!(v.pop(), None); //记得pop方法返回的是Option枚举值
    v.clear(); // 清空v, v: []

    let mut v1 = [11, 22].to_vec(); //append操作会导致v1清空数据，增加可变声明
    v.append(&mut v1); // 将v1中的所有元素附加到v中，v1: []
    v.truncate(1); // 将v的长度截断为1,多余元素被删除,v: [11]
    v.retain(|x| *x > 10); // 保留x大于10的元素,即删除不满足条件的元素 v: [11]

    let mut v = vec![11, 22, 33, 44, 55];
    // 删除指定范围的元素，同时获取被删除的元素的迭代器，v: [11, 55]，m: [22, 33, 44]
    let mut m: Vec<_> = v.drain(1..=3).collect();

    let v2 = m.split_off(1); //指定索引处切分成两个 vec, m: [22], v2: [33, 44]

    let v = vec![11, 22, 33, 44, 55];
    let slice = &v[1..=3];
    assert_eq!(slice, &[22, 33, 44]);

    // Vector的排序
    // 稳定的排序 sort 和 sort_by，以及非稳定排序 sort_unstable 和 sort_unstable_by。
    // 整数数组的排序
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort_unstable();
    println!("{:?}", vec); // [1, 2, 5, 10, 15]

    // 浮点数数组的排序
    let mut vec = vec![1.1, 5.5, 10.0, 2.2, 15.0];
    // vec.sort_unstable(); 报错，在浮点数当中，存在一个 NAN 的值，这个值无法与其他的浮点数进行对比，
    // 因此，浮点数类型并没有实现全数值可比较 Ord 的特性，而是实现了部分可比较的特性 PartialOrd。
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", vec); // [1.1, 2.2, 5.5, 10.0, 15.0]
    vec.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    println!("{:?}", vec); // [15.0, 10.0, 5.5, 2.2, 1.1]

    // 结构体数组的排序
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: String, age: u32) -> Self {
            Person { name, age }
        }
    }

    let mut people = vec![
        Person::new(String::from("Alice"), 30),
        Person::new(String::from("Bob"), 25),
        Person::new(String::from("Charlie"), 40),
    ];
    people.sort_by(|a, b| a.age.cmp(&b.age));
    println!("{:?}", people);

    // 实现Ord特性
    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct Person2 {
        name: String,
        age: u32,
    }

    impl Person2 {
        fn new(name: String, age: u32) -> Self {
            Person2 { name, age }
        }
    }

    let mut people = vec![
        Person2::new(String::from("Alice"), 30),
        Person2::new(String::from("Bob"), 25),
        Person2::new(String::from("Charlie"), 40),
        Person2::new("Bob".to_string(), 60),
        Person2::new("Alice".to_string(), 24),
    ];
    people.sort_by(|a, b| a.age.cmp(&b.age)); //derive 的默认实现会依据属性的顺序依次进行比较
    println!("{:?}", people);
} //当 Vector 被删除后，它内部存储的所有内容也会随之被删除。

// KV存储HashMap
fn KV() {
    // 创建一个HashMap，用于存储宝石种类和对应的数量
    let mut my_gems = HashMap::new();

    my_gems.insert("diamond", 10);
    my_gems.insert("emerald", 12);
    my_gems.insert("ruby", 8);

    println!("My gems: {:#?}", my_gems);

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("日本队".to_string(), 90),
        ("韩国队".to_string(), 80),
    ];

    // 方法一从Vec创建HashMap
    // let mut teams_map = HashMap::new();
    // for team in &teams_list {
    //     teams_map.insert(&team.0, team.1);
    // }

    // 方法二
    let mut teams_map: HashMap<_, _> = teams_list.into_iter().collect();
    //into_iter 方法将列表转为迭代器，接着通过 collect 进行收集

    println!("Teams map: {:?}", teams_map);

    // 查询HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // let score = scores.get(&team_name).unwrap();
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score of {} is {}", team_name, score);

    for (key, vlaue) in &scores {
        println!("{}: {}", key, vlaue);
    }

    // 更新HashMap的值
    // 覆盖已有的值
    let old = scores.insert(String::from("Blue"), 25); //insert如果映射确实存在此键，则更新该值并返回旧值。
    assert_eq!(old, Some(10));
    // 查询新插入的值
    assert_eq!(scores.get(&String::from("Blue")), Some(&25));
    // 查询Red对应的值，若不存在插入新值
    let red = scores.entry(String::from("Red")).or_insert(50);
    assert_eq!(*red, 50); // 不存在，插入50
    let red = scores.entry("Red".to_string()).or_insert(5);
    assert_eq!(*red, 50); // 已经存在，因此50没有插入
    
    // 统计文本词语出现的个数
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
fn main() {
    vector();
    KV();
}
