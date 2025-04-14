use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    let sam = User {
        name: String::from("sam"),
        active: true,
        email: String::from("sam@ed.com"),
    };

    let sams_mail = sam.email;
    println!("sams email : {} ", sams_mail);

    let vamsi = build_user(String::from("vamsi"), String::from("v@ed.com"));
    println!("vamsi: {:?}", vamsi);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let area = rect.rect_area();
    println!("area of rectangle {} ", area);
}

#[derive(Debug)]
struct User {
    name: String,
    active: bool,
    email: String,
}

fn build_user(name: String, email: String) -> User {
    User {
        name: name,
        active: true,
        email: email,
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn rect_area(&self) -> u32 {
        self.width * self.height
    }
}
