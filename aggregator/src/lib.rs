use std::fmt::Display


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}


pub trait Summary {
    fn summarize_author(&self) -> String {
        String::from("Author")
    }

    fn summarize(&self) -> String {
        String::from("(Read more..)")
    }

    // fn notify(&self) -> String {
    //     format!("Breaking news: {}", self.summarize())
    // }
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// trait bound 
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("tylardurten"),
        content: String::from("The thing you own ended up owning you"),
        reply: false,
        retweet: false,
    }
}
