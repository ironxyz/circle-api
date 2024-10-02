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

/// WireCreationRequestAccountNumber : Relevant fields for non-U.S. banks that do NOT support IBAN.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WireCreationRequestAccountNumber {
    /// Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests.
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: uuid::Uuid,
    /// Account number that identifies the bank account.
    #[serde(rename = "accountNumber")]
    pub account_number: String,
    /// The bank's SWIFT / BIC code.
    #[serde(rename = "routingNumber")]
    pub routing_number: String,
    #[serde(rename = "billingDetails")]
    pub billing_details: models::BillingDetails,
    #[serde(rename = "bankAddress")]
    pub bank_address: models::BankAddressNonIban,
}

impl WireCreationRequestAccountNumber {
    /// Relevant fields for non-U.S. banks that do NOT support IBAN.
    pub fn new(idempotency_key: uuid::Uuid, account_number: String, routing_number: String, billing_details: models::BillingDetails, bank_address: models::BankAddressNonIban) -> WireCreationRequestAccountNumber {
        WireCreationRequestAccountNumber {
            idempotency_key,
            account_number,
            routing_number,
            billing_details,
            bank_address,
        }
    }
}

