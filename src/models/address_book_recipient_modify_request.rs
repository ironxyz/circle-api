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
pub struct AddressBookRecipientModifyRequest {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<models::AddressBookRecipientMetadata>,
}

impl AddressBookRecipientModifyRequest {
    pub fn new() -> AddressBookRecipientModifyRequest {
        AddressBookRecipientModifyRequest {
            metadata: None,
        }
    }
}

