#![cfg_attr(not(feature = "std"), no_std)]

pub mod types;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	use crate::types::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Validator Origin
		type RegisterOrigin: EnsureOrigin<Self::Origin>;
	}

	#[pallet::storage]
	#[pallet::getter(fn aadhaar)]
	pub(crate) type Aadhaars<T: Config> = StorageMap<_, Blake2_128Concat, AadhaarId, Aadhaar<T::AccountId>, OptionQuery>;

	// map to enable lookup from AadhaarId to account id
	#[pallet::storage]
	pub type Lookup<T: Config> = StorageMap<_, Blake2_128Concat, AadhaarId, T::AccountId, OptionQuery>;

	// map to enable reverse lookup
	#[pallet::storage]
	pub type RLookup<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, AadhaarId, OptionQuery>;


	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub initial_aadhaars: Vec<(AadhaarId, T::AccountId)>,
		pub phantom: PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				initial_aadhaars: Default::default(),
				phantom: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			Pallet::<T>::initialize_aadhaars(&self.initial_aadhaars);
		}
	}


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New Aadhar registered
		AadhaarRegistered {
			account_id: T::AccountId,
			aadhaar_id: AadhaarId,
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Aadhaar already exists
		AadhaarAlreadyExists,

		/// Account Id already taken
		AccountIdRegistered,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn register_aadhaar(
			origin: OriginFor<T>,
			account_id: T::AccountId,
			aadhaar_id: AadhaarId,
		) -> DispatchResult {
			// Check if origin is a from a validator
			T::RegisterOrigin::ensure_origin(origin)?;
			
			Self::do_register_aadhaar(&account_id, aadhaar_id)?;

			// Emit an event.
			Self::deposit_event(Event::AadhaarRegistered { aadhaar_id, account_id });

			// Return a successful DispatchResult
			Ok(())
		}

		// TODO: Add Remove Aadhaar and Rotate Account Id Extrinsics
	}

	impl<T: Config> Pallet<T> {

		/// Simple type conversion between sr25519::Public and AccountId
		/// Should not panic for any valid sr25519 - need to make more robust to check for valid
		/// publicKey
		pub fn get_accountid_from_pubkey(pk: &PublicKey) -> T::AccountId {
			//convert a publickey to an accountId
			// TODO : Need a better way to handle the option failing?
			T::AccountId::decode(&mut &pk[..]).unwrap()
		}

		/// Initialize aadhaar during genesis
		fn initialize_aadhaars(aadhaars: &Vec<(AadhaarId, T::AccountId)>) {
			for (aadhaar_id, account_id) in aadhaars.iter() {
				Aadhaars::<T>::insert(
					aadhaar_id,
					Aadhaar {
						aadhaar_id: *aadhaar_id,
						account_id: account_id.clone(),
					}
				);

				Lookup::<T>::insert(
					aadhaar_id,
					account_id.clone(),
				);
	
				RLookup::<T>::insert(
					account_id,
					aadhaar_id,
				);
			}
		}

		pub fn do_register_aadhaar(
			account_id: &T::AccountId,
			aadhaar_id: AadhaarId,
		) -> DispatchResult {

			// ensure aadhaar is not already taken
			ensure!(
				!Lookup::<T>::contains_key(aadhaar_id), 
				Error::<T>::AadhaarAlreadyExists
			);

			// ensure the account id is not already linked to a Aadhar
			ensure!(
				!RLookup::<T>::contains_key(account_id),
				Error::<T>::AccountIdRegistered
			);

			Aadhaars::<T>::insert(
				aadhaar_id,
				Aadhaar {
					aadhaar_id: aadhaar_id,
					account_id: account_id.clone(),
				},
			);

			Lookup::<T>::insert(
				aadhaar_id,
				account_id,
			);

			RLookup::<T>::insert(
				account_id,
				aadhaar_id,
			);

			Ok(())
		}

	}
}