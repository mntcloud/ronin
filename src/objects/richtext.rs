use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct RichText {
    plain_text: String,
    href: Option<String>,
    annotations: Annotations,
    
    #[serde(rename = "type")]
    spec: String,

    text: Option<Text>,
    mention: Option<Mention>,
    equation: Option<Equation>
}

#[derive(Serialize, Deserialize)]
struct Annotations {
    bold: bool,
    italic: bool,
    strikethrough: bool,
    underline: bool,
    code: bool,
    color: String
}

#[derive(Serialize, Deserialize)]
struct Text {
    content: String,
    link: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Mention {

    #[serde(rename = "type")]
    spec: String,

    user: Value,
    page: Value,
    database: Value,
    date: Value
}

#[derive(Serialize, Deserialize)]
struct Equation {
    expression: String,
}