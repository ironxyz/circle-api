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
pub struct GooglePayToken {
    /// Verifies the message came from Google. The signature is created using ECDSA.
    #[serde(rename = "signature")]
    pub signature: String,
    /// Identifies which encryption/signing scheme created this message. In this way, the protocol can evolve over time if needed. If it is not set, assume ECv0.
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// A serialized JSON string containing the encryptedMessage, ephemeralPublicKey, and tag. To simplify the signature verification process, this value is serialized.
    #[serde(rename = "signedMessage")]
    pub signed_message: String,
}

impl GooglePayToken {
    pub fn new(signature: String, protocol_version: String, signed_message: String) -> GooglePayToken {
        GooglePayToken {
            signature,
            protocol_version,
            signed_message,
        }
    }
}

