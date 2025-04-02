// fn longest(x: &str, y: &str) -> &str { //错误，主要是编译器无法知道该函数的返回值到底引用 x 还是 y ，因为编译器需要知道这些，来确保函数调用后的引用生命周期分析。
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
//'a 的大小将等于 x 和 y 中较小的那个。由于返回值的生命周期也被标记为 'a，因此返回值的生命周期也是 x 和 y 中作用域较小的那个。

fn longest1<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}

// 错误
// fn longest2<'a>(x: &'a str, y: &str) -> &'a str{
//     let result = String::from("really long string");
//     result.as_str()
// }

fn longest2<'a>(x: &'a str, y: &'a str) -> String {
    String::from("really long string")
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 当存在多个引用时，需要手动标注生命周期来帮助编译器进行借用检查分析
fn main() {
    // 生命周期的主要作用是避免悬垂引用，它会导致程序引用了本不该引用的数据
    // let r;
    // {
    //     let x = 5;
    //     r = &x; //r引用x，但是x在下面的}作用域里被销毁，r就变成了悬垂引用，因此报错
    // }
    // println!("r: {}", r);

    //函数的生命周期
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result_string = longest(string1.as_str(), string2);
    println!("The longest string is {}", result_string);

    // &i32 一个引用
    // &'a i32 具有显示生命周期的引用
    // &'a mut i32 具有显示生命周期的可变引用
    // 一个生命周期标注，它自身并不具有什么意义。因为生命周期的作用就是告诉编译器多个引用之间的关系。
    // let string3 = String::from("long string is long");
    // let result = {
    //     let string4 = String::from("xyz");
    //     longest(string3.as_str(), string4.as_str()) //报错 在上述代码中，result 必须要活到 println!处，因为 result 的生命周期是 'a，因此 'a 必须持续到 println!。
    // }
    // println!("The longest string is {}", result);

    let string3 = String::from("asdfg");
    {
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    }

    // 结构体的生命周期
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    //相反的列子
    //结构体比它引用的字符串活得更久
    // let i;
    // {
    //     let novel = String::from("Call me Ishmael. Some years ago...");
    //     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //     i = ImportantExcerpt {
    //         part: first_sentence,
    //     };
    // }
    // println!("i: {:#?}", i);
    
    // 方法中的生命周期
    struct Point<'a>{
        x : &'a str,
    }
    
    impl<'a> Point<'a> {
        fn level(&self) -> i32{
            3
        }
        
        fn announce(&self, announcement: &str) -> &str{
            println!("Announcement: {}", announcement);
            self.x
        }
        
        // fn announce_and_return_x<'b>(&'a self, announcement: &'b str) -> &'b str{
        //     println!("Announcement: {}", announcement);
        //     self.x
        // }
        // 报错，不知道a与b的关系
        fn announce_and_return_x1<'b>(&'a self, announcement: &'b str) -> &'b str
        where 'a : 'b
        {
            println!("Announcement: {}", announcement);
            self.x
        }
    }
    
    impl <'a : 'b, 'b> Point<'a> { //用于说明 'a 必须比 'b 活得久
        fn announce_and_return_x(&'a self, announcement: &'b str) -> &'b str {
            println!("Announcement: {}", announcement);
            self.x
        }
    }
    
    // 静态生命周期
    // 与程序活得一样久
    let s: &'static str = "I have a static lifetime.";
    println!("s: {}", s);
}
