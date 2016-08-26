use std::fmt::Debug;

#[derive(Debug)]
struct Circle {
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.1415 * (self.radius * self.radius)
    }
}

fn print_area<T: HasArea>(c: T) {
    println!("Area is {}", c.area());
}

// Where clause
fn foo<T>(x: T)
   where T: Clone + Debug {
     println!("{:?}", x.clone());
}

// Default methods
trait Foo {
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool {
        !self.is_valid()
    }
}

trait FooBar : Foo {
    fn foobar(&self);
}

fn main(){
    let c = Circle{radius: 30.0};
    println!("{:?}", c);
    print_area(c);
}
