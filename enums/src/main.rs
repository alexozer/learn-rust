enum IpAddr {
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
    fn poop(&self) {
    }
}

fn main() {
    let four = IpAddr::V4(String::from("poops"));
    let six = IpAddr::V6(String::from("peepee"));
    route(four);
    route(six);

    let msg = Message::Write(String::from("pooper"));
    msg.poop();

    let some_number = Some(5);
    let some_str = Some("hi there");
    let absent_number: Option<i32> = None;
}

fn route(addr: IpAddr) {}
