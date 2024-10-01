use serde::{Deserialize, Serialize};
use chrono::offset::Utc;

#[derive(Serialize, Deserialize)]
pub struct Article {
    id: u64,
    name: String,
    contents: String,
    pub_date: String,
    edit_date: String
}

impl Article {
    pub fn new(id: u64, name: String, contents: String) -> Article {
        Article {
            id, 
            name, 
            contents, 
            pub_date: Utc::now().to_string(), 
            edit_date: Utc::now().to_string()
        }
    }
}
