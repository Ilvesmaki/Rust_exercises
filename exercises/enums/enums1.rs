// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

// I AM NOT DONE

#[derive(Debug)]
enum Message {
	Quit,
	Echo(String),
	Move(x : i32, y: i32),
	ChangeColor(i32, i32, i32),


    // TODO: define a few types of messages as used below
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
