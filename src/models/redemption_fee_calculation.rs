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

/// RedemptionFeeCalculation : A NET burn fee calculation for a day.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedemptionFeeCalculation {
    /// Unique system generated identifier for the entity.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<models::FiatMoney>,
    #[serde(rename = "cumulatedPayoutAmount", skip_serializing_if = "Option::is_none")]
    pub cumulated_payout_amount: Option<models::BurnFeePayoutAmount>,
    #[serde(rename = "cumulatedPaymentAmount", skip_serializing_if = "Option::is_none")]
    pub cumulated_payment_amount: Option<models::BurnFeePaymentAmount>,
    #[serde(rename = "cumulatedNetAmount", skip_serializing_if = "Option::is_none")]
    pub cumulated_net_amount: Option<models::BurnFeeNetAmount>,
    /// A date representing a day for which a fee is calculated.
    #[serde(rename = "valueDate", skip_serializing_if = "Option::is_none")]
    pub value_date: Option<String>,
    /// Fee collection status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Fee calculation reset timestamp.
    #[serde(rename = "thresholdResetTimestamp", skip_serializing_if = "Option::is_none")]
    pub threshold_reset_timestamp: Option<String>,
    /// The create date of the NET burn daily fee calculation.
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

impl RedemptionFeeCalculation {
    /// A NET burn fee calculation for a day.
    pub fn new() -> RedemptionFeeCalculation {
        RedemptionFeeCalculation {
            id: None,
            fee: None,
            cumulated_payout_amount: None,
            cumulated_payment_amount: None,
            cumulated_net_amount: None,
            value_date: None,
            status: None,
            threshold_reset_timestamp: None,
            create_date: None,
        }
    }
}
/// Fee collection status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "paid")]
    Paid,
}

impl Default for Status {
    fn default() -> Status {
        Self::Scheduled
    }
}

