use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum TokenType {
    Native,
    SBT,
    NFT,
    ERC20,
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    id: u32,
    name: String,
    process: bool,
    token_type: TokenType,
}

fn main() {
    let token = Token {
        id: 1,
        name: "Marija".to_owned(),
        process: true,
        token_type: TokenType::NFT,
    };

    let serialize = serde_json::to_string(&token).unwrap();
    println!("{}", serialize);

    let deserialize: Token = serde_json::from_str(&serialize).unwrap();
    println!("{:?}", deserialize);
}
