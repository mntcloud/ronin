use serde::{Serialize, Deserialize};
use crate::objects::richtext::RichText;
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize)]
pub struct Database {
    object: String,
    id: String,
    created_time: String,
    last_edited_time: String,
    title: Vec<RichText>,
    properties: Map<String, Value>
}

impl Database {
    pub fn get_id(&self) -> &str {
        &self.id
    }
}
/*
#[derive(Serialize, Deserialize)]
pub struct Property {
    id: String,

    #[serde(rename = "type")]
    spec: String,
}
*/


// TODO: this
/*
#[derive(Serialize)]
pub struct CompoundFilter {
    and: Vec<Filter>,
    or: Vec<Filter>
}

#[derive(Serialize)]
pub struct Filter {
    property: String,
    text: TextFilter,
    number: NumberFilter,
    checkbox: CheckboxFilter,
    select: SelectFilter,
    multi_select: MultiSelectFilter,
    date: DateFilter,
    created_time: DateFilter,
    last_edited_time: DateFilter,
    people: PeopleFilter,
    files: FileFilter,
    relation: RelationFilter,
    formula: FormulaFilter,
}

#[derive(Serialize)]
pub struct TextFilter {
    equals: String,
    does_not_equal: String,
    contains: String,
    does_not_contain: String,
    starts_with: String,
    ends_with: String,
    is_empty: bool,
    is_not_empty: bool
}

#[derive(Serialize)]
pub struct NumberFilter {
    equals: i32,
    does_not_equal: i32,
    greater_than: i32,
    less_than: i32,
    greater_than_or_equal_to: i32,
    less_than_or_equal_to: i32,
    is_empty: i32,
    is_not_empty: i32
}

*/