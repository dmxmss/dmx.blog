use std::{
    fs::File,
    io::Write,
    path::Path
};
use crate::lib::{
    result::Result,
    article::{Article, NewArticle},
    utils::get_articles
}; 

pub struct Cursor {
    file: File,
    pub articles: Vec<Article>
}

impl Cursor {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Cursor> {
        let articles = get_articles(&path)?;

        let mut file = File::create(&path)?;
        file.write_all(serde_json::to_string(&articles)?.as_bytes())?;

        Ok(Cursor { file, articles })
    }

    pub fn get_article(&self, id: u64) -> Option<Article> {
        self.articles.iter().find(|article| article.id == id).cloned()
    }

    pub fn create_article(&mut self, article: NewArticle) -> Result<u64> {
        let id = self.articles.iter().map(|a| a.id).max().unwrap() + 1;
        let article = Article::new(id, article.name, article.contents);
        self.articles.push(article);

        self.sync()?;

        Ok(id)
    }

    fn sync(&mut self) -> Result<()> {
        self.file.write_all(serde_json::to_string(&self.articles)?.as_bytes())?;

        Ok(())
    }

    pub fn delete_article(&mut self, id: u64) -> Result<()> {
        if let Some(article) = self.get_article(id) {
            self.articles.retain(|a| a.id != article.id);
        }

        self.sync()?;

        Ok(())
    }

    pub fn update_article(&mut self, id: u64, article: NewArticle) -> Result<()> {
        self.delete_article(id)?;

        let article = Article::new(id, article.name, article.contents);
        self.articles.push(article);

        self.sync()?;

        Ok(())
    }
}
