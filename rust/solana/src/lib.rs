#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;

pub mod oracle_program;
pub use oracle_program::*;

pub mod attestation_program;
pub use attestation_program::*;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "client")]
pub use client::*;

#[cfg(feature = "client")]
pub mod sgx;
#[cfg(feature = "client")]
pub use sgx::*;

/// Seed used to derive the SbState PDA.
pub const STATE_SEED: &[u8] = b"STATE";

/// Seed used to derive the PermissionAccountData PDA.
pub const PERMISSION_SEED: &[u8] = b"PermissionAccountData";

/// Seed used to derive the LeaseAccountData PDA.
pub const LEASE_SEED: &[u8] = b"LeaseAccountData";

/// Seed used to derive the OracleAccountData PDA.
pub const ORACLE_SEED: &[u8] = b"OracleAccountData";

/// Seed used to derive the SlidingWindow PDA.
pub const SLIDING_RESULT_SEED: &[u8] = b"SlidingResultAccountData";

/// Discriminator used for Switchboard buffer accounts.
pub const BUFFER_DISCRIMINATOR: &[u8] = b"BUFFERxx";

/// Seed used to derive the FunctionAccountData PDA.
pub const FUNCTION_SEED: &[u8] = b"FunctionAccountData";

/// Seed used to derive the QuoteAccountData PDA.
pub const QUOTE_SEED: &[u8] = b"QuoteAccountData";

/// Program id for the Switchboard oracle program
/// SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f
pub const SWITCHBOARD_PROGRAM_ID: anchor_lang::solana_program::pubkey::Pubkey =
    pubkey!("SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f");

/// Program id for the Switchboard oracle program
/// 2No5FVKPAAYqytpkEoq93tVh33fo4p6DgAnm4S6oZHo7
pub const SWITCHBOARD_ATTESTATION_PROGRAM_ID: Pubkey =
    pubkey!("2No5FVKPAAYqytpkEoq93tVh33fo4p6DgAnm4S6oZHo7");

declare_id!(SWITCHBOARD_PROGRAM_ID);
