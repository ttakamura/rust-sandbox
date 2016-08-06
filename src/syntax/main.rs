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

    println!("Done");
}
