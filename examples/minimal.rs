use wikiapi::responses::{ WikiSearchResult, Language };

fn main() {
    let query = "black";
    let lang = Language::English;
    let result = WikiSearchResult::new(query, lang);
    println!("{:#?}", result);
}
