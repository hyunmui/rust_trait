use rust_trait::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("I study rust language"),
        reply: false,
        retweet: false,
    };

    println!("One of New tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("대한민국, 러시아 월드컵 예선에서 독일을 이겼다."),
        location: String::from("카잔 아레나, 러시아"),
        author: String::from("위키백과"),
        content: String::from(
            "2018년 6월 27일 러시아 카잔의 카잔 아레나에서 열린 
            2018 월드컵 F조 3차전 경기에서 대한민국이 독일에 2:0 승리를 거뒀다.",
        ),
    };

    println!("New Article: {}", article.summarize());
}
