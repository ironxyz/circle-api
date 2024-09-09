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

/// UnwithdrawalObject : Return information if the payout is returned by bank. Only present if `errorCode` of payout is `transaction_returned`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnwithdrawalObject {
    /// Unique system generated identifier for the entity.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Universally unique identifier (UUID v4) of the payout that is associated with the return.
    #[serde(rename = "payoutId", skip_serializing_if = "Option::is_none")]
    pub payout_id: Option<uuid::Uuid>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<models::FiatMoneyUsd>>,
    #[serde(rename = "fees", skip_serializing_if = "Option::is_none")]
    pub fees: Option<Box<models::FiatMoneyUsd>>,
    /// Reason for the return.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the return. A `pending` status indicates that the return is in process; `complete` indicates it finished successfully; `failed` indicates it failed.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "updateDate", skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
}

impl UnwithdrawalObject {
    /// Return information if the payout is returned by bank. Only present if `errorCode` of payout is `transaction_returned`.
    pub fn new() -> UnwithdrawalObject {
        UnwithdrawalObject {
            id: None,
            payout_id: None,
            amount: None,
            fees: None,
            reason: None,
            status: None,
            create_date: None,
            update_date: None,
        }
    }
}
/// Status of the return. A `pending` status indicates that the return is in process; `complete` indicates it finished successfully; `failed` indicates it failed.
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

