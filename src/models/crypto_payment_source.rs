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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CryptoPaymentSource {
    /// The source address
    #[serde(rename = "address")]
    pub address: String,
    /// The source type
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl CryptoPaymentSource {
    pub fn new(address: String, r#type: Type) -> CryptoPaymentSource {
        CryptoPaymentSource {
            address,
            r#type,
        }
    }
}
/// The source type
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

