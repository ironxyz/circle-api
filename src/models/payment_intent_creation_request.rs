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
pub struct PaymentIntentCreationRequest {
    /// Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests.
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: uuid::Uuid,
    #[serde(rename = "amount")]
    pub amount: models::CryptoPaymentsMoney,
    /// Desired currency for the payments to settle in.
    #[serde(rename = "settlementCurrency")]
    pub settlement_currency: SettlementCurrency,
    #[serde(rename = "paymentMethods")]
    pub payment_methods: Vec<models::PaymentMethodBlockchain>,
    /// Unique system generated identifier for the wallet of the merchant.
    #[serde(rename = "merchantWalletId", skip_serializing_if = "Option::is_none")]
    pub merchant_wallet_id: Option<String>,
}

impl PaymentIntentCreationRequest {
    pub fn new(idempotency_key: uuid::Uuid, amount: models::CryptoPaymentsMoney, settlement_currency: SettlementCurrency, payment_methods: Vec<models::PaymentMethodBlockchain>) -> PaymentIntentCreationRequest {
        PaymentIntentCreationRequest {
            idempotency_key,
            amount,
            settlement_currency,
            payment_methods,
            merchant_wallet_id: None,
        }
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

