fn main() {
    // Each value in Rust has an owner
    // There can only be one owner at a time
    // When the owner goes out of scope, the value will be dropped

    // sr is a owner
    let sr = String::from("hello");
    // sr didn't move to fn print_string_reference
    print_string_reference(&sr);
    // sr can print
    println!("After fn: {}", sr);
    println!("After fn: {}", &sr);

    // s is a owner with String value "hello"
    let s = String::from("hello");
    // s move to fn print_string
    print_string(s);
    // s has been removed in print_string
    //println!("After fn: {}", s);

    // sc common String
    let sc= "hello";
    print_str(sc);
    // after fn print_string 
    println!("After fn: {}", sc);

    // mut s
    let mut strm = "Hello";
    println!("{strm}");
    strm = "Hello World";
    println!("{strm}");

    // mut stringm
    let mut stringm = String::from("Hello");
    println!("{stringm}");
    stringm.push_str(" World");
    println!("{stringm}");

    // variable & data interact with move
    let mut x = 5;
    // y is a copy of x's value
    // this is fast and no need a move
    let y = x;
    println!("x = {}, y = {}", x, y);
    x = 6;
    println!("x = {}, y = {}", x, y);

    // s2 = s1 is like a move
    // Rust doesn't have a automatic deep copy
    let s1 = String::from("sustc");
    println!("s1 = {s1}");
    let s2 = s1;
    println!("s2 = {s2}");
    // after s2 = s1, s1 is invalid
    //println!("s1 = {s1}");

    // clone is a deep copy
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    // return value and its area
    let t1 = gives_ownership();
    let t2 = String::from("mine");
    // t2 move to t3 here and t2 invalid
    let t3 = takes_and_gives_back(t2);

    println!("{t1}");
    //println!("{t2}");
    println!("{t3}");

}

fn print_string(s: String) {
    println!("{}", s);
    // s will destroy after the function
}

fn print_string_reference(s: &String) {
    println!("{}", s);
    // s is a reference and won't be destroyed
}

fn print_str(s: &str) {
    println!("{}", s);
}

fn gives_ownership() -> String {
    let  some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}