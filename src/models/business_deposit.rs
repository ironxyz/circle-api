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

/// BusinessDeposit : A deposit
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessDeposit {
    /// Unique system generated identifier for the entity.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The identifier for the bank account where the funds were deposited from.
    #[serde(rename = "sourceWalletId", skip_serializing_if = "Option::is_none")]
    pub source_wallet_id: Option<uuid::Uuid>,
    #[serde(rename = "destination")]
    pub destination: Box<models::WalletLocation>,
    #[serde(rename = "amount")]
    pub amount: Box<models::FiatMoney>,
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<Box<models::FiatMoneyUsd>>,
    /// Status of the deposit. Status `pending` indicates that the deposit is in the process of running; `complete` indicates it finished successfully; `failed` indicates it failed.
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "riskEvaluation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub risk_evaluation: Option<Option<Box<models::RiskEvaluation>>>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "createDate")]
    pub create_date: String,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "updateDate", skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
}

impl BusinessDeposit {
    /// A deposit
    pub fn new(id: uuid::Uuid, destination: models::WalletLocation, amount: models::FiatMoney, status: Status, create_date: String) -> BusinessDeposit {
        BusinessDeposit {
            id,
            source_wallet_id: None,
            destination: Box::new(destination),
            amount: Box::new(amount),
            fee: None,
            status,
            risk_evaluation: None,
            create_date,
            update_date: None,
        }
    }
}
/// Status of the deposit. Status `pending` indicates that the deposit is in the process of running; `complete` indicates it finished successfully; `failed` indicates it failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

