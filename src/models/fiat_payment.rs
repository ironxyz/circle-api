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

/// FiatPayment : Status information of the related payment. This property is only present on refund or cancel items.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiatPayment {
    /// Unique system generated identifier for the entity.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Type of the payment object.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Unique system generated identifier for the merchant.
    #[serde(rename = "merchantId")]
    pub merchant_id: uuid::Uuid,
    /// Unique system generated identifier for the wallet of the merchant.
    #[serde(rename = "merchantWalletId", skip_serializing_if = "Option::is_none")]
    pub merchant_wallet_id: Option<String>,
    #[serde(rename = "amount")]
    pub amount: models::FiatMoneyUsd,
    #[serde(rename = "fromAmount", skip_serializing_if = "Option::is_none")]
    pub from_amount: Option<models::FiatMoney>,
    #[serde(rename = "source")]
    pub source: models::SourceResponse,
    /// Enumerated description of the payment.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "status")]
    pub status: models::PaymentStatus,
    /// Determines if a payment has successfully been captured. This property is only present for payments that did not use auto capture.
    #[serde(rename = "captured", skip_serializing_if = "Option::is_none")]
    pub captured: Option<bool>,
    #[serde(rename = "captureAmount", skip_serializing_if = "Option::is_none")]
    pub capture_amount: Option<models::FiatMoneyUsd>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "captureDate", skip_serializing_if = "Option::is_none")]
    pub capture_date: Option<String>,
    #[serde(rename = "requiredAction", skip_serializing_if = "Option::is_none")]
    pub required_action: Option<models::RequiredAction>,
    #[serde(rename = "cancel", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cancel: Option<Option<models::PaymentInfoCancel>>,
    #[serde(rename = "refunds", skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Vec<models::PaymentInfoPaymentAndRefund>>,
    #[serde(rename = "fees", skip_serializing_if = "Option::is_none")]
    pub fees: Option<models::FiatMoneyUsd>,
    /// The channel identifier that can be set for the payment. When not provided, the default channel is used.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<uuid::Uuid>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "updateDate", skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
}

impl FiatPayment {
    /// Status information of the related payment. This property is only present on refund or cancel items.
    pub fn new(id: uuid::Uuid, r#type: Type, merchant_id: uuid::Uuid, amount: models::FiatMoneyUsd, source: models::SourceResponse, status: models::PaymentStatus) -> FiatPayment {
        FiatPayment {
            id,
            r#type,
            merchant_id,
            merchant_wallet_id: None,
            amount,
            from_amount: None,
            source,
            description: None,
            status,
            captured: None,
            capture_amount: None,
            capture_date: None,
            required_action: None,
            cancel: None,
            refunds: None,
            fees: None,
            channel: None,
            create_date: None,
            update_date: None,
        }
    }
}
/// Type of the payment object.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "payment")]
    Payment,
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

