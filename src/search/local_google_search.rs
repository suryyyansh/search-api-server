use llama_core::{
    error::SearchError,
    search::{SearchOutput, SearchResult},
};
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct LocalGoogleSearchInput {
    pub term: String,
    pub engine: String,
    pub maxSearchResults: u8,
}

#[allow(dead_code)]
pub fn local_google_parser(raw_results: &serde_json::Value) -> Result<SearchOutput, SearchError> {
    let results_array = match raw_results.as_array() {
        Some(array) => array,
        None => {
            let msg = "No results returned from server";
            error!(target: "search_server", "google_parser: {}", msg);
            return Err(SearchError::Response(msg.to_string()));
        }
    };

    let mut results = Vec::new();

    for result in results_array {
        let current_result = SearchResult {
            url: result["url"].to_string(),
            site_name: result["siteName"].to_string(),
            text_content: result["textContent"].to_string(),
        };
        results.push(current_result)
    }

    Ok(SearchOutput { results })
}