use std::error::Error;
use wikiapi::responses::{ WikiSearchResult, all_languages };

fn main() -> Result<(), Box<Error>> {
    let query = "black";
    for lang in all_languages() {
        let result = WikiSearchResult::new(query, lang);
        println!("{}", result.json()?);
    }
    Ok(())
}
