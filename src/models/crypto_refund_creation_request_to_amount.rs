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

/// CryptoRefundCreationRequestToAmount : The destination amount of the refund, it must be in the original payment currency.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CryptoRefundCreationRequestToAmount {
    /// Magnitude of the amount, in units of the currency, with a `.`.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Currency code.
    #[serde(rename = "currency")]
    pub currency: Currency,
}

impl CryptoRefundCreationRequestToAmount {
    /// The destination amount of the refund, it must be in the original payment currency.
    pub fn new(amount: String, currency: Currency) -> CryptoRefundCreationRequestToAmount {
        CryptoRefundCreationRequestToAmount {
            amount,
            currency,
        }
    }
}
/// Currency code.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "ETH")]
    Eth,
    #[serde(rename = "BTC")]
    Btc,
}

impl Default for Currency {
    fn default() -> Currency {
        Self::Usd
    }
}

