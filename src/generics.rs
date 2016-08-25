enum Res<T, E> {
    Ok(T),
    Err(E)
}

fn left<T>(x: T, y: T) -> T {
    x
}

struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}

fn main(){
    // Generics
    let x: Option<i32> = Some(5);
}
