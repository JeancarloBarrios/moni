use crate::templates;
use crate::templates::{
    AddToReportDialogueTemplate, DocumentDetailsTemplate, DocumentsTemplate, InsightReportPage,
};
use axum::{extract::State, response::IntoResponse};
use axum::extract::Path as AxumPath;
use chrono::prelude::*;
use vertex_ai::discovery_engine::client::{DataStoreClient, Document, SessionSpec, SnippetSpec, ExtractiveContentSpec, ContentSearchSpec, Mode, SpellCorrectionSpec, Condition, QueryExpansionSpec, DiscoveryEngineSearchRequest, SearchChunksRequest, SearchRequest};
use gemini::client::GeminiClient;
use std::sync::Arc;
use crate::AppState;
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

// TODO: new iteration we can create a datastore to customize templates per user
pub fn build_report_prompt(insights_content: String) -> String {
    let report_template = r#"
        Create a report using the following template and using the next information as a baseline.

        {{ insights }}

        ----------------------------------

        Summary of Global Public Policies Relevant to Climate Change

        1. Executive Summary
        Purpose of the Report: Provide an overview of the most significant public policies related to climate change globally.
        Scope: Briefly mention the geographical regions covered and the types of policies (e.g., emission reduction targets, renewable energy incentives, etc.).
        Key Findings: Highlight the most critical points, such as major policy trends, significant legislative actions, and emerging areas of focus.

        2. Introduction
        Background: Provide context on why climate-related public policies are important. Mention global initiatives like the Paris Agreement and the role of policy in driving climate action.
        Objective: Clearly state the objective of the report, such as informing stakeholders of the latest policy developments or assessing the impact of these policies on your organization's goals.

        3. Policy Analysis by Region
        Region 1: [Name of Region]

        Overview: Briefly describe the region's approach to climate policy.
        Key Policies: List and describe the most relevant policies, including their goals, implementation status, and expected impact.
        Challenges and Opportunities: Analyze any barriers to implementation or potential benefits for the region.

        Region 2: [Name of Region]

        Overview
        Key Policies
        Challenges and Opportunities

        [Continue this structure for each region]

        4. Thematic Analysis
        Policy Types: Summarize the different types of policies observed (e.g., carbon pricing, renewable energy incentives, climate adaptation strategies).
        Global Trends: Identify and discuss any global trends, such as increased focus on climate finance or nature-based solutions.
        Comparative Analysis: Compare policies across regions, highlighting differences and similarities in approaches.

        5. Impact on Industry/Organization
        Relevance to Our Operations: Discuss how the identified policies may impact your organization, including potential risks and opportunities.
        Strategic Recommendations: Provide recommendations on how to align your operations or strategies with these policies. Suggest actions for compliance, risk management, or leveraging opportunities.

        6. Conclusion
        Summary of Findings: Recap the most significant insights from your analysis.
        Next Steps: Outline any further research or actions that should be taken based on the report's findings.

        7. References
        List all sources used in your research, formatted according to your company's or industry's preferred citation style.


         If you are not able to find references, please ommit this section.
        "#;
        let final_report = report_template.replace("{{ insights }}", insights_content.as_str());
        final_report
}
pub async fn insight_report_page(
    State(app_state): State<Arc<AppState>>, // Extract the AppState
) -> impl IntoResponse {
    let insights = vec![
        crate::documents::DocumentInsight {
            document: crate::documents::Document {
                url: "https://pdfobject.com/pdf/sample.pdf".to_string(),
                title: "Example Document".to_string(),
                id: 101,
            },
            insight: "Inisght #1.".to_string(),
            id: 1,
        },
        crate::documents::DocumentInsight {
            document: crate::documents::Document {
                url: "https://pdfobject.com/pdf/sample.pdf".to_string(),
                title: "Example Document".to_string(),
                id: 102,
            },
            insight: "Insight #2".to_string(),
            id: 2,
        },
        crate::documents::DocumentInsight {
            document: crate::documents::Document {
                url: "https://pdfobject.com/pdf/sample.pdf".to_string(),
                title: "Example Document".to_string(),
                id: 103,
            },
            insight: "Insight #3".to_string(),
            id: 3,
        },
    ];
    let gemini_client = Arc::clone(&app_state.gemini_client);
    let prompt = build_report_prompt("Some insights about climate policy go here".to_string());
    // Call the `text_post` method on the GeminiClient
    let content = match gemini_client.request_text(prompt.as_str()).await {
        Ok(response) => {
            if let Some(gemini_response) = response.rest() {
                let default_value = "no answer from Gemini".to_string();
                let answer = gemini_response.candidates[0]
                    .content
                    .parts[0]
                    .text
                    .as_ref()
                    .unwrap_or(&default_value);
                answer.to_string()
            } else {
                "no answer from Gemini".to_string()
            }
        },
        Err(e) => {
            eprintln!("Error calling Gemini API: {:?}", e);
            "Error calling Gemini API".to_string() // Return an error message as a string
        }
    };


    let template = InsightReportPage {
        insights: insights,
        report: crate::documents::Report {
            id: 1,
            content: content.to_string(),
            template: " This is the template to provide LLM for report generation".to_string(),
            title: "Insights Report".to_string(),
            date: current_timestamp(),
        },
    };
    // HtmlTemplate(template)
    template
}
