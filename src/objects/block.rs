use serde::{Serialize, Deserialize};
use crate::objects::richtext::RichText;

#[derive(Serialize, Deserialize)]
pub struct Blocks {
    object: String,
    results: Vec<Block>,
    next_cursor: Option<i32>,
    has_more: bool
}

#[derive(Serialize, Deserialize)]
pub struct AppendBlocks {
    children: Vec<Block>
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    object: String,
    id: String,
    
    #[serde(rename = "type")]
    spec: String,

    created_time: String,
    last_edited_time: String,
    has_children: bool,
    paragraph: Option<Paragraph>,
    heading_1: Option<H1>,
    heading_2: Option<H2>,
    heading_3: Option<H3>,
    bulleted_list_item: Option<BulletedListItem>,
    numbered_list_item: Option<NumberedListItem>,
    to_do: Option<ToDo>,
    toggle: Option<Toggle>,
    child_page: Option<ChildPage>,
}

#[derive(Serialize, Deserialize)]
struct Paragraph {
    text: Vec<RichText>,
    children: Vec<Block>
}

#[derive(Serialize, Deserialize)]
struct H1 {
    text: Vec<RichText>
}

#[derive(Serialize, Deserialize)]
struct H2 {
    text: Vec<RichText>
}

#[derive(Serialize, Deserialize)]
struct H3 {
    text: Vec<RichText>
}

#[derive(Serialize, Deserialize)]
struct BulletedListItem {
    text: Vec<RichText>,
    children: Vec<Block>
}

#[derive(Serialize, Deserialize)]
struct NumberedListItem {
    text: Vec<RichText>,
    children: Vec<Block>
}

#[derive(Serialize, Deserialize)]
struct ToDo {
    text: Vec<RichText>,
    checked: bool,
    children: Vec<Block>
}

#[derive(Serialize, Deserialize)]
struct Toggle {
    text: Vec<RichText>,
    children: Vec<Block>
}

#[derive(Serialize, Deserialize)]
struct ChildPage {
    title: String
}