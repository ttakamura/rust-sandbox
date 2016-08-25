fn main(){
    // &str = string slices
    // 固定サイズでimmutable, UTF-8 bytes への reference
    let greeting: &str = "Hello there";
    println!("{}", greeting);

    // String = heap-allocated sring
    let mut s: String = greeting.to_string();
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);

    // Each Bytes and Chars
    let hachiko = "忠犬ハチ公";

    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }
    println!("");

    for c in hachiko.chars() {
        print!("{}, ", c);
    }
    println!("");

    // Slicing
    let dog   = "忠犬ハチ公";
    let hachi = &dog[6..12];
    println!("{}", hachi);

    // Concat
    println!("{}", ("Hello".to_string() + "World"));
}
