fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    
    let home = IpAddr {
        kind: IpAddrKind::V4(1, 1, 1, 1),
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6(String::from("fdkas")),
        address: String::from("::1"),
    }; 
    let some_number = Some(5); 
    let some_string = Some("a string"); 
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let sum = x + y;
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}