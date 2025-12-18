#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

struct Coordinates(i32, i32, i32);
struct _UnitStruct;

struct Square {
    height: u32,
    width: u32,
}

#[derive(Debug)]
struct MyString<'a> {
    text: &'a str,
}

impl Square {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn whats_my_width(&self) -> u32 {
        self.width
    }

    fn change_width(&mut self, new_width: u32) {
        self.width = new_width
    }
}

pub fn main() {
    let user1 = User {
        active: true,
        username: String::from("Tyler"),
        sign_in_count: 0,
    };
    println!("{}{:?}", user1.username, user1);

    let user2 = build_user(String::from("Tyler2"));
    println!("{}", user2.username);

    let _cords = Coordinates(1, 2, 3);
    // println!("{:?}", cords)

    // Methods
    let mut sq = Square {
        width: 5,
        height: 5,
    };
    println!("{}", sq.area());
    println!("{}", sq.whats_my_width());
    sq.change_width(12);
    println!("{}", sq.whats_my_width());

    // Lifetimes
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r);

    let str1 = String::from("This is my string");
    let xr = MyString {
        text: str1.as_str(),
    };
    let s: &'static str = "I have a static lifetime";
    println!("{:?}, {}", xr.text, s)
}

fn build_user(username: String) -> User {
    User {
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
    y
}
