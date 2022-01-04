use rust_trait::{Tweet, Summary};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("I study rust language"),
        reply: false,
        retweet: false,
    };

    println!("One of New tweet: {}", tweet.summarize());
}
