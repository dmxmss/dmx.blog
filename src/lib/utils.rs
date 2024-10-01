use crate::lib::article::Article;
use std::{
    path::Path,
    fs::File,
    io::BufReader
};

pub fn get_articles<P: AsRef<Path>>(path: P) -> Vec<Article> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}
