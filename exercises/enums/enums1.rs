// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message {
	Quit,
	Echo(String),
	Move{x : i32, y: i32},
    ChangeColor(i32, i32, i32),

}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo("wololoo".to_string()));
    println!("{:?}", Message::Move{x:1,y:1});
    println!("{:?}", Message::ChangeColor(0,0,0));
}
