
struct _User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn _test_user_struct() {
    let mut user1 = _User {
        email: String::from("void@mail.com"),
        username: String::from("void"),
        active: true,
        sign_in_count: 1
    };

    let _name = user1.username;
    user1.username = String::from("baulat456");

    let user2 = _build_user(
        String::from("notbaulat@mail.com"),
        String::from("notbaulat789")
    );

    let _user3 = _User {
        email: String::from("bhau@mail.com"),
        username: String::from("bhau"),
        ..user2 
    };

    struct _Color(i32, i32, i32);
    struct _Point(i32, i32, i32);
}

fn _build_user(email: String, username: String) -> _User {
    _User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct _Rectangle {
    width: u32,
    height: u32,
}

impl _Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &_Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl _Rectangle {
    fn square(size: u32) -> _Rectangle {
        _Rectangle {
            width: size,
            height: size
        }
    }
}

fn _test_area_struct() {
    let rect = _Rectangle {
        width: 30,
        height: 50
    };

    let rect1 = _Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = _Rectangle {
        width: 40,
        height: 50
    };

    let rect3 = _Rectangle::square(27);

    println!("Square size is {} x {}", rect3.width, rect3.height);

    println!("rect can{} hold rect1.", 
        if rect.can_hold(&rect1) {""}
        else {"n't"}
    );
    println!("rect can{} hold rect2.",
        if rect.can_hold(&rect2) {""}
        else {"'t"}
    );

    println!("rect: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}

fn main() {
    // _test_user_struct();
    _test_area_struct();
}
