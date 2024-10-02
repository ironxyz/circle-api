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

/// TransferDestinationWalletLocation : A destination wallet location.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferDestinationWalletLocation {
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The id of the wallet.
    #[serde(rename = "id")]
    pub id: String,
    /// An alphanumeric string which indicates the wallet address used to receive the transfer. Will only be set when the transfer source is a blockchain address.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The secondary identifier for a blockchain address. An example of this is the memo field on the Stellar network, which can be text, id, or hash format.
    #[serde(rename = "addressTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_tag: Option<Option<String>>,
}

impl TransferDestinationWalletLocation {
    /// A destination wallet location.
    pub fn new(r#type: Type, id: String) -> TransferDestinationWalletLocation {
        TransferDestinationWalletLocation {
            r#type,
            id,
            address: None,
            address_tag: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "wallet")]
    Wallet,
}

impl Default for Type {
    fn default() -> Type {
        Self::Wallet
    }
}

