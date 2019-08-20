//! Default value for a field
//!
//! Source: [https://serde.rs/attr-default.html](https://serde.rs/attr-default.html)

use serde::Deserialize;

/// A Request.
#[derive(Deserialize, Debug)]
struct Request {
    /// Uses the result of a function as the default if "resouce" is not
    /// included in the input.
    #[serde(default = "default_resource")]
    resource: String,

    /// Uses the type's implementation of `std::default::Default` if "timeout"
    /// is not included in the input.
    #[serde(default)]
    timeout: Timeout,

    /// Uses a method from the type as the default if "priority" is not
    /// included in the input. This may also be a trait method.
    #[serde(default = "Priority::lowest")]
    priority: Priority,
}

/// Returns the default resource.
fn default_resource() -> String {
    "/".to_string()
}

/// Timeout in seconds.
#[derive(Deserialize, Debug)]
struct Timeout(u32);
impl Default for Timeout {
    fn default() -> Self {
        Timeout(30)
    }
}

/// Priority level of the `Request`.
#[derive(Deserialize, Debug)]
enum Priority {
    ExtraHigh,
    High,
    Normal,
    Low,
    ExtraLow,
}
impl Priority {
    fn lowest() -> Self {
        Priority::ExtraLow
    }
}

fn main() {
    // Use raw string literal because there are too many characters to
    // escape.
    let json = r#"
        [
            {
                "resource": "/users"
            },
            {
                "timeout": 5,
                "priority": "High"
            } 
        ]
    "#;

    let requests: Vec<Request> = serde_json::from_str(json).unwrap();

    println!("First  request:\n{:?}", requests[0]);
    println!("Second request:\n{:?}", requests[1]);
}
