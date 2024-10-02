use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub id: u64,
    pub name: String,
    pub contents: String,
    pub pub_date: String,
    pub edit_date: String
}
