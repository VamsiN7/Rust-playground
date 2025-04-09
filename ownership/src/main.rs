mod practice;

fn main() {
    // Just letting it be
    println!("Hello, world!");

    let mut n:i32 = 5;
    let x:i32  = increment(&mut n);
    println!("{}", x);

    // practice::do_stuff();

}

fn increment(x: &mut i32) -> i32{
    //a reference (like &mut i32) is not itself the value; it’s a pointer to the value. 
    // If you want to access or modify the actual i32 behind the reference, 
    // you must use the dereference operator * on that reference.
    *x = *x+1;
    *x
}
