use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Pagination {
    limit: u64,
    offset: u64,
    total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Users {
    users: Vec<User>,

    #[serde(flatten)]
    pagination: Pagination,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: String,
    username: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn main() {
    let page = r#"
        {
            "limit": 100,
            "offset": 200,
            "total": 1053,
            "users": [
                {"id": "49824073-979f-4814-be10-5ea416ee1c2f", "username": "john_doe", "mascot": "Ferris"},
                {"id": "84820495-859j-9302-kd20-3kl303js2d2d", "username": "bruce_wayne"}
            ]
        }
    "#;

    let users: Users = serde_json::from_str(page).unwrap();
    println!("users = {:?}", users);
}
