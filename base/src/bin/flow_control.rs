//if语句
fn if_ex() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    //用 if 来赋值时，要保证每个分支返回的类型一样
    println!("The value of number is: {}", number);

    let n = 6;
    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn for_ex() {
    for i in 1..=5 {
        println!("{}", i);
    }
    /*
        for item in collection           for item in IntoIterator::into_iter(collection)  转移所有权
        for item in &collection          for item in collection.iter()                    不可变借用
        for item in &mut collection      for item in collection.iter_mut()                可变借用
    */

    let a = [10, 20, 30, 40, 50];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{} 个元素是 {}", i + 1, v);
    }

    //for _ in 0..5 {
    //    println!("hello");
    //}

    //continue
    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }

    //break
    for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("{}", i);
    }
}

//while
fn while_ex() {
    let mut n = 0;
    while n <= 5 {
        println!("{}", n);
        n += 1;
    }
}

//loop
fn loop_ex() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn main() {
    if_ex();
    for_ex();
    while_ex();
    loop_ex();
}
