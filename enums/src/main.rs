/*
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function() {
        println!("Let's get Russsssty");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn _test_enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    Message::_some_function();
}
*/

fn _test_enum_option() {
    let x = 5;
    let y = Option<i8> = Some(5);

    let sum = x + y;
}

fn main() {
    // _test_enums();
    _test_enum_option();
}
