fn main() {
    println!("I'm just practicing rust");
    println!("result of interproduct {}", interproduct(10, 30, 40));
}

fn interproduct(a:i32, b:i32, c:i32) -> i32{
    return a*b*c+10
}