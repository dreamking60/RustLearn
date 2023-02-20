fn main() {
    let x = 5;
    let y: i32 = 10;
    let res = fn_add(x, y);

    // Pass x by value
    fn_by_value(x);
    println!("x = {}", x); // This will compile, since x was not moved into the function

    // Pass a reference to x
    fn_by_reference(&x);
    println!("x = {}", x); // This will also compile, since x was not consumed by the function

    println!("res = {}", res);
    
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

fn fn_add(x: i32, y: i32) -> i32 {
    x + y
}