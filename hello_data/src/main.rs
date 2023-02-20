fn main() {
    let _i32_max = std::i32::MAX;
    let _i32_min = std::i32::MIN;
    println!("Max of i32: {}", _i32_max);
    println!("Min of i32: {}", _i32_min);

    // isize is the length of the machine
    // my computer is Win-64, so the result is i64
    let _isize_max = std::isize::MAX;
    let _isize_min = std::isize::MIN;
    println!("Max of word: {}", _isize_max);
    println!("Min of word: {}", _isize_min);

    let _i64_max = std::i64::MAX;
    let _i64_min = std::i64::MIN;
    println!("Max of i64: {}", _i64_max);
    println!("Min of i64: {}", _i64_min);

    let _i128_max = std::i128::MAX;
    let _i128_min = std::i128::MIN;
    println!("Max of i128: {}", _i128_max);
    println!("Min of i128: {}", _i128_min);

    let a = 0b101;
    let b = 0o17;
    let c = 0xac;
    println!("a: {}\nb: {}\nc: {}\n", a, b, c);

    // change type
    // i to i
    assert_eq!(-1_i16 as i32, -1_i32);
    // i to u
    assert_eq!(-1_i32 as u8, 255_u8);
    // cut of 
    assert_eq!(1000_i16 as u8, 232_u8);
    // f to u
    assert_eq!(33.33_f32 as u8, 33_u8);
    // test wrong?
    //assert_eq!(33.32_f32 as f64, 33.32_f64);

    // method...
    println!("3^2 = {}", 3_u8.pow(2));
    println!("|-3| = {}", (-3_i32).abs());
    println!("45 has 1: {} and 0: {}", 45i32.count_ones(), 45i32.count_zeros());

    // ASCII
    let a = b'A';
    let b = a+1;
    println!("a = {}, b = {}", a, b);
    
    // bool
    let is_correct = true;
    if is_correct {
        println!("The Result is True!");
    } else {
        println!("The Result is False!");
    }

    // print true and false
    println!("True: {}", true as u32);
    println!("False: {}", false as u8);
    //println!("{}", 1u8 as bool);

    // char


}