
fn main() {
    // 1st method, but lose relation between width and height
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area1 of the rectangle is {} square pixels.", 
        area1(width1, height1)
    );

    // 2nd method, tuple has no element name
    let rect1 = (30, 50);

    println!(
        "The area2 of the rectangle is {} square pixels.", 
        area2(rect1)
    );

    // 3rd method
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area3 of the rectangle is {} square pixels.", 
        area3(&rect2)
    );
    // Error because Rectangle no fmt::Disply
    //println!("rect2: {}", rect2);
    //println!("rect2: {:?}", rect2);
    //println!("rect2: {:#?}", rect2);
    //dbg!(&rect2);

    // method of struct
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.", 
        rect3.area()
    );

    // multi parameter method
    let rect4 = Rectangle {
        width: 10,
        height: 20,
    };

    let rect5 = Rectangle {
        width: 40,
        height: 30,
    };

    println!("Can rect3 hold rect4: {}", rect3.can_hold(&rect4));
    println!("Can rect3 hold rect5: {}", rect3.can_hold(&rect5));

    let sq = Rectangle::square(10);
    println!("Square's area: {}", sq.area());

}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // if you want to revise sth, use &mut self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // compare width and height
    // judge if can hold
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // if without &self, it is a associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}