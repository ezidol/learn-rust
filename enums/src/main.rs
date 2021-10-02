#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8,u8,u8,u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> &Message {
        self
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alsaka,
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
            println!("Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home {:#?}", home);
    println!("loopback {:#?}", loopback);

    let home = IpAddr2::V4(127,0,0,1);
    let loopback = IpAddr2::V6(String::from("::1"));
    
    println!("home {:#?}", home);
    println!("loopback {:#?}", loopback);

    let m1 = Message::Write(String::from("Hello"));
    let m1_call = m1.call();
    println!("{:#?}", m1_call);

    let some_number = Some(10);
    let absent_number:Option<i8> = None;
    println!("{:?} {:?}", some_number, absent_number);

    //The match Control Flow Operator
    let p1 = Coin::Penny;
    let n1 = Coin::Nickel;
    let q1 = Coin::Quarter(UsState::Alabama);
    let value_p1 = value_in_cents(p1);
    let value_n1 = value_in_cents(n1);
    let value_q1 = value_in_cents(q1);
    println!("{} {} {}", value_p1, value_n1, value_q1);

    //Matching with Option<T>
    println!("{:?}", plus_one(None));

    //The _ Placeholder
    let some_u8_value = 0;
    match some_u8_value {
        0u8 => println!("Zero"),
        1 => println!("one"),
        3 => println!("three"),
        _ => (),
    }
}
