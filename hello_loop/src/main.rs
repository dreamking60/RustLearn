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
}
