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
pub struct AddressBookRecipientRequest {
    /// Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests.
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: uuid::Uuid,
    #[serde(rename = "chain")]
    pub chain: models::Chain,
    /// An alphanumeric string representing a blockchain address. Will be in different formats for different chains. It is important to preserve the exact formatting and capitalization of the address.
    #[serde(rename = "address")]
    pub address: String,
    /// The secondary identifier for a blockchain address. An example of this is the memo field on the Stellar network, which can be text, id, or hash format.
    #[serde(rename = "addressTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_tag: Option<Option<String>>,
    #[serde(rename = "metadata")]
    pub metadata: Box<models::AddressBookRecipientMetadata>,
}

impl AddressBookRecipientRequest {
    pub fn new(idempotency_key: uuid::Uuid, chain: models::Chain, address: String, metadata: models::AddressBookRecipientMetadata) -> AddressBookRecipientRequest {
        AddressBookRecipientRequest {
            idempotency_key,
            chain,
            address,
            address_tag: None,
            metadata: Box::new(metadata),
        }
    }
}

