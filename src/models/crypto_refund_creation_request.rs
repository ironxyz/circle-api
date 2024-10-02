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
pub struct CryptoRefundCreationRequest {
    /// Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests.
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: uuid::Uuid,
    #[serde(rename = "destination")]
    pub destination: models::CryptoRefundDestination,
    #[serde(rename = "amount")]
    pub amount: models::CryptoRefundCreationRequestAmount,
    #[serde(rename = "toAmount")]
    pub to_amount: models::CryptoRefundCreationRequestToAmount,
}

impl CryptoRefundCreationRequest {
    pub fn new(idempotency_key: uuid::Uuid, destination: models::CryptoRefundDestination, amount: models::CryptoRefundCreationRequestAmount, to_amount: models::CryptoRefundCreationRequestToAmount) -> CryptoRefundCreationRequest {
        CryptoRefundCreationRequest {
            idempotency_key,
            destination,
            amount,
            to_amount,
        }
    }
}

