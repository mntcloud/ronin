use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::objects::block::Block;
use crate::misc::Object;

#[derive(Serialize, Deserialize)]
pub struct Page {
	object: String,
	id: String,
	created_time: String,
	last_edited_time: String,
	archived: bool,
	parent: PageParent,
	properties: Object<Value>
}

impl Page {
	pub fn get_id(&self) -> &str {
		&self.id
	}
}

#[derive(Serialize, Deserialize)]
pub struct PageParent {
	#[serde(rename = "type")]
	spec: String,

	database_id: Option<String>,
	page_id: Option<String>
}

#[derive(Serialize)]
pub struct UpdatePage {
	properties: Object<Value>,
	archived: bool
}

// Page template for creating new pages
#[derive(Serialize)]
pub struct PageTemplate {
	parent: PageParent,
	properties: Object<Value>,
	children: Option<Vec<Block>>
}