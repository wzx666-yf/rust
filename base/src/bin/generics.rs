// 引入 std::ops::Add<Output = T>才能使用 + 操作符,否则报错 —————这是特征
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// 引入特征std::cmp::PartialOrd才能比较，否则报错
fn largst<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 显示指定泛型的类型参数
use std::fmt::Display;

fn create_and_print<T>()
where
    T: From<i32> + Display,
{
    let a: T = 100.into(); // 创建了类型为 T 的变量 a，它的初始值由 100 转换而来
    println!("{}", a);
}

//结构体使用泛型
// 同一类型
struct Point<T> {
    x: T,
    y: T,
}

//不同类型
struct Point2<T, U> {
    x: T,
    y: U,
}

// 枚举中使用泛型
#[warn(dead_code)]
enum Option<T> {
    Some(T),
    None,
}

#[warn(dead_code)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 方法上使用泛型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        // V，W是单独定义在方法mixup的泛型，T，U是结构体上的泛型，一个结构体泛型，一个是函数泛型，两者并不冲突
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

//为具体的泛型类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Const泛型 针对值的泛型
// 与composite_type中的array那里一起看
fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    //Debug该限制表明 T 可以用在 println!("{:?}", arr) 中，因为 {:?} 形式的格式化输出需要 arr 实现该特征
    println!("{:?}", arr);
}

fn display_array_const<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
    println!("{:?}", arr);
    println!("N = {}", N)
}
// const fn 常量函数
// 通常情况下，函数是在运行时被调用和执行的。const fn 我们可以在编译期执行这些函数，从而将计算结果直接嵌入到生成的代码中。
const fn add_1(a: usize, b: usize) -> usize {
    a + b
}
const RESULT: usize = add_1(1, 2);

struct Buffer<const N: usize> {
    data: [u8; N],
}
const fn compute_buffer_size(factor: usize) -> usize {
    factor * 1024
}

fn main() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i16: {}", add(2i16, 3i16));
    println!("add i32: {}", add(2i32, 3i32));

    let list = vec![34, 40, 23, 56, 100, 17];
    let result = largst(&list);
    println!("largest: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largst(&char_list);
    println!("largest: {}", result);

    //create_and_print() 报错，因为无法推断函数 `create_and_print` 的类型参数 `T` 的类型
    create_and_print::<i64>();
    create_and_print::<f64>();
    // create_and_print::<i8>(); 没有实现i32到i8的转换规则，所以报错

    let integer = Point { x: 5, y: 10 };
    println!("integer: {}, float: {}", integer.x, integer.y);
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer: {}, float: {}", integer.x, float.y);
    // let wont_work = Point { x: 5, y: 4.0 }; 报错，因为 x 和 y 不是同一种类型
    let wont_work = Point2 { x: 5, y: 4.1 };
    println!("integer: {}, float: {}", wont_work.x, wont_work.y);

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    let p1 = Point2 { x: 5, y: 10.1 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    use std::any::type_name;
    fn type_of<T>(_: &T) {
        println!("类型是：{}", type_name::<T>())
    }

    let p1 = Point { x: 5.0, y: 10.0 };
    type_of(&p1);
    println!("p1.distance_from_origin() = {}", p1.distance_from_origin());

    let arr = [1, 2, 3, 4, 5];
    display_array(&arr);
    let arr = [1, 2];
    display_array(&arr);
    let arr1: [i32; 3] = [1, 2, 3];
    display_array_const(&arr1);
    let arr1: [i32; 2] = [1, 2];
    display_array_const(&arr1);
    println!("The result is:{}", RESULT);
    
    // 缓冲区的大小在编译期就被确定下来，避免了运行时的计算开销
    const SIZE: usize = compute_buffer_size(4);
    let buffer = Buffer::<SIZE> { data: [0; SIZE] };
    println!("buffer size: {}", buffer.data.len());
}
