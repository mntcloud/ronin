use serde::Deserialize;

#[derive(Deserialize)]
pub struct Users {
    results: Vec<User>
}

#[derive(Deserialize)]
pub struct User {
    object: String,
    id: String, 

    #[serde(rename = "type")]
    spec: String,

    person: Option<Person>,
    name: String,
    avatar_url: String
}

#[derive(Deserialize)]
pub struct Person {
    email: String
}