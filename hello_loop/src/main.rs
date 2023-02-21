fn main() {
    let mut count = 0;
    let mut tmp = 0;

    let result = loop {
        count += 1;
        tmp += 1;
        if count == 10 {
            break count * 2 * tmp;
        }
    };

    println!("The result is {}", result);
    println!("The count is {}", count);
    println!("The tmp is {}", tmp);

    count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // conditonal loops with while
    let mut number = 3;

    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // conditonal loops with for
    let a = [10, 20, 30, 40, 50];

    // for is safe and easy to use 
    // no need to consider the index out of range
    for element in a {
        println!("the value is: {element}");
    }

    // Range in for
    for time in (1..4).rev() {
        println!("Time = {time}");
    }
    println!("LIFTOFF!!!");

}
