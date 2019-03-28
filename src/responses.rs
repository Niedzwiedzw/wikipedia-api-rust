use reqwest::get;
use std::fmt;
use std::iter::FromIterator;
use serde::{ Serialize, Deserialize };
use std::error::Error;
use strum::IntoEnumIterator;
use crate::response_schemas::Root;
use crate::language::Language;


fn raw_opensearch_respnose(query: &str, lang: Language) -> RawWikiSearchResult {
    let link = opensearch_query(query, lang);
    let mut response = get(&link).expect("unable to fetch Wikipedia");
    response.json().expect("Unable to parse JSON")
}

fn raw_language_links_response(title: &str, lang: Language) -> Root {
    let link = language_links_query(title, lang);
    let mut response = get(&link).expect("unable to fetch langlinks");
    response.json().expect("unable to parse string...")
}

fn opensearch_query(query: &str, language: Language) -> String {
    format!("https://{}.wikipedia.org/w/api.php?action=opensearch&search={}&prop=langlinks", language, query)
}

fn language_links_query(title: &str, language: Language) -> String {
    format!(
        "https://{}.wikipedia.org/w/api.php?action=query\
        &format=json\
        &prop=langlinks\
        &meta=\
        &continue=\
        &titles={}\
        &utf8=1\
        &lllimit=500", language, title)
}

pub fn all_languages() -> Vec<Language> {
    Language::iter().collect()
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = serde_json::to_string(&self).expect("failed to serialize language");
        let repr = repr
            .chars()
            .filter(|&b| b != '\\' && b != '"');
        let repr = String::from_iter(repr);
        write!(f, "{}", dbg!(repr))
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

impl Article {
    pub fn language_links(&self) -> Root {
        raw_language_links_response(&self.title, self.lang)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageLink {
    pub lang: Language,
    #[serde(alias = "*")]pub title: String,
}

#[derive(Debug, Serialize)]
pub struct WikiSearchResult {
    pub language: Language,
    pub articles: Vec<Article>,
}

impl WikiSearchResult {
    pub fn new(query: &str, lang: Language) -> Self {
        let query = opensearch_query(query, lang);
        let response = raw_opensearch_respnose(&query, lang);
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
