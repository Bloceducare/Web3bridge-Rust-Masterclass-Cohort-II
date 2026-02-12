// solidity guys interface to explain traits
// function view() public view returns uint;
//
// python
// def view()-> int;

use std::fmt::format;

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// {
//         let our_string = "Summarize".to_string();
//         format!("{}", our_string)
//     }
pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for (SocialPost, NewsArticle) {
    fn summarize(&self) -> String {
        format!(
            "{}, {}, {},{}",
            self.1.headline, self.0.username, self.0.content, self.1.author
        )
    }
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}, {}, {}", self.username, self.content, self.repost)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} {} {}", self.headline, self.author, self.content)
    }
}

fn main() {
    let post = SocialPost {
        username: String::from("Qubzes"),
        content: String::from("W3B"),
        repost: true,
        reply: true,
    };
    let article = NewsArticle {
        headline: String::from("Breaking news"),
        location: String::from("Ikorodu"),
        author: String::from("Qubzes"),
        content: String::from("w3b"),
    };

    let post_summary = (post, article).summarize();
    // let article = article.summarize();

    println!("Post summary: {post_summary}");
    // println!("Post summary: {article}");
}
