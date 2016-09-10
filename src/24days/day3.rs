extern crate csv;
extern crate rustc_serialize;

#[derive(RustcDecodable, RustcEncodable)]
struct User {
    full_name: String,
    name: String,
    age: usize,
}

fn main(){
    let users = vec![
        User{
            full_name: "Taro Tanaka".to_owned(),
            name: "taro".to_owned(),
            age: 1982
        },
        User{
            full_name: "Tako Ken".to_owned(),
            name: "ken".to_owned(),
            age: 1989
        },
    ];
    let path = "tmp/users.csv";

    let mut writer = csv::Writer::from_file(path).unwrap();
    for user in users {
        writer.encode(user).expect("CSV write error");
    }
    writer.flush().expect("Flush error");

    let mut reader = csv::Reader::from_file(path).unwrap().has_headers(false);
    for row in reader.decode() {
        let row: (String, String, usize) = row.unwrap();
        println!("{:?}", row);
    }

    let mut reader = csv::Reader::from_file(path).unwrap().has_headers(false);
    for row in reader.decode() {
        let user: User = row.unwrap();
        println!("{}, {}, {}", user.full_name, user.name, user.age);
    }
}
