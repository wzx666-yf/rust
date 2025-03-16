//match 和 if let
fn match_ex() {
    #[allow(dead_code)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dire = Direction::South;

    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => println!("North or South"),
        _ => println!("West"),
    };

    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    println!("{}", value_in_cents(Coin::Penny));

    //使用 match 表达式赋值
    #[allow(dead_code)]
    enum IpAddr {
        Ipv4,
        Ipv6,
    }

    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);

    //模式绑定
    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
    }

    let actions = [
        Action::Say("Hello".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("Say: {}", s);
            }
            Action::MoveTo(x, y) => {
                println!("print from (0,0) Move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!("Change color to RGB({}, {}, 0)", r, g,);
            }
        }
    }

    //_通配符
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), //_通配符
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Direction2 {
        East,
        West,
        North,
        South,
    }
    let dire = Direction2::East;
    match dire {
        Direction2::North => println!("North"),
        other => println!("{:?}", other),
    };
}

fn main() {
    match_ex();
}
