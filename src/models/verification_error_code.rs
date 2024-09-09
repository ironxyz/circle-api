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

/// VerificationErrorCode : Indicates the failure reason of the card verification. Only present on cards with failed verification. Possible values are [verification_failed, verification_fraud_detected, verification_denied, verification_not_supported_by_issuer, verification_stopped_by_issuer, card_failed, card_invalid, card_address_mismatch, card_zip_mismatch, card_cvv_invalid, card_expired, card_limit_violated, card_not_honored, card_cvv_required, credit_card_not_allowed, card_account_ineligible, card_network_unsupported]'
/// Indicates the failure reason of the card verification. Only present on cards with failed verification. Possible values are [verification_failed, verification_fraud_detected, verification_denied, verification_not_supported_by_issuer, verification_stopped_by_issuer, card_failed, card_invalid, card_address_mismatch, card_zip_mismatch, card_cvv_invalid, card_expired, card_limit_violated, card_not_honored, card_cvv_required, credit_card_not_allowed, card_account_ineligible, card_network_unsupported]'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VerificationErrorCode {
    #[serde(rename = "verification_failed")]
    VerificationFailed,
    #[serde(rename = "verification_fraud_detected")]
    VerificationFraudDetected,
    #[serde(rename = "verification_denied")]
    VerificationDenied,
    #[serde(rename = "verification_not_supported_by_issuer")]
    VerificationNotSupportedByIssuer,
    #[serde(rename = "verification_stopped_by_issuer")]
    VerificationStoppedByIssuer,
    #[serde(rename = "card_failed")]
    CardFailed,
    #[serde(rename = "card_invalid")]
    CardInvalid,
    #[serde(rename = "card_address_mismatch")]
    CardAddressMismatch,
    #[serde(rename = "card_zip_mismatch")]
    CardZipMismatch,
    #[serde(rename = "card_cvv_invalid")]
    CardCvvInvalid,
    #[serde(rename = "card_expired")]
    CardExpired,
    #[serde(rename = "card_limit_violated")]
    CardLimitViolated,
    #[serde(rename = "card_not_honored")]
    CardNotHonored,
    #[serde(rename = "card_cvv_required")]
    CardCvvRequired,
    #[serde(rename = "credit_card_not_allowed")]
    CreditCardNotAllowed,
    #[serde(rename = "card_account_ineligible")]
    CardAccountIneligible,
    #[serde(rename = "card_network_unsupported")]
    CardNetworkUnsupported,

}

impl std::fmt::Display for VerificationErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::VerificationFailed => write!(f, "verification_failed"),
            Self::VerificationFraudDetected => write!(f, "verification_fraud_detected"),
            Self::VerificationDenied => write!(f, "verification_denied"),
            Self::VerificationNotSupportedByIssuer => write!(f, "verification_not_supported_by_issuer"),
            Self::VerificationStoppedByIssuer => write!(f, "verification_stopped_by_issuer"),
            Self::CardFailed => write!(f, "card_failed"),
            Self::CardInvalid => write!(f, "card_invalid"),
            Self::CardAddressMismatch => write!(f, "card_address_mismatch"),
            Self::CardZipMismatch => write!(f, "card_zip_mismatch"),
            Self::CardCvvInvalid => write!(f, "card_cvv_invalid"),
            Self::CardExpired => write!(f, "card_expired"),
            Self::CardLimitViolated => write!(f, "card_limit_violated"),
            Self::CardNotHonored => write!(f, "card_not_honored"),
            Self::CardCvvRequired => write!(f, "card_cvv_required"),
            Self::CreditCardNotAllowed => write!(f, "credit_card_not_allowed"),
            Self::CardAccountIneligible => write!(f, "card_account_ineligible"),
            Self::CardNetworkUnsupported => write!(f, "card_network_unsupported"),
        }
    }
}

impl Default for VerificationErrorCode {
    fn default() -> VerificationErrorCode {
        Self::VerificationFailed
    }
}

