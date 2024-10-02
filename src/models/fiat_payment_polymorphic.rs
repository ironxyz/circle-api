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
pub struct FiatPaymentPolymorphic {
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
    #[serde(rename = "requiredAction", skip_serializing_if = "Option::is_none")]
    pub required_action: Option<models::RequiredAction>,
    #[serde(rename = "verification", skip_serializing_if = "Option::is_none")]
    pub verification: Option<models::PaymentVerificationResponse>,
    #[serde(rename = "originalPayment", skip_serializing_if = "Option::is_none")]
    pub original_payment: Option<models::FiatPayment>,
    #[serde(rename = "cancel", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cancel: Option<Option<models::FiatCancel>>,
    #[serde(rename = "refunds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Option<Vec<models::FiatRefund>>>,
    #[serde(rename = "fees", skip_serializing_if = "Option::is_none")]
    pub fees: Option<models::FiatMoneyUsd>,
    /// Payment tracking reference. Will be present once known.
    #[serde(rename = "trackingRef", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tracking_ref: Option<Option<String>>,
    /// External network identifier which will be present once provided from the applicable network.   Examples: * **Input/Output Message Accountability Data (IMAD/OMAD)**: unique number given to each FedWire payment when using the Federal Reserve Bank Service which can be used to investigate and track wire transfers. 
    #[serde(rename = "externalRef", skip_serializing_if = "Option::is_none")]
    pub external_ref: Option<String>,
    #[serde(rename = "errorCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<Option<models::PaymentErrorCode>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<models::MetadataPhoneEmail>,
    /// The channel identifier that can be set for the payment. When not provided, the default channel is used.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<uuid::Uuid>,
    #[serde(rename = "riskEvaluation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub risk_evaluation: Option<Option<models::RiskEvaluation>>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "updateDate", skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
}

impl FiatPaymentPolymorphic {
    pub fn new(id: uuid::Uuid, r#type: Type, merchant_id: uuid::Uuid, amount: models::FiatMoneyUsd, source: models::SourceResponse, status: models::PaymentStatus) -> FiatPaymentPolymorphic {
        FiatPaymentPolymorphic {
            id,
            r#type,
            merchant_id,
            merchant_wallet_id: None,
            amount,
            from_amount: None,
            source,
            description: None,
            status,
            required_action: None,
            verification: None,
            original_payment: None,
            cancel: None,
            refunds: None,
            fees: None,
            tracking_ref: None,
            external_ref: None,
            error_code: None,
            metadata: None,
            channel: None,
            risk_evaluation: None,
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

