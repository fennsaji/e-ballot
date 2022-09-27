use super::*;

use sp_core::sr25519;

pub type PublicKey = sr25519::Public;
pub type AadhaarId = [u8; 16];

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Aadhaar<AccountId> {
    pub aadhaar_id: AadhaarId,
    pub account_id: AccountId,
}
