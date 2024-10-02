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
pub struct BillingDetails {
    /// Full name of the card or bank account holder.
    #[serde(rename = "name")]
    pub name: String,
    /// City portion of the address.
    #[serde(rename = "city")]
    pub city: String,
    /// Country portion of the address. Formatted as a two-letter country code specified in ISO 3166-1 alpha-2.
    #[serde(rename = "country")]
    pub country: String,
    /// Line one of the street address.
    #[serde(rename = "line1")]
    pub line1: String,
    /// Line two of the street address.
    #[serde(rename = "line2", skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// State / County / Province / Region portion of the address. If the country is US or Canada, then district is required and should use the two-letter code for the subdivision.
    #[serde(rename = "district", skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    /// Postal / ZIP code of the address.
    #[serde(rename = "postalCode")]
    pub postal_code: String,
}

impl BillingDetails {
    pub fn new(name: String, city: String, country: String, line1: String, postal_code: String) -> BillingDetails {
        BillingDetails {
            name,
            city,
            country,
            line1,
            line2: None,
            district: None,
            postal_code,
        }
    }
}

