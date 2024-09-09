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
pub struct ApplePayTokenRsaHeader {
    /// Optional. Hash of the applicationData property of the original PKPaymentRequest object. If the value of that property is null, this key is omitted.
    #[serde(rename = "applicationData", skip_serializing_if = "Option::is_none")]
    pub application_data: Option<String>,
    /// The symmetric key wrapped using your RSA public key.
    #[serde(rename = "wrappedKey")]
    pub wrapped_key: String,
    /// Hash of the X.509 encoded public key bytes of the merchant’s certificate.
    #[serde(rename = "publicKeyHash")]
    pub public_key_hash: String,
    /// Transaction identifier, generated on the device.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

impl ApplePayTokenRsaHeader {
    pub fn new(wrapped_key: String, public_key_hash: String, transaction_id: String) -> ApplePayTokenRsaHeader {
        ApplePayTokenRsaHeader {
            application_data: None,
            wrapped_key,
            public_key_hash,
            transaction_id,
        }
    }
}

