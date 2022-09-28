use super::*;

pub type VoteIndex = u32;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub enum VoteState {
    Idle,
    Voting,
    Ended,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen, Default)]
pub struct Candidate {
    pub aadhaar_id: AadhaarId,
    pub vote_count: u16, 
}
