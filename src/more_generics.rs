struct Point<T>{
    x: T
}

fn main(){
    let o: Point<u32> = Point{ x: 5 };
    println!("{}", o.x);
    let o2: Point<u8> = Point{ x: 5 };
    println!("{}", o2.x);
}
