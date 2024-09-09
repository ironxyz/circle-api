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
pub struct CryptoRefundCreationRequest {
    /// Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests.
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: uuid::Uuid,
    #[serde(rename = "destination")]
    pub destination: Box<models::CryptoRefundDestination>,
    #[serde(rename = "amount")]
    pub amount: Box<models::CryptoRefundCreationRequestAmount>,
    #[serde(rename = "toAmount")]
    pub to_amount: Box<models::CryptoRefundCreationRequestToAmount>,
}

impl CryptoRefundCreationRequest {
    pub fn new(idempotency_key: uuid::Uuid, destination: models::CryptoRefundDestination, amount: models::CryptoRefundCreationRequestAmount, to_amount: models::CryptoRefundCreationRequestToAmount) -> CryptoRefundCreationRequest {
        CryptoRefundCreationRequest {
            idempotency_key,
            destination: Box::new(destination),
            amount: Box::new(amount),
            to_amount: Box::new(to_amount),
        }
    }
}

