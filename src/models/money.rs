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
pub struct Money {
    /// Magnitude of the amount, in units of the currency, with a `.`.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Currency code for the amount.
    #[serde(rename = "currency")]
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: String, currency: Currency) -> Money {
        Money {
            amount,
            currency,
        }
    }
}
/// Currency code for the amount.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "BTC")]
    Btc,
    #[serde(rename = "ETH")]
    Eth,
}

impl Default for Currency {
    fn default() -> Currency {
        Self::Usd
    }
}

