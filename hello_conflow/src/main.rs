fn main() {
    //let number = 7; //false
    let number = 3; //true

    // if only allow bool type (not integer)
    if number < 5 {
        println!("condition was true"); 
    } else {
        println!("condition was false");
    }

    // if in a let Statement
    let condition = true;
    // in this situation the branch should return the same kind of value
    let result = if condition { 5 } else { 6 };
    println!("The value of result is: {result}");

}
