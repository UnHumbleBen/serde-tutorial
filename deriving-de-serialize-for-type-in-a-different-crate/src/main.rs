mod other_crate {
    #[derive(Debug)]
    pub struct Duration {
        secs: i64,
        nanos: i32,
    }

    impl Duration {
        pub fn new(secs: i64, nanos: i32) -> Self {
            Duration { secs, nanos }
        }

        pub fn seconds(&self) -> i64 {
            self.secs
        }

        pub fn subsec_nanos(&self) -> i32 {
            self.nanos
        }
    }
}

////////////////////////////////////////////////////

use other_crate::Duration;
use serde::{Deserialize, Serialize};

/// Serde calls this the definition of the remote type. It is just a copy of
/// the remote data structure. The `remove` attribute gives the path the actual
/// type we intend to derive code for.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Duration")]
struct DurationDef {
    #[serde(getter = "Duration::seconds")]
    secs: i64,
    #[serde(getter = "Duration::subsec_nanos")]
    nanos: i32,
}

impl From<DurationDef> for Duration {
    fn from(def: DurationDef) -> Duration {
        Duration::new(def.secs, def.nanos)
    }
}

/// Now the remote type can be used almost like it had its own Serialize and
/// Deserialize impls all along. The `with` attribute gives the path to the
/// definition for the remote type. Note that the real type of the field is the
/// remote type, not the definition type.
#[derive(Serialize, Deserialize)]
struct Process {
    command_line: String,

    #[serde(with = "DurationDef")]
    wall_time: Duration,
}

#[derive(Deserialize)]
struct Helper(#[serde(with = "DurationDef")] Duration);

fn main() {
    // let duration = DurationDef { secs: 42, nanos: 1234};

    // let j = serde_json::to_string_pretty(&duration).unwrap();
    let j = r#"
        {
            "secs": 42,
            "nanos": 1321
        }
    "#;
    println!("{}", j);
    let mut de = serde_json::Deserializer::from_str(&j);
    let dur = DurationDef::deserialize(&mut de).unwrap();

    println!("{:?}", dur);

    let dur = serde_json::from_str(j).map(|a: Helper| a.0).unwrap();
    println!("{:?}", dur);
}
