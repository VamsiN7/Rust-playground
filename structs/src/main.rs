use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    let sam = User {
        name: String::from("sam"),
        active: true,
        email: String::from("sam@ed.com")
    };

    let sams_mail = sam.email;
    println!("sams email : {} ", sams_mail);

    let vamsi = build_user(String::from("vamsi"), String::from("v@ed.com"));
    println!("vamsi: {:?}", vamsi);
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
        email: email
    }
}
