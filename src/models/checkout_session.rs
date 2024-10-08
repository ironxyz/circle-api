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
pub struct CheckoutSession {
    /// Unique system generated identifier for the entity.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The type of this response
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The URL returned to you through client-side callback when the payment is completed.
    #[serde(rename = "successUrl", skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    /// The access token for the SDK to access checkout session resources
    #[serde(rename = "clientToken")]
    pub client_token: String,
    #[serde(rename = "status")]
    pub status: models::CheckoutSessionStatus,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "expiresOn")]
    pub expires_on: String,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "createDate")]
    pub create_date: String,
    /// ISO-8601 UTC date/time format.
    #[serde(rename = "updateDate")]
    pub update_date: String,
    #[serde(rename = "amount")]
    pub amount: models::CheckoutSessionMoney,
    #[serde(rename = "amountPaid")]
    pub amount_paid: models::CheckoutSessionMoney,
    /// IDs of all the associated payments.
    #[serde(rename = "paymentIds")]
    pub payment_ids: Vec<uuid::Uuid>,
    /// IDs of all the associated payment intents.
    #[serde(rename = "paymentIntentIds")]
    pub payment_intent_ids: Vec<uuid::Uuid>,
}

impl CheckoutSession {
    pub fn new(id: uuid::Uuid, r#type: Type, client_token: String, status: models::CheckoutSessionStatus, expires_on: String, create_date: String, update_date: String, amount: models::CheckoutSessionMoney, amount_paid: models::CheckoutSessionMoney, payment_ids: Vec<uuid::Uuid>, payment_intent_ids: Vec<uuid::Uuid>) -> CheckoutSession {
        CheckoutSession {
            id,
            r#type,
            success_url: None,
            client_token,
            status,
            expires_on,
            create_date,
            update_date,
            amount,
            amount_paid,
            payment_ids,
            payment_intent_ids,
        }
    }
}
/// The type of this response
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "checkout_session")]
    CheckoutSession,
}

impl Default for Type {
    fn default() -> Type {
        Self::CheckoutSession
    }
}

