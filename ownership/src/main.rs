mod practice;

fn main() {
    // Just letting it be
    println!("Hello, world!");

    let mut n:i32 = 5;
    let x:i32  = increment(&mut n);
    println!("{}", x);

    practice::do_stuff();
    return_a_string();

    let name = vec![String::from("Vamsi")];
    let first = &name[0];
    stringify_name_with_title(&name);
    println!("{}",first);
}

fn increment(x: &mut i32) -> i32{
    //a reference (like &mut i32) is not itself the value; it’s a pointer to the value. 
    // If you want to access or modify the actual i32 behind the reference, 
    // you must use the dereference operator * on that reference.
    *x = *x+1;
    *x
}

// This gives error because The variable s is declared inside the function return_a_string(). 
// As soon as the function finishes executing, s goes out of scope and is dropped (its memory is deallocated).

// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s
// }

// This is how we fix it
fn return_a_string() -> String {
    let s = String::from("Hello ownership");
    s
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    //If we dont use clone() here we would be needed to pass &mut instead of & to the name and even then it would give error 
    // because we would be having both immutable ref(at first inside main() ) and mutable ref inside this funcntion 
    name.clone().push(String::from("Esq.")); 
    let full = name.join(" ");
    println!("Inside stringify {}",full);
    full
}