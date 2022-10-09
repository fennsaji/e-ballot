#![cfg_attr(not(feature = "std"), no_std)]

/// This pallet allows to cast vote on a voting session
/// Anyone with Election Commission Origin can start a 
/// voting session by calling start_voting and then
/// add candidates. Then users can vote for candidates


use frame_support::{
    codec::{ Decode, Encode, MaxEncodedLen },
};
use scale_info::TypeInfo;
use pallet_aadhaar::types::AadhaarId;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_std::prelude::*;
use crate::types::*;

mod types;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;


pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	pub type AadhaarPallet<T> = pallet_aadhaar::Pallet::<T>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_aadhaar::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Origin who manage voting session
		type ElectionCommissionOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;
	}

	/// Stores the current index of voting session
	#[pallet::type_value]
	pub fn VoteIndexDefault() -> VoteIndex { 0 }
	#[pallet::storage]
	#[pallet::getter(fn current_vote_index)]
	pub type CurrentVoteIndex<T> = StorageValue<Value = VoteIndex, OnEmpty = VoteIndexDefault, QueryKind = ValueQuery>;


	/// It stores the chief commissioner/ admin of the given vote index
	#[pallet::storage]
	#[pallet::getter(fn chief_commissioner)]
	pub type ChiefCommissioner<T> = StorageMap<_, Blake2_128Concat, VoteIndex, AadhaarId>;


	/// It stores voting state (Idle, Voting and Ended) of given vote index
	#[pallet::type_value]
	pub fn StateDefault() -> VoteState { VoteState::Idle }
	#[pallet::storage]
	#[pallet::getter(fn vote_state)]
	pub type VotingState<T> = StorageMap<Hasher = Blake2_128Concat, Key = VoteIndex, Value = VoteState, OnEmpty = StateDefault, QueryKind = ValueQuery>;


	/// It stores candidates and their votes in the given voting session
	#[pallet::storage]
	#[pallet::getter(fn candidates)]
	pub type Candidates<T> = StorageDoubleMap<_, Blake2_128Concat, VoteIndex, Blake2_128Concat, AadhaarId, Candidate, ValueQuery>;


	/// It stores if votes were casted on an voting session by a voter id
	#[pallet::storage]
	#[pallet::getter(fn votes)]
	pub type Votes<T> = StorageDoubleMap<_, Blake2_128Concat, VoteIndex, Blake2_128Concat, AadhaarId, bool, ValueQuery>;



	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Voting started by Aadhaar Id
		VotingStarted { aadhaar_id: AadhaarId, vote_index: VoteIndex },
		/// Voting started by Aadhaar Id
		VotingStopped { aadhaar_id: AadhaarId, vote_index: VoteIndex },
		/// Voting started by Aadhaar Id
		VotingReset { aadhaar_id: AadhaarId, vote_index: VoteIndex },
		/// Added Candidates
		AddedCandidates { vote_index: VoteIndex, candidates: Vec<T::AccountId> },
		/// Voted
		Voted { vote_index: VoteIndex, candidate: AadhaarId }
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Vote session not found
		VoteSessionNotFound,
		/// Already Cast Vote
		VoteAlreadyCast,
		/// Voting Not Active
		VotingNotActive,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Start a voting session
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn start_voting(origin: OriginFor<T>) -> DispatchResult {
			// Check if origin is a from a validator
			let account_id = T::ElectionCommissionOrigin::ensure_origin(origin)?;

			// Create voting session and set status to Voting
			let (aadhaar_id, vote_index) = Self::do_start_voting(&account_id)?;

			// Emit an event.
			Self::deposit_event(Event::VotingStarted { aadhaar_id, vote_index });

			Ok(())
		}

		/// Stop a voting session. After winner can be chosen based on no of votes on candidates
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn stop_voting(origin: OriginFor<T>, vote_index: VoteIndex) -> DispatchResult {
			// Check if origin is a from a validator
			let account_id = T::ElectionCommissionOrigin::ensure_origin(origin)?;

			// Change voting state to ended
			let (aadhaar_id, vote_index) = Self::do_stop_voting(&account_id, vote_index)?;

			// Emit an event.
			Self::deposit_event(Event::VotingStarted { aadhaar_id, vote_index });

			Ok(())
		}

		/// Remove voting data
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn reset_voting(origin: OriginFor<T>, vote_index: VoteIndex) -> DispatchResult {
			// Check if origin is a from a validator
			let account_id = T::ElectionCommissionOrigin::ensure_origin(origin)?;

			// Set voting state to Idle and remove candidates and votes
			let (aadhaar_id, vote_index) = Self::do_reset_voting(&account_id, vote_index)?;

			// Emit an event.
			Self::deposit_event(Event::VotingStarted { aadhaar_id, vote_index });

			Ok(())
		}

		/// Add candidates to voting session
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_candidates(origin: OriginFor<T>, vote_index: VoteIndex, candidates: Vec<T::AccountId>) -> DispatchResult {
			// Check if origin is a from a validator
			T::ElectionCommissionOrigin::ensure_origin(origin)?;

			// Add Candidates to the voting session
			Self::do_add_candidates(vote_index, &candidates)?;

			// Emit an event.
			Self::deposit_event(Event::AddedCandidates { vote_index, candidates });

			Ok(())
		}

		/// Vote on session by aadhaar registered users
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn vote(origin: OriginFor<T>, vote_index: VoteIndex, candidate: AadhaarId) -> DispatchResult {
			// Check if origin is a from a validator
			let account_id = ensure_signed(origin)?;

			// Cast vote on a candidate on given voting session
			Self::do_vote(&account_id, vote_index, candidate)?;

			// Emit an event.
			Self::deposit_event(Event::Voted { vote_index, candidate });

			Ok(())
		}

	}


	impl<T: Config> Pallet<T> {

		/// Create voting session and set status to Voting
		pub fn do_start_voting(account_id: &T::AccountId) -> Result<(AadhaarId, VoteIndex), DispatchError> {

			let aadhaar_id = AadhaarPallet::<T>::get_aadhaar_id(&account_id);
			let vote_index = Self::current_vote_index();
			VotingState::<T>::set(vote_index, VoteState::Voting);
			ChiefCommissioner::<T>::set(vote_index, aadhaar_id);
			CurrentVoteIndex::<T>::set(vote_index.saturating_add(1));

			Ok((aadhaar_id.unwrap(), vote_index))
		}

		/// Set Voting status of a voting session to Ended
		pub fn do_stop_voting(account_id: &T::AccountId, vote_index: VoteIndex) -> Result<(AadhaarId, VoteIndex), DispatchError> {

			ensure!(VotingState::<T>::get(vote_index) == VoteState::Voting, Error::<T>::VotingNotActive);

			let aadhaar_id = AadhaarPallet::<T>::get_aadhaar_id(&account_id).unwrap();
			VotingState::<T>::set(vote_index, VoteState::Ended);

			Ok((aadhaar_id, vote_index))
		}

		/// Remove all voting session data and set voting status to ended
		pub fn do_reset_voting(account_id: &T::AccountId, vote_index: VoteIndex) -> Result<(AadhaarId, VoteIndex), DispatchError> {

			let aadhaar_id = AadhaarPallet::<T>::get_aadhaar_id(&account_id).unwrap();
			VotingState::<T>::set(vote_index, VoteState::Idle);
			ChiefCommissioner::<T>::remove(vote_index);

			// No of record to delete 
			//TODO: To be updated to a config and set limit on max candidates
			let limit = 20; 
			let _ = Candidates::<T>::clear_prefix(vote_index, limit, None);
			let _ = Votes::<T>::clear_prefix(vote_index, limit, None);

			Ok((aadhaar_id, vote_index))
		}

		/// Add candidates for voting session
		pub fn do_add_candidates(vote_index: VoteIndex, candidates: &Vec<T::AccountId>) -> DispatchResult {
			// Add candidates to the storage
			candidates.into_iter().for_each(|candidate| {
				let candidate_id = AadhaarPallet::<T>::get_aadhaar_id(candidate);
				if let Some(candidate_id) = candidate_id {
					Candidates::<T>::set(vote_index, candidate_id, Candidate {
						aadhaar_id: candidate_id,
						vote_count: Default::default(),
					});
				}
			});

			Ok(())
		}

		/// Cast vote on candidate
		pub fn do_vote(voter_acc: &T::AccountId, vote_index: VoteIndex, candidate_id: AadhaarId) -> DispatchResult {
			let voter_id = AadhaarPallet::<T>::get_aadhaar_id(&voter_acc).unwrap();

			// Ensure voting session exists and is active and user didn't vote already
			ensure!(Candidates::<T>::contains_key(vote_index, candidate_id), Error::<T>::VoteSessionNotFound);
			ensure!(VotingState::<T>::get(vote_index) == VoteState::Voting, Error::<T>::VotingNotActive);
			ensure!(Votes::<T>::contains_key(vote_index, voter_id) == false, Error::<T>::VoteAlreadyCast);

			let mut candidate: Candidate = Candidates::<T>::get(vote_index, candidate_id);
			candidate.vote_count = candidate.vote_count + 1;
			Candidates::<T>::set(vote_index, candidate_id, candidate);
			Votes::<T>::set(vote_index, voter_id, true);

			Ok(())
		}

	}
}
