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
pub struct GetBusinessPixAccountResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<models::PixFiatAccountResponse>,
}

impl GetBusinessPixAccountResponse {
    pub fn new() -> GetBusinessPixAccountResponse {
        GetBusinessPixAccountResponse {
            data: None,
        }
    }
}

