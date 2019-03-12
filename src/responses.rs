use reqwest::get;
use std::fmt;
use serde::{ Serialize };
use std::error::Error;
use strum::IntoEnumIterator;

fn raw_response(query: &str, lang: Language) -> RawWikiSearchResult {
    let link = full_query(query, lang);
    let mut response = get(&link).expect("unable to fetch Wikipedia");
    response.json().expect("Unable to parse JSON")
}

fn full_query(query: &str, language: Language) -> String {
    format!("https://{}.wikipedia.org/w/api.php?action=opensearch&search={}", language, query)
}

#[derive(Debug, Copy, Clone, Serialize, EnumIter)]
pub enum Language {
    English,
    Polish,
}

pub fn all_languages() -> Vec<Language> {
    Language::iter().collect()
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Language::Polish => "pl",
            Language::English => "en",
        };
        write!(f, "{}", repr)
    }
}

pub type RawWikiSearchResult = (String, Vec<String>, Vec<String>, Vec<String>);

#[derive(Debug, Serialize)]
pub struct Article {
    pub title: String,
    pub summary: String,
    pub link: String,
    pub lang: Language,
}

#[derive(Debug, Serialize)]
pub struct WikiSearchResult {
    pub language: Language,
    pub articles: Vec<Article>,
}

impl WikiSearchResult {
    pub fn new(query: &str, lang: Language) -> Self {
        let query = full_query(query, lang);
        let response = raw_response(&query, lang);
        results(response, lang)
    }

    pub fn json(&self) -> Result<String, Box<Error>> {
        Ok(serde_json::to_string(self)?)
    }
}

pub fn results(result: RawWikiSearchResult, lang: Language) -> WikiSearchResult {
    let (_query, titles, summaries, links) = result;
    let articles = izip!(titles, summaries, links)
        .into_iter()
        .map(|(title, summary, link)| Article { title, summary, link, lang })
        .collect();
    WikiSearchResult { language: lang, articles }
}
