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

// fn _test_enum_option() {
//     let x = 5;
//     let y:Option<i8> = Some(5);
//     // let y:Option<i8> = None;

//     let sum = x + y.unwrap_or(2);

//     println!("sum is {}", sum);
// }

// fn test_match() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
//     // value_in_cents(Coin::Dime);
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     Arizona,
//     Arkansas,
//     California,

// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState)
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         // Coin::Penny => {
//         //     println!("Lucky penny!");
//         //     1
//         // },
//         // Coin::Nickel => 5,
//         // Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         },
//         _ => {
//             print!("Dr. Nickel Penny won't give a Dime about this");
//             0
//         }
//     }
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None
    }

}

fn test_plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_value = Some(3);

    // match some_value {
    //     Some(3) => print!("three"),
    //     _ => (),
    // }

    // create a variable with value Some(i), if Some(i) can be created and given the value of some_value, then it's true, otherwise false
    // (which only happens when Some(i)'s type matched some_value's type ... let a:i32 = 1; .. not if .. let a:i32 = "zen";)
    // ...i is a temporary variable created included in Some(i) 

    if let Some(i) = some_value {
        println!("three");
    }
}

fn main() {
    // _test_enums();
    // _test_enum_option();
    // test_match()
    test_plus_one()
}
 