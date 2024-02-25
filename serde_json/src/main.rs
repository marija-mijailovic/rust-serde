use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    id: u32,
    name: String,
    process: bool,
}

fn main() {
    let token = Token {
        id: 1,
        name: "Marija".to_owned(),
        process: true,
    };

    let serialize = serde_json::to_string(&token).unwrap();
    println!("{}", serialize);

    let deserialize: Token = serde_json::from_str(&serialize).unwrap();
    println!("{:?}", deserialize);
}
