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
pub struct CryptoPayoutCreationRequest {
    /// Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests.
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: uuid::Uuid,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<models::TransferSourceWalletLocation>,
    #[serde(rename = "destination")]
    pub destination: models::CryptoPayoutDestination,
    #[serde(rename = "amount")]
    pub amount: models::Money,
    #[serde(rename = "toAmount", skip_serializing_if = "Option::is_none")]
    pub to_amount: Option<models::ToAmount>,
}

impl CryptoPayoutCreationRequest {
    pub fn new(idempotency_key: uuid::Uuid, destination: models::CryptoPayoutDestination, amount: models::Money) -> CryptoPayoutCreationRequest {
        CryptoPayoutCreationRequest {
            idempotency_key,
            source: None,
            destination,
            amount,
            to_amount: None,
        }
    }
}

