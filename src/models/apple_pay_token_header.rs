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

/// ApplePayTokenHeader : Additional version-dependent information used to decrypt and verify the payment.
/// Additional version-dependent information used to decrypt and verify the payment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApplePayTokenHeader {
    ApplePayTokenEcHeader(models::ApplePayTokenEcHeader),
    ApplePayTokenRsaHeader(models::ApplePayTokenRsaHeader),
}

impl Default for ApplePayTokenHeader {
    fn default() -> Self {
        Self::ApplePayTokenEcHeader(Default::default())
    }
}

