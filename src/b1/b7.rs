use std::fmt::Display;
use std::fmt::Debug;
pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        String::from("read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
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
}



pub fn notify1<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("{}", a.summarize())
}

pub fn notify2<T, U>(a: T, b: U) -> String
where
    T: Summary + Display, 
    U: Clone + Debug,
{
    format!("{}", a.summarize())
}


