use serde::{Deserialize, Serialize};

use std::collections::BTreeMap as Map;

#[derive(Serialize, Deserialize)]
struct Resource {
    name: String,
    #[serde(skip_serializing)]
    hash: String,
    #[serde(skip_serializing_if = "Map::is_empty")]
    metadata: Map<String, String>,
}
fn main() {
    let resources = vec![
        Resource {
            name: "Stack Overflow".to_string(),
            hash: "b6469c3f31653d281bbbfa6f94d60fea130abe38".to_string(),
            metadata: Map::new(),
        },
        Resource {
            name: "GitHub".to_string(),
            hash: "5cb7a0c47e53854cd00e1a968de5abce1c124601".to_string(),
            metadata: {
                let mut metadata = Map::new();
                metadata.insert("headquarters".to_string(), "San Francisco".to_string());
                metadata
            },
        },
    ];

    let json = serde_json::to_string_pretty(&resources).unwrap();

    println!("{}", json);

    // Code panics!
    // let resources: Vec<Resource> = serde_json::from_str(&json).unwrap();
}
