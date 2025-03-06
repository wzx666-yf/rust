//变量绑定与变量解构
fn main() {
    // x为不可变的变量，y为可变的变量
    let x = 5;
    let mut y = 10;
    println!("x is {}, y is {}", x, y);
    //x = 6; // 错误，不能修改x的值
    y = 14; // 加入mut,y值可以修改
    println!("x is {}", x);
    println!("y is {}", y);

    let _a = 1;
    let b = 2;
    //希望告诉 Rust 不要警告未使用的变量，为此可以用下划线作为变量名的开头

    //变量解构
    //let 表达式不仅仅用于变量的绑定，还能进行复杂变量的解构：从一个相对复杂的变量中，匹配出该变量的一部分内容
    let (x, y) = (1, 2);
    println!("x is {}, y is {}", x, y);
    let (a, mut b): (bool, bool) = (true, false);
    // a = true, 不可变; b = false, 可变
    println!("a is {:?}, b is {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    jiegoushi();

    //常量
    const MAX_POINTS: i32 = 100_000; //命名约定—————>大写，用下划线分隔单词
    println!("MAX_POINTS: {}", MAX_POINTS);
    /*常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变，因为常量在编译完成后，已经确定它的值。
    常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注。 */

    //变量遮蔽
    shadowing();

    let _b = 6;
    let b = 7;
    let b = b + 1;
    let b = b * 2;
    let b = b % 3;
    println!("b = {}", b); // b = 1

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);

    // let mut spaces = "   ";
    // spaces = spaces.len(); // 编译错误
}

struct Struct {
    e: i32,
}

//在 Rust 1.59 版本后，我们可以在赋值语句的左式中使用元组、切片和结构体模式了
fn jiegoushi() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);

    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
    println!("{:?}", [a, b, c, d, e]);
}

fn shadowing() {
    let x = 5;
    // 在shadowing函数的作用域内对之前的x进行遮蔽
    let x = x + 1;
    //第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配
    //，而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配
    {
        // 在作用域内对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
