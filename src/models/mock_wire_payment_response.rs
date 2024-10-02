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
pub struct MockWirePaymentResponse {
    /// Wire tracking reference that needs to be set in the wire reference to beneficiary field. This field is retrievable through the response during wire creation or via the bank instruction endpoint.
    #[serde(rename = "trackingRef", skip_serializing_if = "Option::is_none")]
    pub tracking_ref: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<models::FiatMoneyUsd>,
    #[serde(rename = "beneficiaryBank", skip_serializing_if = "Option::is_none")]
    pub beneficiary_bank: Option<models::MockWirePaymentBeneficiaryBankInstruction>,
    /// Enumerated status of the wire payment. Status `pending` indicates that the wire payment is in process; `processed` indicates it finished successfully; `failed` indicates it failed.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl MockWirePaymentResponse {
    pub fn new() -> MockWirePaymentResponse {
        MockWirePaymentResponse {
            tracking_ref: None,
            amount: None,
            beneficiary_bank: None,
            status: None,
        }
    }
}
/// Enumerated status of the wire payment. Status `pending` indicates that the wire payment is in process; `processed` indicates it finished successfully; `failed` indicates it failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processed")]
    Processed,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

