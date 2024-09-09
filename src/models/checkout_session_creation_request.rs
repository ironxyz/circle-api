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
pub struct CheckoutSessionCreationRequest {
    /// The URL returned to you through client-side callback when the payment is completed.
    #[serde(rename = "successUrl", skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(rename = "amount")]
    pub amount: Box<models::CheckoutSessionMoney>,
}

impl CheckoutSessionCreationRequest {
    pub fn new(amount: models::CheckoutSessionMoney) -> CheckoutSessionCreationRequest {
        CheckoutSessionCreationRequest {
            success_url: None,
            amount: Box::new(amount),
        }
    }
}

