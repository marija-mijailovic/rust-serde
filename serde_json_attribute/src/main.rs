use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Token {
    token_id: u32,
    token_name: String,
    #[serde(default = "default_decimals")]
    #[serde(skip_deserializing)]
    default_decimals: u32,
    is_process: bool,
    amount: u32,
}

fn default_decimals() -> u32 {
    18
}

fn main() {
    let token = Token {
        token_id: 1,
        token_name: "Marija".to_owned(),
        default_decimals: 8,
        is_process: true,
        amount: 100,
    };

    let serialize = serde_json::to_string(&token).unwrap();
    println!("{}", serialize);

    let deserialize: Token = serde_json::from_str(&serialize).unwrap();
    println!("{:?}", deserialize);
}
