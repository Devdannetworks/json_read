use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Article {
    title: String,
    body: String,
    footer: Vec<Paragraph>,
}

fn main() {
    let json = r#"{
     "title": "This is my Json title",
     "body": "THis is my json body",
     "footer": [
       {
       
        "name": "This is my first paragraph name"
       },
       {
        "name": "This is my second paragraph name"
       },
       {
        "name": "This is my third paragraph name"
       }
     ]
  }"#;

    //serde_json::from-str() deserializes the json into the corresponding rust data structure
    let parsed: Article = serde_json::from_str(&json).unwrap();

    println!(
        " The first name of the paragraph is: {}",
        parsed.footer[0].name
    );
}
