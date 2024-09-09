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
pub struct Stablecoin {
    /// Name of the stablecoin.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Symbol of the stablecoin.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Total circulating amount of the stablecoin.
    #[serde(rename = "totalAmount", skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<String>,
    /// A list of the broken down totalAmount by chain of the stablecoin.
    #[serde(rename = "chains", skip_serializing_if = "Option::is_none")]
    pub chains: Option<Vec<models::TokenAmount>>,
}

impl Stablecoin {
    pub fn new() -> Stablecoin {
        Stablecoin {
            name: None,
            symbol: None,
            total_amount: None,
            chains: None,
        }
    }
}

