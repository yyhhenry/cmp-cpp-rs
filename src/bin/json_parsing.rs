use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    authors: Vec<Author>,
}

fn main() {
    let config = Config {
        name: "test".to_owned(),
        authors: vec![
            Author {
                name: "Test Me".to_owned(),
                email: "me@test.com".to_owned(),
            },
            serde_json::from_value(json!({
                "name": "Test You",
                "email": "you@test.com",
            }))
            .unwrap(),
        ],
    };
    println!("config: {:?}", config);
    let serialized = serde_json::to_string(&config).unwrap();
    println!("serialized: {}", serialized);
    let deserialized: Config = serde_json::from_str(&serialized).unwrap();
    println!("deserialized: {:?}", deserialized);
    println!(
        "deserialized author names: {:?}",
        deserialized
            .authors
            .iter()
            .map(|a| a.name.to_owned())
            .collect::<Vec<_>>()
    );
}
