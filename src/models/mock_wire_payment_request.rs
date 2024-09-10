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
pub struct MockWirePaymentRequest {
    /// Wire tracking reference that needs to be set in the wire reference to beneficiary field. This field is retrievable through the response during wire creation or via the bank instruction endpoint.
    #[serde(rename = "trackingRef")]
    pub tracking_ref: String,
    #[serde(rename = "amount")]
    pub amount: models::FiatMoneyUsd,
    #[serde(rename = "beneficiaryBank")]
    pub beneficiary_bank: models::MockWirePaymentBeneficiaryBankInstruction,
}

impl MockWirePaymentRequest {
    pub fn new(tracking_ref: String, amount: models::FiatMoneyUsd, beneficiary_bank: models::MockWirePaymentBeneficiaryBankInstruction) -> MockWirePaymentRequest {
        MockWirePaymentRequest {
            tracking_ref,
            amount,
            beneficiary_bank,
        }
    }
}

