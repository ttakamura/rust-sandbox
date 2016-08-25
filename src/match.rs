struct Point {
    x: i32,
}

struct Person {
    name: Option<String>,
}

fn main(){
    let c = 3;
    match c {
        y @ 1 ... 3 => println!("y = {}", y),
        x           => println!("x = {}", x),
    }

    let p = Point{ x: 23 };
    match p {
        Point{ x: x } => println!("Point.x = {}",x),
    }

    let name = "Stave".to_string();
    let tom: Option<Person> = Some(Person{ name: Some(name) });

    match tom {
        Some(Person{ name: ref x @ Some(_) }) => println!("{:?}", x),
        _ => println!("anything"),
    }
}
