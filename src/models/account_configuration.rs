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
pub struct AccountConfiguration {
    #[serde(rename = "payments", skip_serializing_if = "Option::is_none")]
    pub payments: Option<models::WalletConfig>,
}

impl AccountConfiguration {
    pub fn new() -> AccountConfiguration {
        AccountConfiguration {
            payments: None,
        }
    }
}

