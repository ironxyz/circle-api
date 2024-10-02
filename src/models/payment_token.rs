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
pub struct PaymentToken {
    /// Unique system generated identifier for the digital wallet token.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Type of the digital wallet token.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Datetime when the digital token expires. ISO-8601.
    #[serde(rename = "expiresOn")]
    pub expires_on: String,
    #[serde(rename = "cardDetails")]
    pub card_details: models::TokenizedCardDetails,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "createDate")]
    pub create_date: String,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "updateDate")]
    pub update_date: String,
}

impl PaymentToken {
    pub fn new(id: uuid::Uuid, r#type: Type, expires_on: String, card_details: models::TokenizedCardDetails, create_date: String, update_date: String) -> PaymentToken {
        PaymentToken {
            id,
            r#type,
            expires_on,
            card_details,
            create_date,
            update_date,
        }
    }
}
/// Type of the digital wallet token.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "applepay")]
    Applepay,
    #[serde(rename = "googlepay")]
    Googlepay,
}

impl Default for Type {
    fn default() -> Type {
        Self::Applepay
    }
}

