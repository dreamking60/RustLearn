fn main() {
    let x = 5;

    // Pass x by value
    fn_by_value(x);
    println!("x = {}", x); // This will compile, since x was not moved into the function

    // Pass a reference to x
    fn_by_reference(&x);
    println!("x = {}", x); // This will also compile, since x was not consumed by the function
}

fn fn_by_value(x: i32) {
    // Do something with x
    println!("fn_by_value: x = {}", x);
    // x is consumed and goes out of scope here
}

fn fn_by_reference(x: &i32) {
    // Do something with x
    println!("fn_by_reference: x = {}", x);
    // x is not consumed and can still be used outside of the function
}
