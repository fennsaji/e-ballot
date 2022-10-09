use super::*;

use sp_core::sr25519;

pub type PublicKey = sr25519::Public;
pub type AadhaarId = [u8; 16];

/// Aadhaar type to register user
/// TODO: Add username
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Aadhaar<AccountId> {
    /// Aadhaar id of user
    pub aadhaar_id: AadhaarId,
    /// Linked user for the account
    pub account_id: AccountId,
}
