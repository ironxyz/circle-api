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

/// Eci : ECI (electronic commerce indicator) value returned by Directory Servers (namely Visa, MasterCard, JCB, and American Express) indicating the outcome of authentication attempted on transactions enforced by 3DS.
/// ECI (electronic commerce indicator) value returned by Directory Servers (namely Visa, MasterCard, JCB, and American Express) indicating the outcome of authentication attempted on transactions enforced by 3DS.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Eci {
    #[serde(rename = "00")]
    Variant00,
    #[serde(rename = "01")]
    Variant01,
    #[serde(rename = "02")]
    Variant02,
    #[serde(rename = "05")]
    Variant05,
    #[serde(rename = "06")]
    Variant06,
    #[serde(rename = "07")]
    Variant07,

}

impl std::fmt::Display for Eci {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant00 => write!(f, "00"),
            Self::Variant01 => write!(f, "01"),
            Self::Variant02 => write!(f, "02"),
            Self::Variant05 => write!(f, "05"),
            Self::Variant06 => write!(f, "06"),
            Self::Variant07 => write!(f, "07"),
        }
    }
}

impl Default for Eci {
    fn default() -> Eci {
        Self::Variant00
    }
}

