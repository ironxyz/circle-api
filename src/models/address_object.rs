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
pub struct AddressObject {
    /// An alphanumeric string representing a blockchain address. Will be in different formats for different chains. It is important to preserve the exact formatting and capitalization of the address.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The secondary identifier for a blockchain address. An example of this is the memo field on the Stellar network, which can be text, id, or hash format.
    #[serde(rename = "addressTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_tag: Option<Option<String>>,
    #[serde(rename = "currency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Option<models::Currency>>,
    #[serde(rename = "chain", skip_serializing_if = "Option::is_none")]
    pub chain: Option<models::Chain>,
}

impl AddressObject {
    pub fn new() -> AddressObject {
        AddressObject {
            address: None,
            address_tag: None,
            currency: None,
            chain: None,
        }
    }
}

