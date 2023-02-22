use rand::Rng;

fn main() {
    // how to crate a obj of a enum
    //let ip1_v4 = IpAddrKind::V4;
    //let ip2_v6 = IpAddrKind::V6;
    let _home = IpAddrKind::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // some_number is not i32 so it can't print as i32 directly
    // however we can make a translate use the match
    //println!("Number: {}", some_number);
    match some_number{
        Some(x) => println!("Number: {}", x),
        None => println!("Number is null"),
    }

    // match Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    print_op(six);
    print_op(none);

    // pattern match and '_'
    let dice_roll = rand::thread_rng().gen_range(1..=10);
    match dice_roll {
        3 => println!("add fancy hat!"),
        7 => println!("remove fancy hat!"),
        _ => println!("move player"),
        //other => remove_player(other),
    }

    // if let
    let config_max = Some(3u8);
    // match config_max {
    //     Some(x) => println!("Config_max: {x}"),
    //     _ => (),
    // }
    if let Some(x) = config_max {
        println!("Config_max: {x}");
    }

    let mut count = 0;
    let coin = Coin::Dime;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}", state),
    //     _ => count += 1,
    // }
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
    println!("Count: {count}");

}

enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

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
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn print_op(x: Option<i32>) {
    match x {
        None => {
            println!("None");
        }
        Some(i) => {
            println!("Value is {}", i);
        }
    }
}

impl Message {
    fn call(&self) {
        //method
    }
}

// function can user IpAddrKind as parameter
//fn route(ip_kind: IpAddrKind) {}
