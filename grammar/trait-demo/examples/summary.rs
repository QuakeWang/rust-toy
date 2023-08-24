pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("title: {}, author: {}, content: {}", self.title, self.author, self.content)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} posted on Weibo: {}", self.username, self.content )
    }
}

fn main() {
    let post = Post{title: "Rust Toy".to_string(),author: "QuakeWang".to_string(), content: "Rust is awesome!!!".to_string()};
    let weibo = Weibo{username: "QuakeWang".to_string(),content: "bad app".to_string()};

    println!("{}",post.summarize());
    println!("{}",weibo.summarize());
}