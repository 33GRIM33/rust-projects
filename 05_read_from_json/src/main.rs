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
    let json1 = r#"
    {
        "article": "how to work with json in rust",
        "author": "name1",
        "paragraphs": [
            {
                "name": "starting scentence"
            },
            {
                "name": "second sentence"
            },
            {
                "name": "third sentence"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json1);
    println!(
        "The name of the first paragraph is: {}\n",
        parsed.paragraphs[0].name
    );
    println!(
        "The name of the second paragraph is: {}\n",
        parsed.paragraphs[1].name
    );
    println!(
        "The name of the third paragraph is: {}\n",
        parsed.paragraphs[2].name
    );
}


fn read_json_typed(json: &str)->Article{
    let parsed: Article = serde_json::from_str(json).unwrap();
    parsed

}