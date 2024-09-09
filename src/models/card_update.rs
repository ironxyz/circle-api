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

use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardUpdate {
    /// Universally unique identifier (UUID v4) of the public key used in encryption. NOTE the sandbox environment uses the default value of `key1`. For this reason the example supplied is `key1` rather than a UUID.
    #[serde(rename = "keyId")]
    pub key_id: uuid::Uuid,
    /// PGP encrypted base64 encoded string. Contains CVV. * **CVV (Card Verification Number)**: Three or four digit security code. REQUIRED' 
    #[serde_as(as = "serde_with::base64::Base64")]
    #[serde(rename = "encryptedData")]
    pub encrypted_data: Vec<u8>,
    /// Two digit number representing the card's expiration month.
    #[serde(rename = "expMonth", skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i32>,
    /// Four digit number representing the card's expiration year.
    #[serde(rename = "expYear", skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i32>,
    #[serde(rename = "billingDetails", skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<Box<models::UpdateBillingDetails>>,
}

impl CardUpdate {
    pub fn new(key_id: uuid::Uuid, encrypted_data: Vec<u8>) -> CardUpdate {
        CardUpdate {
            key_id,
            encrypted_data,
            exp_month: None,
            exp_year: None,
            billing_details: None,
        }
    }
}

