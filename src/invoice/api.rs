//! Contains various type definitions for API request and response types that leverage the Bindle
//! objects

use serde::{Deserialize, Serialize};

use crate::invoice::{Invoice, Label};
use crate::search::SearchOptions;

/// A custom type for responding to invoice creation requests. Because invoices can be created
/// before parcels are uploaded, this allows the API to inform the user if there are missing parcels
/// in the bindle spec
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct InvoiceCreateResponse {
    pub invoice: Invoice,
    pub missing: Option<Vec<Label>>,
}

/// A response to a missing parcels request. TOML doesn't support top level arrays, so they
/// must be embedded in a table
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MissingParcelsResponse {
    pub missing: Vec<Label>,
}

/// A string error message returned from the server
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Available options for the query API
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct QueryOptions {
    #[serde(alias = "q")]
    pub query: Option<String>,
    #[serde(alias = "v")]
    pub version: Option<String>,
    #[serde(alias = "o")]
    pub offset: Option<u64>,
    #[serde(alias = "l")]
    pub limit: Option<u8>,
    pub strict: Option<bool>,
    pub yanked: Option<bool>,
}

impl From<QueryOptions> for SearchOptions {
    fn from(qo: QueryOptions) -> Self {
        let defaults = SearchOptions::default();
        SearchOptions {
            limit: qo.limit.unwrap_or(defaults.limit),
            offset: qo.offset.unwrap_or(defaults.offset),
            strict: qo.strict.unwrap_or(defaults.strict),
            yanked: qo.yanked.unwrap_or(defaults.yanked),
        }
    }
}
