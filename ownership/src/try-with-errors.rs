// This fails borrow checker because largest has immutable reference to dst but
// inside if check dst.push() requires mutable reference and it might deallocate memory
// thats being referenced by largest variable 
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = 
      dst.iter().max_by_key(|s| s.len()).unwrap();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

// one solution is to reduce the life span of largest so that it doesnt have reference
// to dst by the time we need to push data into dst, thinking of it
// infact we need only the length of s, so we can have a variable just to take length
// of largest string in dst
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = 
        dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}