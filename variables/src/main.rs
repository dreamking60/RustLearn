fn main() {
    //immutable
    //let x = 5;
    //mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The const is: {THREE_HOURS_IN_SECONDS}");

    //immutable
    let y = 5;
    let y = y+1;
    println!("The value of y is: {y}");
    {
        let y = y*2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    //mutable
    let mut z = 6;
    println!("The value of z is: {z}");
    {
        z = z*2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("The valjue of z is: {z}");

    //compund Type
    // Tuple
    let tup: (i32,f64,u8) = (500, 6.4, 1);
    println!("The value of tup is: {}, {}, {}", tup.0, tup.1, tup.2);
    let (t1, t2, t3) = tup;
    println!("The value of t1, t2, t3 are: {t1}, {t2}, {t3}");

    let u1 = tup.0;
    let u2 = tup.1;
    let u3 = tup.2;
    println!("The value of u1, u2, u3 are: {u1}, {u2}, {u3}");

    //Array
    let _a = [1,2,3,4,5];

}
