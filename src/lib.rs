use std::fmt::{format, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("{}님의 기사 더 읽기", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: impl Summary) {
    println!("속보! {}", item.summarize())
}

pub fn notify2<T: Summary>(item1: T, item2: T) {
    todo!()
}

pub fn notify3_1(item: impl Summary + Display) {
    todo!()
}

pub fn notify3_2<T: Summary + Display>(item: T) {
    todo!()
}

fn return_summarizable() -> impl Summary {
    // 한가지의 객체종류만 리턴 가능함. 조건절로 분기 안됨
    // 454page 참조 필요
    Tweet {
        username: String::from("abcd"),
        content: String::from("eeeee"),
        reply: false,
        retweet: false,
    }
}