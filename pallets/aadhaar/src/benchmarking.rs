#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use crate::types::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::{EventRecord, Pallet as System};
use frame_support::traits::{EnsureOrigin, UnfilteredDispatchable};


fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = System::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks! {
	// This will measure the execution time of `register_aadhaar`.
	register_aadhaar {
        
        let new_account_id: T::AccountId = whitelisted_caller();
        let origin = T::RegisterOrigin::successful_origin();
        let new_aadhaar_id: AadhaarId = *b"2222222222222222";

        let call = Call::<T>::register_aadhaar { aadhaar_id: new_aadhaar_id, account_id: new_account_id.clone() };

	}: { call.dispatch_bypass_filter(origin)? }
	verify {

		assert_eq!(Pallet::<T>::aadhaar(new_aadhaar_id), Some(Aadhaar{
            aadhaar_id: new_aadhaar_id,
            account_id: new_account_id.clone(),
        }));

		assert_last_event::<T>(Event::AadhaarRegistered { aadhaar_id: new_aadhaar_id, account_id: new_account_id }.into());
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test)
}
