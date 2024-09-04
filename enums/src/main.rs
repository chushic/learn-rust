fn main() {
}

// 6.3.1 concise control flow with if-let

fn two_way() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {}
}

// catch-all and _ placeholder

fn dice_match() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}


// Matching with Option<T>

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}


// 6.2 Pattern the bind to values

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}



// 6.2 Control flow

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}


fn some_type() {
    let some_number = Some(5);
    let some_char = Some('e');
    
    let absent_number: Option<i32> = None;
}

fn addresses() {
    let home = IpAddr {
        kind: IpAddrKind::V4(String::from("IPv4")),
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6(String::from("hi")),
        address: String::from("::1"),
    };

}

fn messages() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

enum Message {
    Quit,
    Move {x: i32, y:i32}, //named fields
    Write(String), // includes a single string
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body
    }
}

// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColor(i32, i32, i32); // tuple struct


enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

