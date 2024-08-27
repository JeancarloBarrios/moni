use crate::templates;
use crate::templates::{
    AddToReportDialogueTemplate, DocumentDetailsTemplate, DocumentsTemplate, InsightReportPage,
};
use askama_axum::IntoResponse;
use axum::extract::Path as AxumPath;
use chrono::prelude::*;
use vertex_ai::discovery_engine::client::{DataStoreClient, Document, SessionSpec, SnippetSpec, ExtractiveContentSpec, ContentSearchSpec, Mode, SpellCorrectionSpec, Condition, QueryExpansionSpec, DiscoveryEngineSearchRequest, SearchChunksRequest, SearchRequest};
use crate::documents::read_documents;
pub async fn home() -> impl IntoResponse {
    templates::Index
}

// TODO: new iteration we can create a datastore per user/project
const ProjectId: &str = "875055333740";
const Collection: &str = "default_collection";
const DatastoreId: &str = "moni-demo_1722720098936";

// TODO: fetch from firebase
const alerting_config: &str = "Climate and Carbon credit policies";
//get documents handler
pub async fn get_documents() -> impl IntoResponse {
    let client = DataStoreClient::new().await.unwrap();
    let request = SearchRequest {
        project_id: ProjectId.to_string(),
        discovery_engine_search_request: DiscoveryEngineSearchRequest {
            session: "projects/875055333740/locations/global/collections/default_collection/engines/moni-demo-final_1722720080773/sessions/-".to_string(),
            query: alerting_config.to_string(),
            page_size: 10,
            filter: "".to_string(),
            query_expansion_spec: QueryExpansionSpec {
                condition: Condition::Auto,
                ..Default::default()
            },
            spell_correction_spec: SpellCorrectionSpec { mode: Mode::Auto },
            content_search_spec: ContentSearchSpec {
                extractive_content_spec: Some(ExtractiveContentSpec {
                    max_extractive_segment_count: Some(1),
                    ..Default::default()
                }),
                snippet_spec: Some(SnippetSpec {
                    max_snippet_count: 1,
                    return_snippet: true,
                    ..Default::default()
                }),
                chunk_spec: None,
                ..Default::default()
            },
            session_spec: SessionSpec {
                search_result_persistence_count: 5,
                ..Default::default()
            },
            ..Default::default()
        },
    };
    let response = client.search(request).await.unwrap();
    // Parse the documents from the response
    let docs: Vec<Document> = response.results.unwrap_or_default().into_iter()
        .filter_map(|result| result.document)
        .collect();
    
    let template = DocumentsTemplate {
        docs: docs,
        summary_text: "Here are the documents that match your search query.".to_string(),
    };
    // HtmlTemplate(template)
    template
}
fn current_timestamp() -> String {
    Utc::now().to_rfc3339()
}

pub async fn add_to_repo_dialogue_document() -> impl IntoResponse {
    let dummy_document = crate::documents::Document {
        url: "https://pdfobject.com/pdf/sample.pdf".to_string(),
        title: "Example Document".to_string(),
        id: 123,
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
        },
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

    let template = DocumentDetailsTemplate {
        document: dummy_document,
        document_chat: chat,
    };
    // HtmlTemplate(template)
    template
}

pub async fn insight_report_page() -> impl IntoResponse {
    let insights = vec![
        crate::documents::DocumentInsight {
            document: crate::documents::Document {
                url: "https://pdfobject.com/pdf/sample.pdf".to_string(),
                title: "Example Document".to_string(),
                id: 101,
            },
            insight: "The world is round.".to_string(),
            id: 1,
        },
        crate::documents::DocumentInsight {
            document: crate::documents::Document {
                url: "https://pdfobject.com/pdf/sample.pdf".to_string(),
                title: "Example Document".to_string(),
                id: 102,
            },
            insight: "The world is flat.".to_string(),
            id: 2,
        },
        crate::documents::DocumentInsight {
            document: crate::documents::Document {
                url: "https://pdfobject.com/pdf/sample.pdf".to_string(),
                title: "Example Document".to_string(),
                id: 103,
            },
            insight: "The world is a donut.".to_string(),
            id: 3,
        },
    ];

    let template = InsightReportPage {
        insights: insights,
        report: crate::documents::Report {
            id: 1,
            content: "This is a report on the insights gathered from various documents."
                .to_string(),
            template: " This is the template to provide LLM for report generation".to_string(),
            title: "Insights Report".to_string(),
            date: current_timestamp(),
        },
    };
    // HtmlTemplate(template)
    template
}
