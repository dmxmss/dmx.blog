use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Article {
    id: u64,
    name: String,
    contents: String,
    pub_date: String,
    edit_date: String
}
