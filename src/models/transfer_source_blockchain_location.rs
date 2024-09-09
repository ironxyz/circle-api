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

/// TransferSourceBlockchainLocation : A source blockchain address.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferSourceBlockchainLocation {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "chain")]
    pub chain: models::Chain,
    #[serde(rename = "identities", skip_serializing_if = "Option::is_none")]
    pub identities: Option<Vec<models::Identity>>,
}

impl TransferSourceBlockchainLocation {
    /// A source blockchain address.
    pub fn new(r#type: Type, chain: models::Chain) -> TransferSourceBlockchainLocation {
        TransferSourceBlockchainLocation {
            r#type,
            chain,
            identities: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "blockchain")]
    Blockchain,
}

impl Default for Type {
    fn default() -> Type {
        Self::Blockchain
    }
}

