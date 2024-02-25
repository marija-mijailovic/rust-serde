use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Token {
    token_id: u32,
    token_name: String,
    #[serde(default = "default_decimals")]
    default_decimals: u32,
    #[serde(skip_serializing_if = "not_is_process")]
    is_process: bool,
    amount: u32,
}

fn default_decimals() -> u32 {
    18
}

fn not_is_process(value: &bool) -> bool {
    !*value
}

fn main() {
    let token = vec![
        Token {
            token_id: 1,
            token_name: "Marija".to_owned(),
            default_decimals: 8,
            is_process: false,
            amount: 100,
        },
        Token {
            token_id: 2,
            token_name: "Mami".to_owned(),
            default_decimals: 6,
            is_process: true,
            amount: 100,
        },
    ];

    let serialize = serde_json::to_string(&token).unwrap();
    println!("{}", serialize);
}
