fn main() {
    let item = Message::Quit;
    println(item);
    let item = Message::Move { x: 1, y: 2 };
    println(item);
    let item = Message::Write(String::from("foo"));
    println(item);
    let item = Message::ChangeColor(1, 2, 3);
    println(item);
    println(Message::Quit);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn println(item: Message) {
    println!("{:?}", item)
}
