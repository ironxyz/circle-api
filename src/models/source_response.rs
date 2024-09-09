/*
 * All Circle APIs
 *
 * Circle's General, Core Functionality, Payments, Payouts, Accounts, and Crypto Payments APIs bundled into one OpenAPI Specification.
 *
 * The version of the OpenAPI document: 2.11.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SourceResponse : The payment source.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceResponse {
    /// Unique system generated identifier for the entity.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Type of the source.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl SourceResponse {
    /// The payment source.
    pub fn new() -> SourceResponse {
        SourceResponse {
            id: None,
            r#type: None,
        }
    }
}
/// Type of the source.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "wire")]
    Wire,
    #[serde(rename = "sepa")]
    Sepa,
}

impl Default for Type {
    fn default() -> Type {
        Self::Card
    }
}

