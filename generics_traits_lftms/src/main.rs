// mod generics;
// mod in_struct_definitions;
// use generics::*;
use aggregator::{self, NewsArticle, Summary, Tweet};

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    println!("New article availble: {}", article.summarize());
}
