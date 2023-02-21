fn main() {
    // struct user and how to init
    let user1 = User {
        active: true,
        username: String::from("dreamking60"),
        email: String::from("dreamlyboyczh@foxmail.com"),
        sign_in_count: 1,
    };

    if user1.active {
        println!("User: {}, email: {}, count: {}", user1.username, user1.email, user1.sign_in_count);
    }

    // mutable user
    let mut user2 = User {
        active: true,
        username: String::from("gigi"),
        email: String::from("example@foxmail.com"),
        sign_in_count: 10,
    };

    if user2.active {
        println!("User: {}, email: {}, count: {}", user2.username, user2.email, user2.sign_in_count);
    }

    user2.email = String::from("gigi@foxmail.com");
    if user2.active {
        println!("User: {}, email: {}, count: {}", user2.username, user2.email, user2.sign_in_count);
    }

    // fn build_user
    let user3 = build_user(String::from("example@hotmail.com"), String::from("example"));

    print_user(&user3);

    // Tips!!! struct update also obey the ownership rule
    // struct update syntax using other user's value
    let user4 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("dreamlyboyczh@outlook.com"),
        sign_in_count: user1.sign_in_count,
    };

    print_user(&user4);
    
    // struct update syntax a much simpler way
    let user5 = User{
        email: String::from("gigi@outlook.com"),
        username: String::from("gigifo"),
        // ..user need at the last
        ..user2
    };
    
    print_user(&user5);
    //print_user(&user1);
    print_user(&user2);

    // tuple structs
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // unit-like structs
    let _subject = AlwaysEqual;

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        //username: username,
        // field init shorthand
        username,
        //email: email, 
        // field init shorthand
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    if user.active {
        println!("User: {}, email: {}, count: {}", user.username, user.email, user.sign_in_count);
    } else {
        println!("User is not active!");
    }
}


