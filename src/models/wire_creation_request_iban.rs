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

/// WireCreationRequestIban : Relevant fields for non-U.S. bank accounts that support IBAN.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WireCreationRequestIban {
    /// Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests.
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: uuid::Uuid,
    /// International Bank Account Number (IBAN) for the bank account.
    #[serde(rename = "iban")]
    pub iban: String,
    #[serde(rename = "billingDetails")]
    pub billing_details: models::BillingDetails,
    #[serde(rename = "bankAddress")]
    pub bank_address: models::BankAddressIbanSupported,
}

impl WireCreationRequestIban {
    /// Relevant fields for non-U.S. bank accounts that support IBAN.
    pub fn new(idempotency_key: uuid::Uuid, iban: String, billing_details: models::BillingDetails, bank_address: models::BankAddressIbanSupported) -> WireCreationRequestIban {
        WireCreationRequestIban {
            idempotency_key,
            iban,
            billing_details,
            bank_address,
        }
    }
}

