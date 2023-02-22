fn main() {
    // how to crate a obj of a enum
    //let ip1_v4 = IpAddrKind::V4;
    //let ip2_v6 = IpAddrKind::V6;
    let _home = IpAddrKind::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();
}

enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //method
    }
}

// function can user IpAddrKind as parameter
//fn route(ip_kind: IpAddrKind) {}
