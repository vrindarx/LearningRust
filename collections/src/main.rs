// fn collections() {
//     let a = [1, 2, 3];
//     let mut v:Vec<i32> = Vec::new();
//     v.push(1);
//     v.push(2);
//     v.push(3);

//     {
//         let v2 = vec![1, 2, 3];
//     }

//     let mut v = vec![1, 2, 3, 4, 5];

//     // let third = &v[2];
//     // v.push(6);
//     // println!("The third element is {}", third);

//     match v.get(20) {
//         Some(third) => println!("The third element is {}", third),
//         None => println!("There is no third element."),
//     }
// }

// fn vectors() {
//     let mut v = vec![1, 2, 3, 4, 5];

//     for i in &mut v {
//         // println!("{}", i);
//         *i += 50;
//     }

//     for i in &v {
//         println!("{}", i);
//     }
    
// }

// fn vectors2() {
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];

//     match &row[1] {
//         SpreadsheetCell::Int(i) => println!("{}", i),
//         _ => println!("Not an integer!")
//     };
// }

use unicode_segmentation::UnicodeSegmentation;

fn strings() {
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    // let s7: String = s5 + &s6;

    let s8 = format!("{}{}", s5, s6);

    let hello: String = String::from("नमस्कार"); // Marathi-lang

    for b in "नमस्कार".bytes() {
        println!("{}", b);
    }

    for c in "नमस्कार".chars() {
        println!("{}", c);
    }

    for g in "नमस्कार".graphemes(true) {
        println!("{}", g);
    }
}

fn main() {
    // collections();
    // vectors();
    // vectors2();
    strings();
}

