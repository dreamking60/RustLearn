# Rust Learn
Learning coding by Rust.

## Chapter1
./hello_cargo
./hello_world
### Compile and Execute
```sh
#compile
rustc main.rs
#execute
./main
```

### Build project by cargo
```sh
#init project
cargo new [project name]
#build project for debug
cargo build
#run the project
cargo run
#check the project's executable file
cargo check
# build for release
cargo build --release
```

### Expression
All code in Rust can be specified as a expression. And add ';' can make it be sentence.

## Chapter2
./guessing_game
Use Cargo.toml file to manage rust project

## Chapter3
### varables and Mutability
By default varaiables are immutable.  
Use `mut` to make variables mutable.  
Constant value use `const` to define.
```Rust
    //immutable
    let x = 5; 
    //mutable
    let mut y = 5;
    //constant
    const z = 5;
```
Declare a new variable with the same name sa a previous variable, then the first variable is shadowed by the second.
```Rust
    let x = 5;
    let x = x+1;
    
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
```

Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

If defined a variable with using, it will warn. Or you need to add '_' before the variable name to ignore the warning. And using '_' as variable name will ignore the apply process.

The new variable with the same name as the old variable will shadow the old one. But the old one is still alive in Stack and Heap.

let is sentence but not expression.

In Rust, the apply value to variable is actually the pattern match.

### Data Types
Basic Type: Integer, float, boolean and char.  

Compund Type: tuple and array.
```Rust
    let tup: (i32, f64, u8) = (500, 6.4, 1)
```
### Conditon
#### match
arm is a branch of match expression.
```Rust
let x = 5;
match x {
    0 => println!("x is zero"),
    1..=5 => println!("x is between one and five"),
    _ => println!("x is something else"),
}
```

## Chapter4