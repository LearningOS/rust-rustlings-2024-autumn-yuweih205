// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
Echo(i32),
Move(bool),
ChangeColor(String),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(42));
    println!("{:?}", Message::Move(true));
    println!("{:?}", Message::ChangeColor(String::from("red")));
}
