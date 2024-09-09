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
pub struct MockWirePaymentBeneficiaryBankInstruction {
    /// Virtual account number or Circle corporate Silvergate Wire account number that needs to be set as destination.
    #[serde(rename = "accountNumber")]
    pub account_number: String,
}

impl MockWirePaymentBeneficiaryBankInstruction {
    pub fn new(account_number: String) -> MockWirePaymentBeneficiaryBankInstruction {
        MockWirePaymentBeneficiaryBankInstruction {
            account_number,
        }
    }
}

