//match 和 if let
fn match_ex() {
    #[allow(dead_code)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dire = Direction::South;

    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => println!("North or South"),
        _ => println!("West"),
    };

    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    println!("{}", value_in_cents(Coin::Penny));

    //使用 match 表达式赋值
    #[allow(dead_code)]
    enum IpAddr {
        Ipv4,
        Ipv6,
    }

    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);

    //模式绑定
    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
    }

    let actions = [
        Action::Say("Hello".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("Say: {}", s);
            }
            Action::MoveTo(x, y) => {
                println!("print from (0,0) Move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!("Change color to RGB({}, {}, 0)", r, g,);
            }
        }
    }

    //_通配符
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), //_通配符
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Direction2 {
        East,
        West,
        North,
        South,
    }
    let dire = Direction2::East;
    match dire {
        Direction2::North => println!("North"),
        other => println!("{:?}", other),
    };

    //if let匹配
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    //当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // matches!宏
    // 它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 true or false
    #[derive(Debug)]
    enum MyEnum {
        Foo,
        Bar,
    }
    let mut v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    // 只保留 MyEnum::Foo
    // v.iter().filter(|x| x == MyEnum::Foo) 错误，无法将x直接与一个枚举成员进行比较
    v = v.into_iter().filter(|x| matches!(x, MyEnum::Foo)).collect();
    println!("{:?}", v);

    let foo = 'f';
    println!("{}", matches!(foo, 'A'..='Z' | 'a'..='z'));

    //Some() 的作用是包裹一个具体的值，表示该值存在。这在处理可能缺失的值时非常有用，可以避免空指针异常等问题。
    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));

    //变量遮蔽
    let age = Some(18);
    println!("age is {:?}", age);
    if let Some(age) = age {
        println!("You are {} years old", age);
    }
    println!("age is {:?}", age);

    //while let条件循环
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let arr: &[u16] = &[114, 514];
}

// 解构 Option
fn option_destruct() {
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // 匹配 Option<T>
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        Some(_) => println!("not three"),
        None => (),
    }

    fn add_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
    }
    let five = Some(5);
    let six = add_one(five);
    let none = add_one(None);
    println!("{:?}, {:?}", six, none);
}

//解构
fn destructure() {
    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!(),
    }

    //解构枚举
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }

    //解构结构体和元组
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);

    //解构数组
    //定长数组
    let arr: [u16; 2] = [114, 514];
    let [x, y] = arr;
    println!("x: {}, y: {}", x, y);

    //不定长数组
    let arr: &[u16] = &[114, 514]; // &[u16] 表示对 u16 类型元素的不可变切片引用 , &[...] 创建数组的切片引用（数组 [114, 514] 的类型是 [u16; 2]，通过 & 转换为切片）
    if let [x, ..] = arr {
        //匹配至少一个元素的切片,  x 绑定为第一个元素的引用 &u16, ..表示忽略剩余元素（支持任意长度）
        assert_eq!(x, &114);
    }
    if let &[.., y] = arr {
        //尾部绑定
        assert_eq!(y, 514); //y为u16类型
    }
    let arr: &[u16] = &[];
    assert!(matches!(arr, [..])); // 匹配任意长度（包括空）
    assert!(!matches!(arr, [x, ..])); // 匹配至少一个元素的切片
}

// 忽略模式中的值
fn ignore_pattern() {
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(50);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            //不关心里面的值，只关心两个元素的类型没有None就行
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    let s = Some(String::from("blue"));
    // if let Some(_s) = s {    已经将s的所有权转移给_s, 所以这里会报错
    //    println!("found a string");
    // }
    if let Some(_) = s {
        println!("found a string");
    }
    println!("s is {:?}", s);
    
    struct Point{
        x: i32,
        y: i32,
        z: i32,
    }
    
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin { 
        Point { x, .. } => println!("x is {}", x),
    }
    
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .. ,last) => println!("first is {}, last is {}", first, last),
    }
    
    // 使用..必须非歧义 example
    // match numbers {
    //     (.., second, ..) => println!("second is {}", second),
    // }
}

// 匹配守卫提供的额外条件
fn match_graud(){
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    
    let x =Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n), //这里的y是外部的y，并不如上面的一样引入新变量
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
    
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

// @绑定
// 当你既想要限定分支范围，又想要使用分支的变量时，就可以用 @ 来绑定到一个新的变量上，实现想要的功能。
fn bind(){
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    
    // 可以发现，第一个id和第二分支id与第三个分支id类型不一样
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => //@ 将匹配成功的 id 值绑定到变量 id_variable  
        {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => { //此模式仅检查 id 是否在 [10, 12] 区间内，但未将 id 值绑定到任何变量
            println!("Found an id in another range"  )
        }
        Message::Hello { id } => println!("Found some other id: {}", id), // id 作为字段名直接绑定值到同名变量（等价于 id: id @ _）
    }
    
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p @ Point { x: px, y: py } = Point { x: 0, y: 7 };
    println!("x: {}, y: {}", px, py);
    println!("Point p is {:?}", p);
    
    let point = Point { x: 0, y: 7 };
    if let p @ Point{x: 0, y} = point {
        println!("x coordinate is zero at y: {}  in {:?}", y, p);
    } else {
        println!("x was not 0");
    }
    
    match 1{
        num @ (1 | 2) => {
            println!("num is {}", num);
        }
        _ => {
            println!("num is not 1 or 2");
        }
    }
}

fn main() {
    match_ex();
    option_destruct();
    destructure();
    ignore_pattern();
    match_graud();
    bind();
}
