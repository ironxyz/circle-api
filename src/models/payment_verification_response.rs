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

/// PaymentVerificationResponse : Indicates the status of the payment verification. This property will be present once the payment is confirmed.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentVerificationResponse {
    /// Status of the AVS check. Raw AVS response, expressed as an upper-case letter. `not_requested` indicates check was not made. `pending` is pending/processing.
    #[serde(rename = "avs")]
    pub avs: String,
    #[serde(rename = "cvv")]
    pub cvv: models::CvvResults,
    #[serde(rename = "threeDSecure", skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<models::ThreeDsResult>,
    #[serde(rename = "eci", skip_serializing_if = "Option::is_none")]
    pub eci: Option<models::Eci>,
}

impl PaymentVerificationResponse {
    /// Indicates the status of the payment verification. This property will be present once the payment is confirmed.
    pub fn new(avs: String, cvv: models::CvvResults) -> PaymentVerificationResponse {
        PaymentVerificationResponse {
            avs,
            cvv,
            three_d_secure: None,
            eci: None,
        }
    }
}

