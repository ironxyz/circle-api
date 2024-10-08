/*
 * All Circle APIs
 *
 * Circle's General, Core Functionality, Payments, Payouts, Accounts, and Crypto Payments APIs bundled into one OpenAPI Specification.
 *
 * The version of the OpenAPI document: 2.12.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ExternalFiatAccountStatus : Status of the account. A `pending` status indicates that the linking is in-progress; `complete` indicates the account was linked successfully; `failed` indicates it failed.
/// Status of the account. A `pending` status indicates that the linking is in-progress; `complete` indicates the account was linked successfully; `failed` indicates it failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExternalFiatAccountStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "failed")]
    Failed,

}

impl std::fmt::Display for ExternalFiatAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Complete => write!(f, "complete"),
            Self::Failed => write!(f, "failed"),
        }
    }
}

impl Default for ExternalFiatAccountStatus {
    fn default() -> ExternalFiatAccountStatus {
        Self::Pending
    }
}

