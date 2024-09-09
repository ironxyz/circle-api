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

/// BusinessRecipientAddressCreationRequest : Adds a recipient address. The currency parameter will default to USD for all chains except for BTC where it defaults to BTC.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessRecipientAddressCreationRequest {
    /// Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests.
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: uuid::Uuid,
    /// An alphanumeric string representing a blockchain address. Will be in different formats for different chains. It is important to preserve the exact formatting and capitalization of the address.
    #[serde(rename = "address")]
    pub address: String,
    /// The secondary identifier for a blockchain address. An example of this is the memo field on the Stellar network, which can be text, id, or hash format.
    #[serde(rename = "addressTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_tag: Option<Option<String>>,
    #[serde(rename = "chain")]
    pub chain: models::Chain,
    #[serde(rename = "currency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Option<models::Currency>>,
    /// An identifier or sentence that describes the recipient.
    #[serde(rename = "description")]
    pub description: String,
}

impl BusinessRecipientAddressCreationRequest {
    /// Adds a recipient address. The currency parameter will default to USD for all chains except for BTC where it defaults to BTC.
    pub fn new(idempotency_key: uuid::Uuid, address: String, chain: models::Chain, description: String) -> BusinessRecipientAddressCreationRequest {
        BusinessRecipientAddressCreationRequest {
            idempotency_key,
            address,
            address_tag: None,
            chain,
            currency: None,
            description,
        }
    }
}

