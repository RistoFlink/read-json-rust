use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String
}
#[derive(Deserialize, Serialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let json = r#"
    {
        "article": "How to work with JSON in Rust",
        "author": "RF",
        "paragraph": [
            {
                "name": "Start of the paragraph."
            },
            {
                 "name": "Body of the paragraph."
            },
            {
                "name": "End of the paragraph."
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);
    println!("\n\n The name of the first paragraph is: {}", parsed.paragraph[0].name);
}
fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed
}
