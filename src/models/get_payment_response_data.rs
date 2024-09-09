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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPaymentResponseData {
    FiatPaymentPolymorphic(Box<models::FiatPaymentPolymorphic>),
    CryptoPayment(Box<models::CryptoPayment>),
}

impl Default for GetPaymentResponseData {
    fn default() -> Self {
        Self::FiatPaymentPolymorphic(Default::default())
    }
}
/// Type of the payment object.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "refund")]
    Refund,
    #[serde(rename = "cancel")]
    Cancel,
}

impl Default for Type {
    fn default() -> Type {
        Self::Payment
    }
}
/// Enumerated description of the payment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Description {
    #[serde(rename = "Payment")]
    Payment,
}

impl Default for Description {
    fn default() -> Description {
        Self::Payment
    }
}

