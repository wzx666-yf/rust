use std::any::type_name;

//打印数据类型
fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}
fn typename() {
    let x = 42;
    let y = "Hello, world!";
    let z = 3.14;
    let w = true;

    print_type_of(&x);
    println!("{:?}", x);
    print_type_of(&y);
    print_type_of(&z);
    print_type_of(&w);

    let s: i64 = x.into();
    print_type_of(&s);
}

//整数溢出
fn yichu() {
    let a: u8 = 255;
    let b = a.wrapping_add(20); //使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理
    let c = a.checked_add(20); //如果使用 checked_* 方法时发生溢出，则返回 None 值
    let d = a.overflowing_add(20); //使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
    let e = a.saturating_add(20); //使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值
    println!("{}", b);
    println!("{:?}", c);
    println!("{:?}, {}, {}", d, d.0, d.1);
    println!("{}", e);
}

//浮点数
fn floattype() {
    let x = 2.0; //默认f64
    let y: f32 = 3.0;
    print_type_of(&x);
    print_type_of(&y);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2 = {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3 = {:x}", (abc.2).to_bits());

    println!("xyz (f64)");
    println!("   0.1 + 0.2 = {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3 = {:x}", (xyz.2).to_bits());

    assert!(abc.0 + abc.1 == abc.2);
    //assert!(xyz.0 + xyz.1 == xyz.2); //panic!

    //NaN
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("x is NaN");
    } else {
        println!("x is not NaN");
    }

    //数字运算
    let sum = 1 + 2;
    let difference = 1 - 2;
    let product = 4 * 5;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!(
        "sum = {}, difference = {}, product = {}, quotient = {}, remainder = {}",
        sum, difference, product, quotient, remainder
    );

    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    //类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    //只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    //对于较长的数字，使用下划线进行分隔，以增强可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    //定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [42.0, 42f32, -42.0_f32];
    // 打印数组的第一个元素，并保留两位小数
    println!("{:.2}", forty_twos[0]);
}

//位运算
fn bitwise() {
    let a: u8 = 2; //无符号八位整数，二进制为00000010
                   //也可以写为let a : u8 = 0b00000010

    //二进制为00000011
    let b: u8 = 3;

    println!("a value is {:08b}", a);
    println!("b value is {:08b}", b);
    println!("a & b value is {:08b}", a & b);
    println!("a | b value is {:08b}", a | b);
    println!("a ^ b value is {:08b}", a ^ b);
    println!("!a value is {:08b}", !a);
    println!("a << b value is {:08b}", a << b);
    println!("a >> b value is {:08b}", a >> b);

    let mut a = a;
    a <<= b;
    println!("a <<= b value is {:08b}", a);
}

//序列
fn sequence() {
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
    for i in 'a'..'z' {
        print!("{} ", i);
    }
    println!();
}

//有理数和复数
use num::complex::Complex;
fn complex_number() {
    let a = Complex { re: 1.0, im: 2.0 };
    let b = Complex::new(3.0, 4.0);
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
}

//字符类型 布尔类型 单元类型
fn char_type() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!(
        "c = {}, z = {}, g = {}, heart_eyed_cat = {}",
        c, z, g, heart_eyed_cat
    );

    //Rust 的字符不仅仅是 ASCII，所有的 Unicode 值都可以作为 Rust 字符
    println!(
        "字符'国'占用了 {} 个字节的内存大小",
        std::mem::size_of_val(&g)
    );
    //Rust 的字符只能用 '' 来表示， "" 是留给字符串的。

    //布尔类型
    //Rust 中的布尔类型有两个可能的值：true 和 false，布尔值占用内存的大小为 1 个字节
    let _t = true;
    let f: bool = false;

    if f {
        println!("这是一个 false");
    }

    //单元类型
    let _unit = (); //单元类型就是 ()，完全不占用如何内存
}

//语句和表达式
fn statement_and_expression() {
    //语句会执行一些操作但是不会返回一个值，而表达式会在求值后返回一个值
    //语句
    let _a = 8;
    let _b: Vec<u8> = Vec::new();
    let (_a, _c) = ("hi", false);
    // let b = (let a = 8); 这是错误的，由于 let 是语句，因此不能将 let 语句赋值给其它值

    //表达式
    let y = {
        let x = 3;
        x + 1 //表达式不能有分号
    };
    println!("y = {}", y);

    fn ret_unit_type() {
        let x = 1;
        // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
        // 类似三元运算符，在Rust里我们可以这样写
        let _y = if x % 2 == 1 { "odd" } else { "even" };
    }
    assert_eq!(ret_unit_type(), ());
}

//函数
fn function() {
    fn add(i: i32, j: i32) -> i32 {
        i + j
    }
    let x = add(1, 2);
    println!("x = {}", x);

    //函数名和变量名使用蛇形命名法(snake case)，例如 fn add_two() -> {}
    //函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
    //每个函数参数都需要标注类型
    another_function(5, 6.1);
    fn another_function(x: i32, y: f32) {
        println!("x = {}, y = {}", x, y);
    }
    /* 错误
    fn another_function(x: i32, y) {
        println!("x = {}, y = {}", x, y);
    }
     */

    //函数返回
    fn plus_or_minus(x: i32) -> i32 {
        if x > 5 {
            return x - 5;
        }

        x + 5
    }
    let x = plus_or_minus(5);
    println!("x = {}", x);

    //永不返回的发散函数
    //当用 ! 作函数返回类型的时候，表示该函数永不返回
    fn _divergent() -> ! {
        panic!("This function never returns.");
    }
}

fn main() {
    typename();
    yichu();
    floattype();
    bitwise();
    sequence();
    complex_number();
    char_type();
    statement_and_expression();
    function();
}
