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
pub struct PixInstruction {
    /// Circle tracking reference that needs to be set in the PIX reference field.
    #[serde(rename = "trackingRef", skip_serializing_if = "Option::is_none")]
    pub tracking_ref: Option<String>,
    /// ISPB of beneficiary's bank.
    #[serde(rename = "ispb", skip_serializing_if = "Option::is_none")]
    pub ispb: Option<String>,
    /// Beneficiary account branch code.
    #[serde(rename = "branchCode", skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    /// Beneficiary account number.
    #[serde(rename = "accountNumber", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<models::PixAccountType>,
    /// Beneficiary Tax ID.
    #[serde(rename = "taxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// Name of the beneficiary.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PixInstruction {
    pub fn new() -> PixInstruction {
        PixInstruction {
            tracking_ref: None,
            ispb: None,
            branch_code: None,
            account_number: None,
            account_type: None,
            tax_id: None,
            name: None,
        }
    }
}

