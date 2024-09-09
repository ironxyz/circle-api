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
pub struct CryptoPaymentsOptionalAmountMoney {
    /// Magnitude of the amount, in units of the currency, with a `.`.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Currency code.
    #[serde(rename = "currency")]
    pub currency: Currency,
}

impl CryptoPaymentsOptionalAmountMoney {
    pub fn new(currency: Currency) -> CryptoPaymentsOptionalAmountMoney {
        CryptoPaymentsOptionalAmountMoney {
            amount: None,
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

