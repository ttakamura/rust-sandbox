use std::sync::Arc;
use std::cell::RefCell;

struct Point {
    x: i32,
    y: i32
}

fn main(){
    // Mutability
    let mut x = 5;
    let y = &mut x;
    *y = 20;
    println!("{}", y);

    // Interior vs Exterior
    // In Arc's case, the mutation is entirely contained inside the structure itself.
    let x = Arc::new(5);
    let y = x.clone();
    println!("{:?}", y);

    // RefCell check borrowing rule at runtime
    let x = RefCell::new(42);
    let y = x.borrow_mut();

    // So, panic! it at runtime. ----
    // let z = x.borrow_mut();
    // println!("{:?}", z);

    // Struct cannot have some fields mutable and some immutable
    // The mutability of a struct is in its binding.
    let mut a = Point { x: 5, y: 6 };
    println!("x={}, y={}", a.x, a.y);
    a.x = 10;
    println!("x={}, y={}", a.x, a.y);
}
