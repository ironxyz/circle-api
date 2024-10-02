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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodBlockchain {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "chain")]
    pub chain: models::Chain,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl PaymentMethodBlockchain {
    pub fn new(r#type: Type, chain: models::Chain) -> PaymentMethodBlockchain {
        PaymentMethodBlockchain {
            r#type,
            chain,
            address: None,
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

