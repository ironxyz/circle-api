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

/// CryptoRefundDestination : The destination of a crypto refund.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CryptoRefundDestination {
    /// The blockchain address.
    #[serde(rename = "address")]
    pub address: String,
    /// The secondary identifier for a blockchain address. An example of this is the memo field on the Stellar network, which can be text, id, or hash format.
    #[serde(rename = "addressTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_tag: Option<Option<String>>,
    #[serde(rename = "chain")]
    pub chain: models::Chain,
}

impl CryptoRefundDestination {
    /// The destination of a crypto refund.
    pub fn new(address: String, chain: models::Chain) -> CryptoRefundDestination {
        CryptoRefundDestination {
            address,
            address_tag: None,
            chain,
        }
    }
}

