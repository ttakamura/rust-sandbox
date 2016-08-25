struct Point {
    x: i32,
    y: i32,
}

struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

// Tuple struct
struct Color(i32, i32, i32);

fn main(){
    let mut a = Point { x:0, y:0 };
    println!("x={}, y={}", a.x, a.y);
    {
        let r = PointRef {
            x: &mut a.x,
            y: &mut a.y
        };
        *r.x = 5;
        *r.y = 6;
    }
    println!("x={}, y={}", a.x, a.y);

    let red = Color(255, 0, 0);
    let Color(r, g, b) = red;
    println!("{}", r);
}
