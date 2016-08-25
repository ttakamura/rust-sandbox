enum Message {
    Quit,
    Move { x: i32, y: i32 }
}

fn main(){
    let x: Message = Message::Move{ x:3, y:5 };
    println!("{}", x);
}
