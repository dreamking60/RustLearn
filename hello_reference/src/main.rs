fn main() {
    // create reference borrowing & (dereference *)
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len);

    // mutable borrow
    change(&mut s1);
    println!("{s1}");

    // mutable borrow once at a time
    // can't have a immutable reference before a mutable reference
    let r0 = &s1;
    let r3 = &s1;
    println!("r0 = {}, r3 = {}", r0, r3);

    let r1 = &mut s1;
    println!("r1 = {}", r1);
    let r4 = &mut s1;
    println!("r4 = {}", r4);
    {
        let r2 = &mut s1;
        // if we don't use r1 in this scope,r2 is valid
        // if we use r1 in this scope, r2 get wrong
        //println!("r1 = {}", r1);
        r2.push_str(" dreamking");
        println!("r2 = {}", r2);
    }
    //println!("r1 = {}", r1);
    //println!("r4 = {}", r4);
    println!("s1 = {}", s1);

    //dangle reference
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);

    // slice

    
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// The following doesn't work because Rust not allow
// to modify a borrowed value
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// a mutable reference can be used.
fn change(some_string: &mut String) {
    some_string.push_str(" world");
}

// fn no_dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}