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

}
