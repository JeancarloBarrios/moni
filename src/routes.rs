use askama_axum::IntoResponse;
use axum::extract::Path as AxumPath;
use chrono::prelude::*;
use crate::templates;
use crate::templates::{DocumentDetailsTemplate, DocumentsTemplate, AddToReportDialogueTemplate};

use crate::documents::read_documents;
pub async fn home() -> impl IntoResponse {
    templates::Index
}


//get documents handler
pub async fn get_documents() -> impl IntoResponse {
    let template = DocumentsTemplate { docs: read_documents().await};
    // HtmlTemplate(template)
    template
}
fn current_timestamp() -> String {
    Utc::now().to_rfc3339()
}

pub async fn add_to_repo_dialogue_document(AxumPath(id): AxumPath<u64>) -> impl IntoResponse {
    let dummy_document = crate::documents::Document {
        url: "https://pdfobject.com/pdf/sample.pdf".to_string(),
        title: "Example Document".to_string(),
        id: id as u32,
    };
    let insights = r#"
        ## Insights
        * **Insight 1**: The world is round.
        * **Insight 2**: The world is flat.
        * **Insight 3**: The world is a donut.
    "#;

    let template = AddToReportDialogueTemplate {
        insight: crate::documents::DocumentInsight {
            document: dummy_document,
            insight: insights.to_string(),
            id: 89,
        }
    };
    // HtmlTemplate(template)
    template
}

pub async fn view_document(AxumPath(id): AxumPath<u64>) -> impl IntoResponse {
    let dummy_document = crate::documents::Document {
        url: "https://pdfobject.com/pdf/sample.pdf".to_string(),
        title: "Example Document".to_string(),
        id: id as u32,
    };
    let chat = vec![
        crate::documents::DocumentMessage {
            from: "User".to_string(),
            date: current_timestamp(),
            id: 1,
            content: "Can you summarize the introduction of the document?".to_string(),
            document_id: 101,
        },
        crate::documents::DocumentMessage {
            from: "AI".to_string(),
            date: current_timestamp(),
            id: 2,
            content: "Sure! The introduction provides an overview of the document's purpose and outlines the main topics that will be discussed.".to_string(),
            document_id: 101,
        },
        crate::documents::DocumentMessage {
            from: "User".to_string(),
            date: current_timestamp(),
            id: 3,
            content: "What are the key findings in the second chapter?".to_string(),
            document_id: 102,
        },
        crate::documents::DocumentMessage {
            from: "AI".to_string(),
            date: current_timestamp(),
            id: 4,
            content: "The key findings in the second chapter highlight the significant impact of the recent policy changes on the economy. It also discusses the statistical data supporting these findings.".to_string(),
            document_id: 102,
        },
        crate::documents::DocumentMessage {
            from: "User".to_string(),
            date: current_timestamp(),
            id: 5,
            content: "Can you explain the methodology used in the research?".to_string(),
            document_id: 103,
        },
        crate::documents::DocumentMessage {
            from: "AI".to_string(),
            date: current_timestamp(),
            id: 6,
            content: "The research methodology includes both qualitative and quantitative approaches. Surveys and interviews were conducted to gather data, and statistical analysis was used to interpret the results.".to_string(),
            document_id: 103,
        },
        crate::documents::DocumentMessage {
            from: "User".to_string(),
            date: current_timestamp(),
            id: 7,
            content: "What are the recommendations given in the conclusion?".to_string(),
            document_id: 104,
        },
        crate::documents::DocumentMessage {
            from: "AI".to_string(),
            date: current_timestamp(),
            id: 8,
            content: "The conclusion recommends several policy changes to address the identified issues. It also suggests further research in specific areas to validate the findings.".to_string(),
            document_id: 104,
        },
    ];

    let template = DocumentDetailsTemplate { document: dummy_document, document_chat: chat};
    // HtmlTemplate(template)
    template
}