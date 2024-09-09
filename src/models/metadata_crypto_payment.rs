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
pub struct MetadataCryptoPayment {
    /// The protocol type. so far we only support \"TransferWithAuthorization\". find more details in EIP-3009
    #[serde(rename = "type")]
    pub r#type: Type,
    /// meta transaction nonce
    #[serde(rename = "metaTxNonce")]
    pub meta_tx_nonce: String,
    /// This comes from the /presign response
    #[serde(rename = "signatureValidAfter")]
    pub signature_valid_after: String,
    /// This comes from the /presign response
    #[serde(rename = "signatureValidBefore")]
    pub signature_valid_before: String,
    /// Raw signature coming from the wallet response
    #[serde(rename = "rawSignature")]
    pub raw_signature: String,
}

impl MetadataCryptoPayment {
    pub fn new(r#type: Type, meta_tx_nonce: String, signature_valid_after: String, signature_valid_before: String, raw_signature: String) -> MetadataCryptoPayment {
        MetadataCryptoPayment {
            r#type,
            meta_tx_nonce,
            signature_valid_after,
            signature_valid_before,
            raw_signature,
        }
    }
}
/// The protocol type. so far we only support \"TransferWithAuthorization\". find more details in EIP-3009
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "TransferWithAuthorization")]
    TransferWithAuthorization,
}

impl Default for Type {
    fn default() -> Type {
        Self::TransferWithAuthorization
    }
}

