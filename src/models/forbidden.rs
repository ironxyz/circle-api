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
pub struct Forbidden {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "message")]
    pub message: String,
}

impl Forbidden {
    pub fn new(code: i32, message: String) -> Forbidden {
        Forbidden {
            code,
            message,
        }
    }
}

