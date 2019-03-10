use std::error::Error;
use wikiapi::responses::{ WikiSearchResult, Language };

fn main() -> Result<(), Box<Error>> {
    let query = "black";
    let lang = Language::English;
    let result = WikiSearchResult::new(query, lang);
    println!("{}", result.json()?);
    Ok(())
}
