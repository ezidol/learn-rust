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
}
