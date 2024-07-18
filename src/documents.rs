use serde::{Deserialize, Serialize};
use axum::extract::Path as AxumPath;
use chrono::prelude::*;
use askama_axum::IntoResponse;
use crate::templates::DocumentDetailsTemplate;
#[derive(Deserialize)]
pub struct DocumentCard {
    pub title: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Document {
    pub url: String,
    pub title: String,
    pub id: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DocumentInsight {
    pub document: Document,
    pub insight: String,
    pub id : u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DocumentMessage {
    pub from: String,            // Who sent the message (e.g., user, AI)
    pub date: String,            // Date and time of the message
    pub id: u32,                 // Unique identifier for the message
    pub content: String,         // The actual content of the message
    pub document_id: u32, // Specific part of the document being referenced (optional)
}


const DOCS_TEST_PATH: &str = "./test-data/testdata.json";

// Function to get the current timestamp in a readable format
fn current_timestamp() -> String {
    Utc::now().to_rfc3339()
}


//read our documents.json file
pub async fn read_documents() -> Vec<Document> {
    let file = std::fs::read_to_string(DOCS_TEST_PATH).expect("could not read file");
    let documents = serde_json::from_str(&file).expect("error parsing json");
    documents
}

// Handler to view a document and its chat
pub async fn view_document(AxumPath(id): AxumPath<u64>) -> impl IntoResponse {
    let dummy_document = Document {
        url: "https://pdfobject.com/pdf/sample.pdf".to_string(),
        title: "Example Document".to_string(),
        id: id as u32,
    };
    let dummy_chat = vec![
        DocumentMessage {
            from: "User".to_string(),
            date: current_timestamp(),
            id: 1,
            content: "Can you summarize the introduction of the document?".to_string(),
            document_id: 101,
        },
        DocumentMessage {
            from: "AI".to_string(),
            date: current_timestamp(),
            id: 2,
            content: "Sure! The introduction provides an overview of the document's purpose and outlines the main topics that will be discussed.".to_string(),
            document_id: 101,
        },
        DocumentMessage {
            from: "User".to_string(),
            date: current_timestamp(),
            id: 3,
            content: "What are the key findings in the second chapter?".to_string(),
            document_id: 102,
        },
        DocumentMessage {
            from: "AI".to_string(),
            date: current_timestamp(),
            id: 4,
            content: "The key findings in the second chapter highlight the significant impact of the recent policy changes on the economy. It also discusses the statistical data supporting these findings.".to_string(),
            document_id: 102,
        },
        DocumentMessage {
            from: "User".to_string(),
            date: current_timestamp(),
            id: 5,
            content: "Can you explain the methodology used in the research?".to_string(),
            document_id: 103,
        },
        DocumentMessage {
            from: "AI".to_string(),
            date: current_timestamp(),
            id: 6,
            content: "The research methodology includes both qualitative and quantitative approaches. Surveys and interviews were conducted to gather data, and statistical analysis was used to interpret the results.".to_string(),
            document_id: 103,
        },
        DocumentMessage {
            from: "User".to_string(),
            date: current_timestamp(),
            id: 7,
            content: "What are the recommendations given in the conclusion?".to_string(),
            document_id: 104,
        },
        DocumentMessage {
            from: "AI".to_string(),
            date: current_timestamp(),
            id: 8,
            content: "The conclusion recommends several policy changes to address the identified issues. It also suggests further research in specific areas to validate the findings.".to_string(),
            document_id: 104,
        },
    ];

    let template = DocumentDetailsTemplate { document: dummy_document, document_chat: dummy_chat };
    template
}
