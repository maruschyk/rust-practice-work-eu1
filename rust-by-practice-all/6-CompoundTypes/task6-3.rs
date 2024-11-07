enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move { x: 1, y: 2 };

    if let Message::Move { x, y } = msg {
        assert_eq!(x, y); // This will fail as x and y are different
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
}
