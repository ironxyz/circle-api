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
pub struct DetailedRefund {
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
    #[serde(rename = "source")]
    pub source: models::SourceResponse,
    /// Enumerated description of the payment.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "status")]
    pub status: models::CancelRefundReversalStatus,
    #[serde(rename = "originalPayment", skip_serializing_if = "Option::is_none")]
    pub original_payment: Option<models::FiatPayment>,
    #[serde(rename = "cancel", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cancel: Option<Option<models::FiatCancel>>,
    #[serde(rename = "fees", skip_serializing_if = "Option::is_none")]
    pub fees: Option<models::FiatMoneyUsd>,
    /// Payment tracking reference. Will be present once known.
    #[serde(rename = "trackingRef", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tracking_ref: Option<Option<String>>,
    #[serde(rename = "errorCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<Option<models::PaymentErrorCode>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<models::MetadataPhoneEmail>,
    #[serde(rename = "riskEvaluation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub risk_evaluation: Option<Option<models::RiskEvaluation>>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "updateDate", skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
}

impl DetailedRefund {
    pub fn new(id: uuid::Uuid, r#type: Type, merchant_id: uuid::Uuid, amount: models::FiatMoneyUsd, source: models::SourceResponse, status: models::CancelRefundReversalStatus) -> DetailedRefund {
        DetailedRefund {
            id,
            r#type,
            merchant_id,
            merchant_wallet_id: None,
            amount,
            source,
            description: None,
            status,
            original_payment: None,
            cancel: None,
            fees: None,
            tracking_ref: None,
            error_code: None,
            metadata: None,
            risk_evaluation: None,
            create_date: None,
            update_date: None,
        }
    }
}
/// Type of the payment object.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "refund")]
    Refund,
}

impl Default for Type {
    fn default() -> Type {
        Self::Refund
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

