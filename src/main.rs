fn main() {
    let item = Message::Quit;
    item.println();
    let item = Message::Move { x: 1, y: 2 };
    item.println();
    let item = Message::Write(String::from("foo"));
    item.println();
    let item = Message::ChangeColor(1, 2, 3);
    item.println();
    Message::Quit.println();
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn println(&self) {
        println!("{:?}", self)
    }
}
