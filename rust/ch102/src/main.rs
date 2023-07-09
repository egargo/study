use uuid::Uuid;

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more...)")
    }
    fn summarize_author(&self) -> String;
    fn summarize_id(&self) -> Uuid;
}

pub struct NewsArticle {
    pub id: Uuid,
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize_id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub id: Uuid,
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

    fn summarize_id(&self) -> Uuid {
        self.id
    }
}

fn main() {
    let article = NewsArticle {
        id: Uuid::new_v4(),
        author: "Bee".to_string(),
        content: "This is a content".to_string(),
        headline: "This is a headline".to_string(),
        location: "PH".to_string(),
    };


    let tweet = Tweet {
        id: Uuid::new_v4(),
        username: "Bee".to_string(),
        content: "This is a tweet.".to_string(),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());
    println!("{}", tweet.summarize_id());
    println!("{}", tweet.summarize_author());

    println!("{}", article.summarize());
    println!("{}", article.summarize_id());
    println!("{}", article.summarize_author());
}
