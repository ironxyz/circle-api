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

/// TransferTypeInfo : Additional information for specific transfer type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferTypeInfo {
    #[serde(rename = "currencies")]
    pub currencies: Vec<models::FiatCurrency>,
    #[serde(rename = "additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<Box<models::TransferTypeInfoAdditionalProperties>>,
}

impl TransferTypeInfo {
    /// Additional information for specific transfer type.
    pub fn new(currencies: Vec<models::FiatCurrency>) -> TransferTypeInfo {
        TransferTypeInfo {
            currencies,
            additional_properties: None,
        }
    }
}

