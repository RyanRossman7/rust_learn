// Traits (**Similar to interfaces in Java**)
//- Traits define functionality a particular type has and can share with other types.
//- We can use traits to define shared behavior in an abstract way. 
//- We can use trait bounds to specify that a generic type can be any type that has certain behavior.
pub struct NewsArticle {
    pub author: String, 
    pub headline: String, 
    pub content: String, 
}
impl Summary for NewsArticle {}
// It'll use default =)
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
// A trait method is able to access other methods within that trait.
// We will use a triat to define the shared behavior between tweet/news article
pub trait Summary {
    fn summarize(&self) -> String {
        // This sets defualt implementations 
        String::from("Read More...")
    }
}
pub fn run() {
    let tweet = Tweet { 
        username: String::from("@Hy0sh7"), 
        content: String::from("Hello_Rust"),
        reply: false,
        retweet: false, 
    };
    let article = NewsArticle {
        author: String::from("Hamato Yoshi"), 
        headline: String::from("The World is F*ck*D"),
        content: String::from("But Seriously")
    }; 
    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

}