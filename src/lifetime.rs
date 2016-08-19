fn foo<'a>(x: &'a mut i32){
    *x += 1;
    println!("{}", x);
}

struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 {
        self.x
    }
}

fn main(){
    let mut x = 100;
    println!("{}", x);
    foo(&mut x);
    println!("{}", x);

    {
        let y = &5;
        let a;   // y より後に定義する必要がある
        let f;   // y より後に定義する必要がある
        f = Foo { x: y };
        println!("{}", f.x());
        a = f.x();
    }
}
