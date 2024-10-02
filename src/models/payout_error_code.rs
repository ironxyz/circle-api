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

/// PayoutErrorCode : Indicates the failure reason of a payout. Only present for payouts in failed state. Possible values are [`insufficient_funds`, `transaction_denied`, `transaction_failed`, `transaction_returned`, `bank_transaction_error`, `fiat_account_limit_exceeded`, `invalid_bank_account_number`, `invalid_ach_rtn`, `invalid_wire_rtn`, `vendor_inactive`]'
/// Indicates the failure reason of a payout. Only present for payouts in failed state. Possible values are [`insufficient_funds`, `transaction_denied`, `transaction_failed`, `transaction_returned`, `bank_transaction_error`, `fiat_account_limit_exceeded`, `invalid_bank_account_number`, `invalid_ach_rtn`, `invalid_wire_rtn`, `vendor_inactive`]'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PayoutErrorCode {
    #[serde(rename = "insufficient_funds")]
    InsufficientFunds,
    #[serde(rename = "transaction_denied")]
    TransactionDenied,
    #[serde(rename = "transaction_failed")]
    TransactionFailed,
    #[serde(rename = "transaction_returned")]
    TransactionReturned,
    #[serde(rename = "bank_transaction_error")]
    BankTransactionError,
    #[serde(rename = "fiat_account_limit_exceeded")]
    FiatAccountLimitExceeded,
    #[serde(rename = "invalid_bank_account_number")]
    InvalidBankAccountNumber,
    #[serde(rename = "invalid_ach_rtn")]
    InvalidAchRtn,
    #[serde(rename = "invalid_wire_rtn")]
    InvalidWireRtn,
    #[serde(rename = "vendor_inactive")]
    VendorInactive,

}

impl std::fmt::Display for PayoutErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InsufficientFunds => write!(f, "insufficient_funds"),
            Self::TransactionDenied => write!(f, "transaction_denied"),
            Self::TransactionFailed => write!(f, "transaction_failed"),
            Self::TransactionReturned => write!(f, "transaction_returned"),
            Self::BankTransactionError => write!(f, "bank_transaction_error"),
            Self::FiatAccountLimitExceeded => write!(f, "fiat_account_limit_exceeded"),
            Self::InvalidBankAccountNumber => write!(f, "invalid_bank_account_number"),
            Self::InvalidAchRtn => write!(f, "invalid_ach_rtn"),
            Self::InvalidWireRtn => write!(f, "invalid_wire_rtn"),
            Self::VendorInactive => write!(f, "vendor_inactive"),
        }
    }
}

impl Default for PayoutErrorCode {
    fn default() -> PayoutErrorCode {
        Self::InsufficientFunds
    }
}

