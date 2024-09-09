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
pub struct CryptoPayout {
    /// Unique system generated identifier for the entity.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The identifier of the source wallet used to fund a payout.
    #[serde(rename = "sourceWalletId", skip_serializing_if = "Option::is_none")]
    pub source_wallet_id: Option<String>,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<Box<models::CryptoPayoutDestination>>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<models::PayoutMoney>>,
    #[serde(rename = "toAmount", skip_serializing_if = "Option::is_none")]
    pub to_amount: Option<Box<models::PayoutMoney>>,
    #[serde(rename = "fees", skip_serializing_if = "Option::is_none")]
    pub fees: Option<Box<models::PayoutMoney>>,
    #[serde(rename = "networkFees", skip_serializing_if = "Option::is_none")]
    pub network_fees: Option<Box<models::PayoutMoney>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::PayoutStatus>,
    #[serde(rename = "errorCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<Option<models::PayoutErrorCode>>,
    #[serde(rename = "riskEvaluation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub risk_evaluation: Option<Option<Box<models::RiskEvaluation>>>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "updateDate", skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
}

impl CryptoPayout {
    pub fn new() -> CryptoPayout {
        CryptoPayout {
            id: None,
            source_wallet_id: None,
            destination: None,
            amount: None,
            to_amount: None,
            fees: None,
            network_fees: None,
            status: None,
            error_code: None,
            risk_evaluation: None,
            create_date: None,
            update_date: None,
        }
    }
}

