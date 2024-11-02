use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("Article"),
        author: String::from("Author"),
        paragraph: vec![
            Paragraph {
                name: String::from("Paragraph 1"),
            },
            Paragraph {
                name: String::from("Paragraph 2"),
            },
            Paragraph {
                name: String::from("Paragraph 3"),
            },
        ],
    };
    let json = serde_json::to_string(&article).unwrap();
    println!("{}", json);
}
