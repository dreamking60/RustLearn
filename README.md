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
### Ownership Rules
First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

str and String are two different String type in Rust.
1. str is not dynamic
2. String is synamic
3. String = &str.to_string(), &str = &String
4. String has push_str and push method to add String, but doesn't; str can use [] to get one char, but String can't 

move and clone:
- move invalid the old value
- clone won't invalid the old one

For primitive data, no need a move, all copy

A data with drop won't have a copy

### Reference and borrow
reference &

mutable reference &mut

life based on usage

Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced:

dangle reference

## Chapter5 Struct

Struct data must have lifetime. So instead of &str, we use String.

```Rust
struct name {
    name: dataType,
    name: dataType,
}

impl name {
    fn method(&self) {
        self.name
    }

    fn method(&mut self) {
        self.name = ...
    }

}
```

In method, there is a channel called automatic referencing and dereferencing.

Mthod

Associated function

## Chapter6 Enum and Option
### Enum
Enum allow different types of variables in one datat type enum.

Enum can have impl similar as struct

### Option