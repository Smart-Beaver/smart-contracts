//! Error types for the PSP22 token standard.

use ink::prelude::string::String;

/// Errors that may occur during PSP22 token operations.
///
/// This enum defines various errors, like insufficient balance or allowance,
/// that can occur during token transactions.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP22Error {
    /// Custom error type for implementation-based errors.
    Custom(String),
    InsufficientBalance,
    InsufficientAllowance,
}

/// Errors related to ownership operations.
///
/// This enum is used for managing errors that occur in ownership-related
/// functionalities.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OwnableError {
}
