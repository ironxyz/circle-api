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
pub struct BasicChargeback {
    /// Unique system generated identifier for the entity.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Unique system generated identifier for the payment that is associated to the chargeback item.
    #[serde(rename = "paymentId")]
    pub payment_id: uuid::Uuid,
    /// Unique system generated identifier for the merchant.
    #[serde(rename = "merchantId")]
    pub merchant_id: uuid::Uuid,
    /// Reason code given by the card network for the chargeback item.
    #[serde(rename = "reasonCode")]
    pub reason_code: String,
    #[serde(rename = "status")]
    pub status: models::ChargebackStatus,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<models::ChargebackCategories>,
    /// The chargeback item's history list will be sorted by create date descending: more recent chargeback statuses will be at the beginning of the list. 
    #[serde(rename = "history")]
    pub history: Vec<models::BasicChargebackHistory>,
}

impl BasicChargeback {
    pub fn new(id: uuid::Uuid, payment_id: uuid::Uuid, merchant_id: uuid::Uuid, reason_code: String, status: models::ChargebackStatus, history: Vec<models::BasicChargebackHistory>) -> BasicChargeback {
        BasicChargeback {
            id,
            payment_id,
            merchant_id,
            reason_code,
            status,
            category: None,
            history,
        }
    }
}

