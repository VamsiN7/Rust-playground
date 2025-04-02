fn main() {
    println!("I'm just practicing rust");
    println!("result of interproduct: {}", interproduct(10, 30, 40));

    println!("Testing type inference");
    takes_u32(1);
    takes_i8(1);

    println!("Trying fibonacci");
    let x = 10;
    println!("{}th fibonnaci number is {}",x, fib(x));
}

fn interproduct(a:i32, b:i32, c:i32) -> i32{
    return a*b*c+10
}

fn takes_u32(x: u32){
    println!("x: {}",x);
}

fn takes_i8(y: i8){
    println!("y: {} ",y);
}

fn fib(x: u32) -> u32 {
    if x<2 {
        x
    } else{
        fib(x-1)+fib(x-2)
    }
}