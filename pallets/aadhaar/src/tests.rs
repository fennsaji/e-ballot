use crate::mock::*;
use crate::types::*;
use crate::mock::Aadhaar;
use super::*;

use frame_support::{ assert_ok, assert_noop };

//START GENESIS TESTING
#[test]
fn test_genesis_worked() {
	new_test_ext().execute_with(|| {
		assert_eq!(Aadhaars::<Test>::contains_key(INITIAL_USER_AADHAAR), true);
		assert_eq!(Lookup::<Test>::contains_key(INITIAL_USER_AADHAAR), true);
		assert_eq!(
			RLookup::<Test>::contains_key(INITIAL_USER_ACCOUNT),
			true
		);
	})
}
//END GENESIS TESTING

#[test]
fn test_register_aadhaar() {
	new_test_ext().execute_with(|| {
        let new_account_id: AccountId = 2;
        let new_aadhaar_id: AadhaarId = *b"2222222222222222";

        assert_ok!(Aadhaar::register_aadhaar(
			Origin::signed(INITIAL_USER_ACCOUNT),
            new_account_id,
            new_aadhaar_id
		));

		assert_eq!(Aadhaars::<Test>::contains_key(new_aadhaar_id), true);
		assert_eq!(Lookup::<Test>::contains_key(new_aadhaar_id), true);
		assert_eq!(RLookup::<Test>::contains_key(new_account_id), true);
	})
}

#[test]
fn test_register_existing_aadhaar_fails() {
    new_test_ext().execute_with(|| {
        let user_account_id: AccountId = 2;
        let user_aadhaar_id = *b"2222222222222222";

        assert_ok!(Aadhaar::register_aadhaar(
			Origin::signed(INITIAL_USER_ACCOUNT),
            user_account_id,
            user_aadhaar_id
		));

		assert_eq!(Aadhaars::<Test>::contains_key(user_aadhaar_id), true);
		assert_eq!(Lookup::<Test>::contains_key(user_aadhaar_id), true);
		assert_eq!(RLookup::<Test>::contains_key(user_account_id), true);

        let new_user_account_id = 3;

        assert_noop!(Aadhaar::register_aadhaar(
			    Origin::signed(INITIAL_USER_ACCOUNT),
                new_user_account_id,
                user_aadhaar_id,
		    ), 
            Error::<Test>::AadhaarAlreadyExists,
        );
	})
}

#[test]
fn test_register_existing_account_id_fails() {
    new_test_ext().execute_with(|| {
        let user_account_id: AccountId = 2;
        let user_aadhaar_id = *b"2222222222222222";

        assert_ok!(Aadhaar::register_aadhaar(
			Origin::signed(INITIAL_USER_ACCOUNT),
            user_account_id,
            user_aadhaar_id
		));

		assert_eq!(Aadhaars::<Test>::contains_key(user_aadhaar_id), true);
		assert_eq!(Lookup::<Test>::contains_key(user_aadhaar_id), true);
		assert_eq!(RLookup::<Test>::contains_key(user_account_id), true);

        let new_user_aadhaar_id: AadhaarId = *b"3333333333333333";

        assert_noop!(Aadhaar::register_aadhaar(
			    Origin::signed(INITIAL_USER_ACCOUNT),
                user_account_id,
                new_user_aadhaar_id,
		    ), 
            Error::<Test>::AccountIdRegistered,
        );
	})
}
