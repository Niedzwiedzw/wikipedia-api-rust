use serde::{ Serialize, Deserialize };
use std::collections::HashMap;
use crate::responses::LanguageLink;

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    pub batchcomplete: String,
    pub query: Query
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub pages: HashMap<u32, Page>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pageid: u32,
    ns: u32,
    title: String,
    langlinks: Vec<LanguageLink>
}
