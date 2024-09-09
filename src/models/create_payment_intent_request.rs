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
pub enum CreatePaymentIntentRequest {
    PaymentIntentCreationRequest(Box<models::PaymentIntentCreationRequest>),
    ContinuousPaymentIntentCreationRequest(Box<models::ContinuousPaymentIntentCreationRequest>),
}

impl Default for CreatePaymentIntentRequest {
    fn default() -> Self {
        Self::PaymentIntentCreationRequest(Default::default())
    }
}
/// Desired currency for the payments to settle in.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SettlementCurrency {
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "BTC")]
    Btc,
    #[serde(rename = "ETH")]
    Eth,
}

impl Default for SettlementCurrency {
    fn default() -> SettlementCurrency {
        Self::Usd
    }
}
/// Desired currency for the payment
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "BTC")]
    Btc,
    #[serde(rename = "ETH")]
    Eth,
}

impl Default for Currency {
    fn default() -> Currency {
        Self::Usd
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "continuous")]
    Continuous,
}

impl Default for Type {
    fn default() -> Type {
        Self::Continuous
    }
}

