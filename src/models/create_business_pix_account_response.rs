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
pub struct CreateBusinessPixAccountResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::PixFiatAccountResponse>>,
}

impl CreateBusinessPixAccountResponse {
    pub fn new() -> CreateBusinessPixAccountResponse {
        CreateBusinessPixAccountResponse {
            data: None,
        }
    }
}

