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

/// Transfer : A transfer of funds.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    /// Unique system generated identifier for the entity.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "source")]
    pub source: models::TransferSourceLocation,
    #[serde(rename = "destination")]
    pub destination: models::TransferDestinationLocation,
    #[serde(rename = "amount")]
    pub amount: models::Money,
    /// An array of fees applied to a transaction. This is only available when there is at least one non-zero fee.
    #[serde(rename = "fees", skip_serializing_if = "Option::is_none")]
    pub fees: Option<Vec<models::Fee>>,
    /// A hash that uniquely identifies the onchain transaction. This is only available where either source or destination are of type blockchain.
    #[serde(rename = "transactionHash", skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    /// Status of the transfer. Status `pending` indicates that the transfer is in the process of running; `complete` indicates it finished successfully; `failed` indicates it failed. Circle Mint Singapore customers may have transfers in the `pending` status if the recipient addresses are not verified.
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "errorCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<Option<models::TransferErrorCode>>,
    /// The create date of the transfer.
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

impl Transfer {
    /// A transfer of funds.
    pub fn new(id: uuid::Uuid, source: models::TransferSourceLocation, destination: models::TransferDestinationLocation, amount: models::Money, status: Status) -> Transfer {
        Transfer {
            id,
            source,
            destination,
            amount,
            fees: None,
            transaction_hash: None,
            status,
            error_code: None,
            create_date: None,
        }
    }
}
/// Status of the transfer. Status `pending` indicates that the transfer is in the process of running; `complete` indicates it finished successfully; `failed` indicates it failed. Circle Mint Singapore customers may have transfers in the `pending` status if the recipient addresses are not verified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

