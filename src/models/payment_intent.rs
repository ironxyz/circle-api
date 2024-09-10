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
pub struct PaymentIntent {
    /// Unique system generated identifier for the entity.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "amount")]
    pub amount: models::CryptoPaymentsMoney,
    #[serde(rename = "amountPaid", skip_serializing_if = "Option::is_none")]
    pub amount_paid: Option<models::CryptoPaymentsMoney>,
    #[serde(rename = "amountRefunded", skip_serializing_if = "Option::is_none")]
    pub amount_refunded: Option<models::CryptoPaymentsMoney>,
    /// Desired currency for the payments to settle in.
    #[serde(rename = "settlementCurrency")]
    pub settlement_currency: SettlementCurrency,
    #[serde(rename = "paymentMethods")]
    pub payment_methods: Vec<models::PaymentMethodBlockchain>,
    #[serde(rename = "fees", skip_serializing_if = "Option::is_none")]
    pub fees: Option<Vec<models::PaymentIntentFees>>,
    /// List of associated payments.
    #[serde(rename = "paymentIds", skip_serializing_if = "Option::is_none")]
    pub payment_ids: Option<Vec<uuid::Uuid>>,
    /// List of associated refunds.
    #[serde(rename = "refundIds", skip_serializing_if = "Option::is_none")]
    pub refund_ids: Option<Vec<uuid::Uuid>>,
    /// State management timeline.
    #[serde(rename = "timeline", skip_serializing_if = "Option::is_none")]
    pub timeline: Option<Vec<models::Timeline>>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "expiresOn", skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<String>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "updateDate", skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// Unique system generated identifier for the wallet of the merchant.
    #[serde(rename = "merchantWalletId", skip_serializing_if = "Option::is_none")]
    pub merchant_wallet_id: Option<String>,
}

impl PaymentIntent {
    pub fn new(amount: models::CryptoPaymentsMoney, settlement_currency: SettlementCurrency, payment_methods: Vec<models::PaymentMethodBlockchain>) -> PaymentIntent {
        PaymentIntent {
            id: None,
            amount,
            amount_paid: None,
            amount_refunded: None,
            settlement_currency,
            payment_methods,
            fees: None,
            payment_ids: None,
            refund_ids: None,
            timeline: None,
            expires_on: None,
            update_date: None,
            create_date: None,
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

