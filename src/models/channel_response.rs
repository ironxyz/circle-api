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
pub struct ChannelResponse {
    /// Unique system generated identifier for the entity.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Flag to indicate whether the channel is configured as default. At most one of the channels will have this flag set to true and the default channel is used when a payment request does not have the `channel` property set.
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// Descriptor that appears on cardholders' bank statements for card payments submitted through this channel.
    #[serde(rename = "cardDescriptor", skip_serializing_if = "Option::is_none")]
    pub card_descriptor: Option<String>,
    /// Descriptor that appears on end-users' bank statements for ACH payments submitted through this channel.
    #[serde(rename = "achDescriptor", skip_serializing_if = "Option::is_none")]
    pub ach_descriptor: Option<String>,
}

impl ChannelResponse {
    pub fn new() -> ChannelResponse {
        ChannelResponse {
            id: None,
            default: None,
            card_descriptor: None,
            ach_descriptor: None,
        }
    }
}

