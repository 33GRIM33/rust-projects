use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}

fn main() {
    let article = Article{
        article: "how to work with json in Rust".to_string(),
        author: "name1".to_string(),
        paragraphs: vec![
            Paragraph{name: "Starting sentence 1".to_string()},
            Paragraph{name: "Second sentence 2".to_string()},
            Paragraph{name: "Third sentence 3".to_string()}
        ]
    };
    let json = serde_json::to_string(&article).unwrap();
    
    println!("{}",json);


}
