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
pub struct PresignMessageTypesItems {
    /// The name of the field
    #[serde(rename = "name")]
    pub name: String,
    /// The type of the field
    #[serde(rename = "type")]
    pub r#type: String,
}

impl PresignMessageTypesItems {
    pub fn new(name: String, r#type: String) -> PresignMessageTypesItems {
        PresignMessageTypesItems {
            name,
            r#type,
        }
    }
}

