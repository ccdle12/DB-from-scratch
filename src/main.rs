use serde::{Deserialize, Serialize};
mod error;
use error::Result;

mod db;
use db::DBDriver;

const OUTPUT_DIR: &str = "./db";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct User {
    pub id: u128,
    pub name: String,
    pub age: u8,
}

fn main() -> Result<()> {
    let db = DBDriver::new(OUTPUT_DIR);

    let user_0 = User {
        id: 0,
        name: "John".into(),
        age: 20,
    };
    let user_json = serde_json::to_string(&user_0)?;

    db.write(user_0.id, user_json.as_bytes())?;

    Ok(())
}
