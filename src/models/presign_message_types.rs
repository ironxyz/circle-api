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

/// PresignMessageTypes : The data schema of the message
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PresignMessageTypes {
    /// Data schema of the domain field in typedData. It's a list of (name, type) pair
    #[serde(rename = "EIP712Domain", skip_serializing_if = "Option::is_none")]
    pub eip712_domain: Option<Vec<models::PresignMessageTypesItems>>,
    /// Data schema of the message in typedData. It's a list of (name, type) pair
    #[serde(rename = "TransferWithAuthorization", skip_serializing_if = "Option::is_none")]
    pub transfer_with_authorization: Option<Vec<models::PresignMessageTypesItems>>,
}

impl PresignMessageTypes {
    /// The data schema of the message
    pub fn new() -> PresignMessageTypes {
        PresignMessageTypes {
            eip712_domain: None,
            transfer_with_authorization: None,
        }
    }
}

