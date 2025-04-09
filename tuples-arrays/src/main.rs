fn main() {
    println!("Hello, world!");

    let mut a:[u32;5] = [1,2,3,4,5];
    a[2]=0;

    let tuple: (i8, bool) = (7, true);
    println!("a : {:?}",a);
    println!("tuple details : {:?}",tuple.0);
}
