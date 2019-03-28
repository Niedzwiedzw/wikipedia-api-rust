use std::error::Error;
use wikiapi::responses::{ WikiSearchResult, all_languages };

fn main() -> Result<(), Box<Error>> {
    let query = "Waldemar Sierański";
    for lang in all_languages() {
        let result = WikiSearchResult::new(query, lang);
        println!("{:#?}", result);

        let article = &result.articles[0];
        println!("example article: {:#?}", article);

        println!("...and it's language links:");

        let language_links = article.language_links();
        println!("{:?}", language_links);

    }
    Ok(())
}
