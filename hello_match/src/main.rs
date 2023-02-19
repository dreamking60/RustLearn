fn main() {
    // if let
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite coor, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color.");
        } else {
            println!("Using orange as the background color.");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let
    // Vec is dynamic array like Vector in Cpp
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // for Loops
    // vec! is a marco to crate array with init value
    let v = vec!['a', 'b', 'c'];
    
    // iter() return a iterator contains all items in array
    // enumerate() get index and item return as a tuple
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let Statements
    let x = 5;
    let (a1, a2, a3) = (1, 2, 3);
    println!("a1: {a1}, a2: {a2}, a3: {a3}, x: {x}");

    // function parameters
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
