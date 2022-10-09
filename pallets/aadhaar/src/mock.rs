use crate as pallet_aadhaar;

use crate::types::*;
use frame_support::{
	traits::{ GenesisBuild, ConstU16, ConstU32, ConstU64 },
};

use frame_system as system;
use sp_core::{ H256 };
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use system::EnsureSigned;

pub const INITIAL_USER_ACCOUNT: u64 = 1;
pub const INITIAL_USER_AADHAAR: AadhaarId = *b"1111111111111111";

pub type AccounId = u64;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Aadhaar: pallet_aadhaar::{Pallet, Call, Storage, Event<T>, Config<T>},
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccounId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_aadhaar::Config for Test {
	type Event = Event;
	type RegisterOrigin = EnsureSigned<Self::AccountId>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut o = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();
	
	super::GenesisConfig::<Test> {
		initial_aadhaars: vec![(
            INITIAL_USER_AADHAAR,
            INITIAL_USER_ACCOUNT
        )],
		
		phantom: Default::default(),
	}
		.assimilate_storage(&mut o)
		.unwrap();
	o.into()
}
