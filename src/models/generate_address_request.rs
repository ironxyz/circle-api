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
pub struct GenerateAddressRequest {
    /// Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests.
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: uuid::Uuid,
    #[serde(rename = "currency", deserialize_with = "Option::deserialize")]
    pub currency: Option<models::Currency>,
    #[serde(rename = "chain")]
    pub chain: models::Chain,
}

impl GenerateAddressRequest {
    pub fn new(idempotency_key: uuid::Uuid, currency: Option<models::Currency>, chain: models::Chain) -> GenerateAddressRequest {
        GenerateAddressRequest {
            idempotency_key,
            currency,
            chain,
        }
    }
}

