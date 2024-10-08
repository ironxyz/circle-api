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

/// Chain : A blockchain that a given currency is available on.
/// A blockchain that a given currency is available on.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Chain {
    #[serde(rename = "ALGO")]
    Algo,
    #[serde(rename = "ARB")]
    Arb,
    #[serde(rename = "AVAX")]
    Avax,
    #[serde(rename = "BASE")]
    Base,
    #[serde(rename = "BTC")]
    Btc,
    #[serde(rename = "CELO")]
    Celo,
    #[serde(rename = "ETH")]
    Eth,
    #[serde(rename = "FLOW")]
    Flow,
    #[serde(rename = "HBAR")]
    Hbar,
    #[serde(rename = "NEAR")]
    Near,
    #[serde(rename = "NOBLE")]
    Noble,
    #[serde(rename = "OP")]
    Op,
    #[serde(rename = "PAH")]
    Pah,
    #[serde(rename = "POLY")]
    Poly,
    #[serde(rename = "SOL")]
    Sol,
    #[serde(rename = "TRX")]
    Trx,
    #[serde(rename = "XLM")]
    Xlm,

}

impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Algo => write!(f, "ALGO"),
            Self::Arb => write!(f, "ARB"),
            Self::Avax => write!(f, "AVAX"),
            Self::Base => write!(f, "BASE"),
            Self::Btc => write!(f, "BTC"),
            Self::Celo => write!(f, "CELO"),
            Self::Eth => write!(f, "ETH"),
            Self::Flow => write!(f, "FLOW"),
            Self::Hbar => write!(f, "HBAR"),
            Self::Near => write!(f, "NEAR"),
            Self::Noble => write!(f, "NOBLE"),
            Self::Op => write!(f, "OP"),
            Self::Pah => write!(f, "PAH"),
            Self::Poly => write!(f, "POLY"),
            Self::Sol => write!(f, "SOL"),
            Self::Trx => write!(f, "TRX"),
            Self::Xlm => write!(f, "XLM"),
        }
    }
}

impl Default for Chain {
    fn default() -> Chain {
        Self::Algo
    }
}

