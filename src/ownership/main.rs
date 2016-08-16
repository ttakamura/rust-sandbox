fn takeMove(v: Vec<i32>){
    println!("{:?}", v);
}

fn takeBorrow(v: &Vec<i32>){
    // v.push(10); // error
    println!("{:?}", v);
}

fn sum_vec(v: &Vec<i32>) -> i32 {
    return v.iter().fold(0, |a, &b| a + b);
}

fn main(){
    // Ownership -----------------------------
    let v = vec![1,2,3];
    println!("{:?}", v);

    // move the ownership from v to v2
    let v2 = v;
    println!("{:?}", v2);

    // v から v2 に移動しているのでエラー
    //println!("{:?}", v);

    // 関数呼び出しでも移動する
    takeMove(v2);
    // println!("{:?}", v2); // <- also error

    // i32 は Copy trait を実装しているので move ではない
    let v = 1;
    let v2 = v;
    println!("{:?} == {:?}", v, v2);

    // Borrowin ---------------------------------
    println!("reference");
    // &v で ownership が move ではなく reference となる
    let mut v = vec![6,7,8];
    v.push(9);
    takeBorrow(&v);
    println!("{:?}", sum_vec(&v));

    println!("{:?}", v); // Ownerは移動していない

    // mut& reference ---------------------------
    println!("mut reference");
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{:?}", x);

    let mut v = vec![11,22,33];
    for i in &v {
        println!("{}",i);
        // v.push(99); // iterator が reference を持っているのでエラー
    }

    // use after free ---------------------------
    // let y: &i32; // Error
    {
        let x = 9;
        let y: &i32;
        y = &x;
        println!("{}",y);
    }

    println!("done");
}
