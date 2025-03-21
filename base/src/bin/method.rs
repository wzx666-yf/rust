// 方法
// Rust 的对象定义和方法定义是分离的
fn method() {
    //定义
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
        // 这种方法往往用于初始化当前结构体的实例
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x: x,
                y: y,
                radius: radius,
            }
        }

        // Circle的方法，&self表示借用当前的Circle结构体
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    let c = Circle::new(10.0, 20.0, 5.0);
    println!("{:.2}", c.area());

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        // 方法名与结构体名相同
        fn width(&self) -> bool {
            self.width > 0
        }
        
        //带多个参数的方法
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let r = Rectangle {
        width: 10,
        height: 20,
    };
    println!("{:?}", r.area());
    if r.width() {
        println!("The rectangle has a nonzero width; it is {}", r.width);
    }

    mod my {
        pub struct Rectangle {
            width: u32,
            pub height: u32,
        }

        impl Rectangle {
            pub fn new(width: u32, height: u32) -> Self {
                Rectangle { width, height }
            }

            pub fn width(&self) -> u32 {
                return self.width;
            }

            pub fn height(&self) -> u32 {
                return self.height;
            }
        }
    }

    let rect1 = my::Rectangle::new(30, 50);
    println!("{}", rect1.width());
    println!("{}", rect1.height());
    // println!("{}",rect1.width); //错误，因为width是私有的
    println!("{}", rect1.height);
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    // 多个impl定义
    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
}
fn main() {
    method();
}
