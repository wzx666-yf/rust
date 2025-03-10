//所有权
fn ownership() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);
}

fn calculate_length(s: &String) -> usize {
    // s 的作用域是 calculate_length 函数，因此 s 的引用 s 是无效的
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn main() {
    ownership();
}
