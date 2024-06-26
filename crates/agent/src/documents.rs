

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Document {
    pub url: String,
    pub title: String,
    pub id : u32,
}

const DOCS_TEST_PATH: &str = ".\
\
/static/testdata.json";

//read our documents.json file
pub async  fn read_documents() -> Vec<Document> {
    let file = std::fs::read_to_string(DOCS_TEST_PATH).expect("could not read file");
    let documents = serde_json::from_str(&file).expect("error parsing json");
    documents
}