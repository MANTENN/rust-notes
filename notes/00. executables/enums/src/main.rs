#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    Not(Rectangle),
    Move {
        name: String
    },
}

#[derive(Debug)]
enum UsState {
    Alabama,
    _Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from: {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => {
            Some(i + 1)
        },
        None => None
    }
}

fn main() {

    let home = IpAddr::V4(127,0,0,1);

    let loopback = IpAddr::V6(String::from("::1"));
    let not = IpAddr::Not(Rectangle {
        width: 10,
        height: 10
    });
    let move_s = IpAddr::Move {
        name: String::from("test")
    };

    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);

    println!("five {}", five.unwrap());
    println!("six {}", six.unwrap());

    println!("Hello, world!");
    println!("home: {:#?}", home);
    println!("loopback: {:#?}", loopback);
    println!("not: {:#?}", not);
    println!("moveS: {:#?}", move_s);
    value_in_cents(Coin::Quarter(UsState::Alabama));

    let some_u8_value = Some(3u8);
    // match some_u8_value {
    //     1 => println!("one"),
    //     3 => println!("three"),
    //     5 => println!("five"),
    //     7 => println!("seven"),
    //     _ => (),
    // };

    if let Some(3) = some_u8_value {
    println!("three");

    let coin = Coin::Nickel;
    let mut count = 0;
    println!("count {}", count);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count {}", count);
}
}
