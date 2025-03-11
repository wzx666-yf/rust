/*
    所有权原则：
    1、Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
    2、一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
    3、当所有者（变量）离开作用域范围时，这个值将被丢弃(drop)
*/

fn take_ownership(s: String) {
    println!("Received: {}", s);
} // s 被自动释放

fn make_copy(x: i32) {
    // x进入作用域
    println!("Received: {}", x);
} //这里x移出作用域，不会有特殊操作

// String 是动态字符串类型，&str是字符串字面值是不可变的
fn string_ex() {
    let mut s = String::from("hello");
    s.push_str(", world!"); //push_str()在字符串后追加字面值
    println!("{}", s);

    // 这只是简单的拷贝，因为整数是固定大小的简单值保存在栈中，无需在堆中分配内存
    //浅拷贝
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let _s2 = s1;
    // println!("s1 = {}", s1);
    /*
        String 类型是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量共同组成，
        其中堆指针是最重要的，它指向了真实存储字符串内容的堆内存，至于长度和容量，
        容量是堆内存分配空间的大小，长度是目前已经使用的大小。
        因此当 s1 被赋予 s2 后，Rust 认为 s1 不再有效，因此也无需在 s1 离开作用域后 drop 任何东西，
        这就是把所有权从 s1 转移给了 s2，s1 在被赋予 s2 后就马上失效了。
    */
}

// 克隆(深拷贝)
// 原变量保留，复制栈数据和推数据。而移动原变量失效，只复制栈数据。
fn clone_ex() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

//引用与解引用
fn reference_ex() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // assert_eq!(5, y)会发生错误，因为y是引用，需要解引用

    //不可变引用
    let s1 = String::from("hello");
    // & 符号既是引用，它们允许你使用值，但是不获取所有权
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    //可变引用
    let mut s = String::from("hello");
    change(&mut s);
    println!("s = {}", s);
    //可变引用同时只能存在一个
    //同一作用域，特定数据只能有一个可变引用
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    let r1 = &mut s;
    println!("r1 = {}", r1);
    let r2 = &mut s;
    println!("r2 = {}", r2);

    {
        let _r3 = &mut s;
    } //这里r3被释放，r2可以继续使用
    
    //可变引用与不可变引用不能同时存在
    let mut a1 = String::from("hello");
    
    let a2 = &a1;//没问题
    let a3 = &a1;//没问题
    //let a4 = &mut a1;//这里会报错，因为可变引用不能同时存在
    //println!("a1 = {}, a2 = {}, a3 = {}, a4 = {}", a1, a2, a3, a4);
    println!("a1 = {}, a2 = {}, a3 = {}", a1, a2, a3);
    
    
    //悬垂引用 (Rust不会允许这个事发生)
    //意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在，其指向的内存可能不存在任何值或已被其它变量重新使用。
    // let reference_to_nothing = dangle();
    // fn dangle() -> &String { // dangle 返回一个字符串的引用
    //     let s = String::from("hello"); // s 是一个新字符串
    //     &s // 返回字符串 s 的引用
    // } // 这里 s 离开作用域并被丢弃。其内存被释放。
}

fn calculate_length(s: &String) -> usize {
    // s 进入作用域
    s.len() // 返回 s 的长度
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn main() {
    // 所有权初始分配
    let s1 = String::from("hello"); // s1 进入作用域

    // s1 的所有权转移到函数，此后 s1 不可用
    take_ownership(s1);
    // println!("{}", s1); // 这里会报错！

    let x = 5; // x 进入作用域
    make_copy(x); // x 进入作用域，并且 Copy，后买你可继续使用x
    println!("x = {}", x);

    string_ex();
    clone_ex();
    reference_ex();
}
/*
    如下是一些 Copy 的类型：
        所有整数类型，比如 u32
        布尔类型，bool，它的值是 true 和 false
        所有浮点数类型，比如 f64
        字符类型，char
        元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
        不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意：可变引用 &mut T 是不可以 Copy的
*/
