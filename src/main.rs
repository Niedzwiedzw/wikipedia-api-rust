mod responses;
#[macro_use] extern crate itertools;
use crate::responses::WikiSearchResult;

use crate::responses::Language;

fn main() {
    let query = "black";
    let lang = Language::English;
    let result = WikiSearchResult::new(query, lang);
    println!("{:#?}", result);
}
