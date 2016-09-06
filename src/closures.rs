fn main() {
    // borrow reference
    let mut num = 5;
    {
        let mut plus_num = |x: i32| num += x;
        plus_num(100);
    }
    println!("{}", num);

    // copy num
    let mut num = 5;
    {
        let mut plus_num = move |x: i32| num += x;
        plus_num(100);
    }
    println!("{}", num);
}
