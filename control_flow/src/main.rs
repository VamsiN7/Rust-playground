fn main() {
    let z = 10;
    let x = {
        let y = 12;
        dbg!(y);
        z-y
    };
    dbg!(x);
    test_match(x);
    test_for();
    println!("collatz test");
    println!("Length: {}", collatz_length(11)); // should be 15
}

fn test_match(x: i32) {
    match x {
        -2 => println!("x is -ve"),
        2 => println!("x is -ve"),
        0 => println!("x is 0"),
        _ => println!("did you change the values?"),
    }
}

fn test_for(){
    for i in 1..5{
        dbg!(i);
    }
    println!("for from collection");
    for j in [2,4,6,8]{
        dbg!(j);
    }
}

// If ni is 1, then the sequence terminates at ni.
// If ni is even, then ni+1 = ni / 2.
// If ni is odd, then ni+1 = 3 * ni + 1.
// For example, beginning with n1 = 3:

// 3 is odd, so n2 = 3 * 3 + 1 = 10;
// 10 is even, so n3 = 10 / 2 = 5;
// 5 is odd, so n4 = 3 * 5 + 1 = 16;
// 16 is even, so n5 = 16 / 2 = 8;
// 8 is even, so n6 = 8 / 2 = 4;
// 4 is even, so n7 = 4 / 2 = 2;
// 2 is even, so n8 = 1; and
// the sequence terminates.

/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut iterations: u32 = 1;
    
    while n!=1 {
        if n % 2 == 0 {
            n = n/2;
        } else {
            n = 3 * n + 1;
        }
        iterations+=1
    }

    iterations
}