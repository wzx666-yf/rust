//字符串和切片
fn string_slice() {
    let my_name = "Pascal";
    //greet(my_name); //报错，因为greet函数的参数是String类型，而my_name是&str类型
    greet(my_name.to_string());

    fn greet(name: String) {
        println!("Hello, {}!", name);
    }

    //slice切片
    let s = String::from("hello world");
    let hello = &s[0..5]; //左闭右开
    let world = &s[6..11];
    println!("hello = {} ,world = {}", hello, world);

    let s = String::from("hello");
    let slice = &s[0..2];
    println!("slice = {}", slice);
    let slice = &s[..2];
    println!("slice = {}", slice);

    let len = s.len();
    let slice = &s[3..len];
    println!("slice = {}", slice);
    let slice = &s[3..];
    println!("slice = {}", slice);

    let slice = &s[0..len];
    println!("slice = {}", slice);
    let slice = &s[..];
    println!("slice = {}", slice);

    let mut s = String::from("hello world");
    let word = first_word(&s);
    //s.clear(); //错误 因为 clear 需要清空改变 String，因此它需要一个可变借用
    //已经有了可变借用时，就无法再拥有不可变的借用
    println!("the first word is: {}", word);
    fn first_word(s: &String) -> &str {
        &s[0..1]
    }

    //clear
    let mut s = String::from("hello world");

    s.clear();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
    assert_eq!(s.capacity(), 11);

    //数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    //字符串   Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，
    // 字符串字面量是切片
    //但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)
    //&str转String 需要使用 to_string() 方法或者String::from("")
    //String转&str
    let s = String::from("hello world");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
    fn say_hello(s: &str) {
        println!("{}", s);
    }

    let h = String::from("中国人");
    println!("len = {}", h.len()); //九个字节，因为一个汉字占三个字节
                                   //因此h[0]会报错
                                   //println!("h[0] = {}", h[0]);
                                   //let s = &h[0..2];
                                   //println!("s = {}", s); 错误
    let i = String::from("नमस्ते");
    println!("len = {}", i.len());

    //操作字符串
    //追加
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s = {}", s);
    s.push('!');
    println!("s = {}", s);

    //插入
    let mut s = String::from("Hello Rust");
    s.insert(5, ',');
    println!("插入字符insert（） -> {}", s);
    s.insert_str(6, " world");
    println!("插入字符串insert_str（） -> {}", s);

    //替换 都是返回一个新的字符串，而不是操作原有的字符串
    //replace 替换所有匹配的字符串
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "C++");
    println!("替换replace -> {}", new_string_replace);

    //replacen 第三个参数为要替换的个数
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replacen("rust", "C++", 1);
    println!("替换replacen -> {}", new_string_replace);

    //replace_range 第一个参数是要替换字符串的范围
    let mut string_replace = String::from("I like rust. Learning rust is my favorite!");
    string_replace.replace_range(7..11, "C++");
    println!("替换replace_range -> {}", string_replace);

    //删除 有四种方法，分别为 remove、truncate、clear、pop
    // pop 删除并返回字符串的最后一个字符
    //但是存在返回值，其返回值是一个 Option 类型，如果字符串为空，则返回 None。直接操作原来的字符串
    let mut s = String::from("hello 中国！");
    let p1 = s.pop();
    let p2 = s.pop();
    //dbg!(p1);
    //dbg!(p2);
    //dbg!(s);
    println!("p1 = {} , p2 = {}, s = {}", p1.unwrap(), p2.unwrap(), s);

    //remove 删除指定位置的字符，返回被删除的字符
    //按照字节来处理字符串
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    //string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    string_remove.remove(3);
    println!("string_remove = {}", string_remove);

    //truncate 删除字符串从指定位置开始到结尾的全部字符，但是不会返回被删除的字符，按照字节来处理字符串
    let mut string_truncate = String::from("测试truncate方法");
    println!(
        "string_truncate 占 {} 个字节",
        size_of_val(string_truncate.as_str())
    );
    string_truncate.truncate(3);
    println!("string_truncate = {}", string_truncate);

    //clear 清空字符串，删除所有字符，操作原来的字符串
    let mut string_clear = String::from("测试clear方法");
    string_clear.clear();
    println!("string_clear = {}", string_clear);

    //连接
    //（1）使用 + 或 += 连接字符串
    // 要求右边的参数必须为字符串的切片引用（Slice）类型
    // + 是返回一个新的字符串，所以变量声明可以不需要 mut 关键字修饰
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2; //&s2会自动解引用为&str
                       // println!("s1 = {}", s1); 报错
    println!("s3 = {}", s3);
    let mut s3 = s3 + "!";
    s3 += "!!!";
    println!("s3 = {}", s3);

    //以下也是合法的
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // String = String + &str + &str + &str + &str
    let _s = s1 + "-" + &s2 + "-" + &s3;

    //add()方法定义  fn add(self, s: &str) -> String
    //s1 这个变量通过调用 add() 方法后，所有权被转移到 add() 方法里面， add() 方法调用后就被释放了，同时 s1 也被释放了。

    // （2）使用 format! ,适用于String和&str
    let s1 = String::from("Hello, ");
    let s2 = "world";
    let s3 = format!("{}{}", s1, s2);
    println!("s3 = {}", s3);

    //字符串转义
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中包含 # 号，可以在开头和结尾加多个 # 号，最多加255个，只需保证与字符串中连续 # 号的个数不超过开头和结尾的 # 号的个数即可
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    //以Unicode字符的方式遍历字符串，最好的方式是使用chars方法
    for c in "Зд".chars() {
        println!("{}", c);
    }
    // 以字节的方式遍历字符串
    for b in "Зд".bytes() {
        println!("{}", b);
    }
}

// 元组
fn tuple() {
    let tuple: (i32, f64, &str) = (500, 6.4, "hello");

    let (x, y, z) = tuple;
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!(
        "tuple.0 = {}, tuple.1 = {}, tuple.2 = {}",
        tuple.0, tuple.1, tuple.2
    );
    let _tuple2 = tuple;
    println!("{:?}", tuple);
    // 对于 i32 和 f64，它们实现了 Copy 特性，因此会进行复制。
    // 对于 &str，它是一个引用类型，赋值时会复制引用，引用本身不拥有数据的所有权。
}

// 结构体
fn struct_() {
    // 定义结构体
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    //创建结构体实例
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };

    //访问结构体字段
    user1.email = String::from("another@example.com");

    fn build_user(email: String, username: String) -> User {
        // 当函数参数和结构体字段同名时，可以直接使用缩略的方式进行初始化
        User {
            // email: email,
            // username: username,
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // let user2 = User{
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    //结构体更新语法
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    //user1 的部分字段所有权被转移到 user2 中：username 字段发生了所有权转移，作为结果，user1 无法再被使用。
    println!("{}", user1.active);
    // println!("{:?}", user1) 报错

    #[derive(Debug)]
    //使用 #[derive(Debug)] 对结构体进行了标记，这样才能使用 println!("{:?}", s); 的方式对其进行打印输出
    struct File {
        name: String,
        data: Vec<u8>,
    }

    let f1 = File {
        name: String::from("log.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_data_len = &f1.data.len();
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_data_len);

    // 元组结构体
    //元组结构体在你希望有一个整体名称，但是又不关心里面字段的名称时将非常有用
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // 单元结构体
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
    // 不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    // impl SomeTrait for AlwaysEqual {}

    // 想在结构体中使用一个引用，就必须加上生命周期，否则就会报错
    // struct User {
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    //     active: bool,
    // }
    //
    //
    // let user1 = User {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };

    //dbg!
    /*
        它会拿走表达式的所有权，然后打印出相应的文件名、行号等 debug 信息，
        当然还有我们需要的表达式的求值结果。除此之外，它最终还会把表达式值的所有权返回！
    */
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
}

// 枚举
fn enum_() {
    //枚举类型是一个类型，它会包含所有可能的枚举成员，而枚举值是该类型中的具体某个成员的实例。
    #[derive(Debug)]
    enum PokerSuit {
        Spades,
        Hearts,
        Diamonds,
        Clubs,
    }

    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);
    fn print_suit(card: PokerSuit) {
        // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
        println!("{:?}", card);
    }

    // struct PokerCard{
    //     suit: PokerSuit,
    //     value: u8,
    // }
    //
    // let _c1 = PokerCard{
    //     suit: PokerSuit::Hearts,
    //     value: 1,
    // };
    //
    // let _c2 = PokerCard{
    //     suit: PokerSuit::Diamonds,
    //     value: 12,
    // };

    //上面简洁的写法
    enum PokerCard {
        Hearts(u8),
        Diamonds(u8),
        Spades(u8),
        Clubs(u8),
        Joker(String), //可以持有不同的数据类型
    }
    let _c1 = PokerCard::Hearts(1);
    let _c2 = PokerCard::Diamonds(12);

    enum Message {
        Quit,
        Move { x: i32, y: i32 }, //匿名结构体
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let _m1 = Message::Quit;
    let _m2 = Message::Move { x: 1, y: 2 };
    let _m3 = Message::Write(String::from("hello"));
    let _m4 = Message::ChangeColor(255, 255, 255);

    //用于处理空值
    enum Option<T> {
        Some(T),
        None,
    }
}

//数组
fn array() {
    //Array长度固定，Vector可动态增长
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // 5个3
    println!("{:?}", c);
    let first = a[0];
    let second = a[1];
    println!("first = {}， second = {}", first, second);

    use std::io;
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = b[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

    // let array = [String::from("hello"); 8]; 这是错误的，因为[3; 5]底层是不断Copy出来的
    let array: [String; 8] = std::array::from_fn(|_| String::from("hello"));
    println!("{:#?}", array);

    //数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    //例子
    fn example(){
        let one = [1, 2, 3];
        let two : [u8; 3] = [1, 2, 3];
        let blank1 = [0; 3];
        let blank2 : [u8; 3] = [0; 3];
        let arrays : [[u8; 3]; 4] = [one, two, blank1, blank2];
        
        for a in &arrays {
            print!("{:?}: ", a);
            
            for n in a.iter(){
                print!("\t{} + 10 = {}", n, n + 10);
            }
            
            let mut sum =0;
            for i in 0..a.len(){
                sum += a[i];
            }
            println!("\t({:?} = {})", a, sum);
        }
    }
    
    example();

    fn display_array(a: [i32; 3]) {
        println!("{:?}", a);
    }
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);
    let arr: [i32; 2] = [1, 2];
    // display_array(arr); 报错，因为[i32; 3]和[i32; 2]是两个完全不同的类型
    // 只需要将改为不可变引用即可，fn display_array(a: &[i32])
    // display_array(&arr);
}

fn main() {
    string_slice();
    tuple();
    struct_();
    enum_();
    array();
}
