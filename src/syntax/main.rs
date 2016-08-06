fn main(){
    let x: i32 = 5;
    println!("x = {:?}", x);

    // Error - x is immutable
    // x = 10;

    // confirm type
    // let hoge:() = x;

    let (x, y) = (10, 20);
    println!("x = {} y = {}", x, y);

    {
        let y: i32 = 400;
        let z: i32 = 30;
        println!("X = {} Y = {} Z = {}", x,y,z)
    }
    // Error - z is out of the scope
    //println!("X = {} Y = {} Z = {}", x,y,z)

    // 4.2 Function -------------------------------
    print_number(add_one(100));

    // Function pointer
    let f: fn(i32) -> i32 = add_one;
    print_number(f(11));

    // 4.3 Primitive type -------------------------
    let x = '👃';
    println!("{}", x);

    let x: bool = true;

    // Array
    let a: [i32; 6] = [1,2,3,4,5,6];
    println!("{:?}", a);
    println!("{:?}", a[0]);

    // Slice
    let b = &a[1..3];
    println!("{:?}", b);

    // Tuple
    let x: (i32, &str) = (11, "Hello");
    println!("{:?}",x);
    println!("{:?}",x.0);

    // 4.5 If ---------------------------------------
    let mut x = 4;
    if x == 4 {
        println!("X is 4");
    }else{
        println!("X is not 4");
    }

    let y = if x > 5 { 10 } else { 0};

    // 4.6 Loop ---------------------------------------
    while x < 10 {
        x += 1;
    }
    println!("{:?}", x);

    for (i,x) in (10..14).enumerate() {
        println!("x {:?} at {:?} loop", x, i);
    }

    for x in "A\nB\nC".lines() {
        println!("{:?}", x);
    }

    // 4.7 Vector -------------------------------------
    let v = vec![1,2,3,4,5];
    println!("{:?}", v);
    println!("{:?}", v[1]);
    println!("{:?}", v.get(3));

    match v.get(3) {
        Some(x) => println!("v[3] is {}", x),
        None    => println!("v[3] is none")
    };

    for i in &v {
        println!("{:?}", i);
    }
    for i in &v {
        println!("{:?}", i);
    }

}

fn print_number(x: i32) {
    println!("x is {}", x);
}

fn add_one(x: i32) -> i32{
    x + 1
}

fn diverges() -> ! {
    panic!("This function never return");
}
