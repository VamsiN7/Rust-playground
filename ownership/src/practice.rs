fn do_stuff() {
    let mut m1 = String::from("Hello");
    let mut m2 = String::from("world");
    greet(&m1, &m2); //&mut if we intend to change m1 and m2 inside greet
    let _s = format!("{} {}", m1, m2);
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}