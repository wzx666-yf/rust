use std::fmt::{Debug, Display, Formatter};

// 特征定义了一组可以被共享的行为，只要实现了特征，你就能使用这组行为。与接口类似
//如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！
pub trait Summary {
    fn summarize(&self) -> String;

    // 默认实现
    fn smary(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;

    // 默认实现允许调用相同特征中的其他方法
    fn summarize_default(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章是：{}, by {}", self.title, self.author)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博：{}", self.username, self.content)
    }

    fn smary(&self) -> String {
        format!("{}发表了微博：{}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 使用特征作为函数参数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 特征约束
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 可以都是post也可以都是weibo，也可以一个weibo一个post
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("不同的类型：（weibo）{}", item1.summarize());
    println!("不同的类型：（post）{}", item2.summarize());
}

// 都是weibo或者都是post，不能一个是weibo一个是post会报错
pub fn notify4<T: Summary>(item1: &T, item2: &T) {
    println!("相同的类型：（weibo）{}", item1.summarize());
    println!("相同的类型：（weibo）{}", item2.summarize());
}

// 多重约束
pub fn notify5<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item);
}
// 或者
// pub fn notify5(item: &(impl Summary + Display)){
//     println!("Breaking news! {}", item.summarize());
// }

// where约束
fn some_function<T, U>() -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // 只有 T 同时实现了 Display + PartialOrd 的 Pair<T> 才可以拥有此方法。
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 函数返回中的impl Trait
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("Rust"),
        content: String::from("Rust is awesome"),
    }
}

// 错误代码，只能有一个具体的类型
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         Post {
//             title: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Weibo {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//         }
//     }
// }

// 解决上面的办法————特征对象
// 特征对象的限制
// 方法的返回类型不能是Self
// 方法没有如何泛型类型
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", self)
    }
}

// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) {
    x.draw();
}

fn draw2(x: &dyn Draw) {
    x.draw();
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) -> String {
        format!(
            "Button: label: {}, width: {}, height: {}",
            self.label, self.width, self.height
        )
    }
}

impl Draw for SelectBox {
    fn draw(&self) -> String {
        format!(
            "SelectBox: width: {}, height: {}, options: {:?}",
            self.width, self.height, self.options
        )
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            let s = component.draw();
            println!("{}", s);
        }
    }
}

// Vec<T>中的每一个袁术都是同类型的，例如每个元素必须都是Button类型或者全是SelectBox类型
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
//
// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// example 1
use std::ops::Add;

#[derive(Debug)]
struct Point<T: Add<Output = T>> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>; // type定义别名

    fn add(self, p: Self) -> Self::Output {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// example 2
use std::fmt;

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<{}, ({})>", self.name, self.state) // write！格式化写入
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

// 关联类型
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
// 其中Item就是关联类型，可以自定义类型
// Self 用来指代当前调用者的具体类型，那么 Self::Item 就用来指代该类型实现中定义的 Item 类型
// trait Container<A, B>{
//     fn contains(&self,a: A,b: B) -> bool;
// }
//
// fn difference<A, B, C>(container: &C) -> i32
// where
//     C: Container<A, B>{
//     todo!()
// }
// 与
trait Container {
    type A: Display; //关联类型可以被其他特征进行约束
    type B;
    fn contains(&self, a: Self::A, b: Self::B) -> bool;
}
fn difference<C: Container>(container: &C) -> i32 {
    todo!()
}

// 默认泛型类型参数
// 给 RHS 一个默认值，也就是当用户不指定 RHS 时，默认使用两个同样类型的值进行相加
trait Add2<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

#[derive(Debug, PartialEq)]
struct Point2 {
    x: i32,
    y: i32,
}
impl Add for Point2 {
    type Output = Point2;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

// 默认类型参数主要用于
// 减少实现的样板代码
// 扩展类型但是无需大幅修改现有代码

// 调用同名的方法
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}

struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// 若没有self
trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// 在外部类型上实现外部特征（newtype）
// 前面提到，特征或者类型必需至少有一个是本地的，才能在此类型上定义特征。
// 这里提供一个办法来绕过孤儿规则，那就是使用newtype 模式，
// 简而言之：就是为一个元组结构体创建新类型。该元组结构体封装有一个字段，该字段就是希望实现特征的具体类型。
struct Wrapper(Vec<String>);
impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let post = Post {
        title: String::from("Rust"),
        author: String::from("Rust"),
        content: String::from("Rust is awesome"),
    };
    println!("{}", post.summarize());
    let weibo = Weibo {
        username: String::from("Rust"),
        content: String::from("Rust is awesome"),
    };
    println!("{}", weibo.summarize());

    println!("{}", post.smary());
    println!("{}", weibo.smary());

    println!("1 new weibo: {}", weibo.summarize_default());

    notify(&weibo);
    notify2(&post);
    notify3(&weibo, &post);
    notify4(&weibo, &weibo);

    let pair = Pair::new(1, 2);
    pair.cmp_display();

    let p = returns_summarizable();
    println!("{}", p.summarize());

    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.2f32,
        y: 2.2f32,
    };
    println!("{:?}", add(p1, p2));

    let f6 = File::new("f6.txt");
    println!("{:?}", f6);
    println!("{}", f6);

    let x = 1.1f64;
    let y = 8u8;

    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    draw1(Box::new(x));
    draw1(Box::new(y));

    // 基于 y 的值创建一个 Box<u8> 类型的智能指针，指针指向的数据被放置在了堆上
    draw2(&x);
    draw2(&y);

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    assert_eq!(
        Point2 { x: 1, y: 2 } + Point2 { x: 2, y: 3 },
        Point2 { x: 3, y: 5 }
    );

    let person = Human;
    person.fly(); // 默认调用 Human::fly
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby dog is called a {}", Animal::baby_name()) 错误,因为实现 Animal 特征的类型可能有很多
    // 完全限定语法
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("{}", w);
}
