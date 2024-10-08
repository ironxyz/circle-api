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
pub struct PayoutMoney {
    /// Magnitude of the amount, in units of the currency, with a `.`.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Currency code for the amount.
    #[serde(rename = "currency")]
    pub currency: Currency,
}

impl PayoutMoney {
    pub fn new(amount: String, currency: Currency) -> PayoutMoney {
        PayoutMoney {
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
    #[serde(rename = "MTC")]
    Mtc,
    #[serde(rename = "FLW")]
    Flw,
    #[serde(rename = "MAN")]
    Man,
}

impl Default for Currency {
    fn default() -> Currency {
        Self::Usd
    }
}

