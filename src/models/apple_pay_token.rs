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
pub struct ApplePayToken {
    /// ApplePay token version information.
    #[serde(rename = "version")]
    pub version: Version,
    /// Encrypted payment data. Base64 encoded as a string.
    #[serde(rename = "data")]
    pub data: String,
    /// Signature of the payment and header data. The signature includes the signing certificate, its intermediate CA certificate, and information about the signing algorithm.
    #[serde(rename = "signature")]
    pub signature: String,
    #[serde(rename = "header")]
    pub header: models::ApplePayTokenHeader,
}

impl ApplePayToken {
    pub fn new(version: Version, data: String, signature: String, header: models::ApplePayTokenHeader) -> ApplePayToken {
        ApplePayToken {
            version,
            data,
            signature,
            header,
        }
    }
}
/// ApplePay token version information.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "EC_v1")]
    EcV1,
    #[serde(rename = "RSA_v1")]
    RsaV1,
}

impl Default for Version {
    fn default() -> Version {
        Self::EcV1
    }
}

