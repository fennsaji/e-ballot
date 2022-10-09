use crate::mock::*;
use crate::mock::Ballot;
use super::*;

use frame_support::{ assert_ok, assert_noop };


#[test]
fn test_start_voting() {
	new_test_ext().execute_with(|| {
        let vote_index = 0;

        assert_ok!(Ballot::start_voting(
			Origin::signed(INITIAL_USER_ACCOUNT),
		));

		assert_eq!(VotingState::<Test>::get(vote_index), VoteState::Voting);
		assert_eq!(ChiefCommissioner::<Test>::get(vote_index), Some(INITIAL_USER_AADHAAR));
		assert_eq!(CurrentVoteIndex::<Test>::get(), 1);
	})
}

#[test]
fn test_add_candidates() {
	new_test_ext().execute_with(|| {
        let vote_index = 0;

        assert_ok!(Ballot::start_voting(
			Origin::signed(INITIAL_USER_ACCOUNT),
		));


        let candidates = vec![INITIAL_USER_TWO_ACCOUNT, INITIAL_USER_THREE_ACCOUNT];

        assert_ok!(Ballot::add_candidates(
			Origin::signed(INITIAL_USER_ACCOUNT),
            vote_index,
            candidates,
		));

        assert_eq!(Candidates::<Test>::contains_key(vote_index, INITIAL_USER_TWO_AADHAAR), true);
        assert_eq!(Candidates::<Test>::contains_key(vote_index, INITIAL_USER_THREE_AADHAAR), true);
	})
}

#[test]
fn test_vote() {
	new_test_ext().execute_with(|| {
        let vote_index = 0;

        assert_ok!(Ballot::start_voting(
			Origin::signed(INITIAL_USER_ACCOUNT),
		));


        let candidates = vec![INITIAL_USER_TWO_ACCOUNT, INITIAL_USER_THREE_ACCOUNT];

        assert_ok!(Ballot::add_candidates(
			Origin::signed(INITIAL_USER_ACCOUNT),
            vote_index,
            candidates,
		));

        assert_ok!(Ballot::vote(
			Origin::signed(INITIAL_USER_ACCOUNT),
            vote_index,
            INITIAL_USER_TWO_AADHAAR,
		));

        assert_eq!(Candidates::<Test>::get(vote_index, INITIAL_USER_TWO_AADHAAR).vote_count, 1);
        assert_eq!(Votes::<Test>::get(vote_index, INITIAL_USER_AADHAAR), true);
	})
}

#[test]
fn test_vote_fails_on_candidate() {
	new_test_ext().execute_with(|| {
        let vote_index = 0;
        
        assert_noop!(Ballot::vote(
                Origin::signed(INITIAL_USER_ACCOUNT),
                vote_index,
                INITIAL_USER_TWO_AADHAAR,
            ), Error::<Test>::VoteSessionNotFound,
        );
	})
}

#[test]
fn test_vote_fails_on_duplicate_vote() {
	new_test_ext().execute_with(|| {
        let vote_index = 0;

        assert_ok!(Ballot::start_voting(
			Origin::signed(INITIAL_USER_ACCOUNT),
		));

        let candidates = vec![INITIAL_USER_TWO_ACCOUNT, INITIAL_USER_THREE_ACCOUNT];

        assert_ok!(Ballot::add_candidates(
			Origin::signed(INITIAL_USER_ACCOUNT),
            vote_index,
            candidates,
		));

        assert_ok!(Ballot::vote(
			Origin::signed(INITIAL_USER_ACCOUNT),
            vote_index,
            INITIAL_USER_TWO_AADHAAR,
		));

        assert_noop!(Ballot::vote(
                Origin::signed(INITIAL_USER_ACCOUNT),
                vote_index,
                INITIAL_USER_TWO_AADHAAR,
		    ), Error::<Test>::VoteAlreadyCast,
        );
	})
}

#[test]
fn test_stop_voting() {
	new_test_ext().execute_with(|| {

        assert_ok!(Ballot::start_voting(
			Origin::signed(INITIAL_USER_ACCOUNT),
		));

        let vote_index = 0;

        assert_ok!(Ballot::stop_voting(
			Origin::signed(INITIAL_USER_ACCOUNT),
            vote_index,
		));

		assert_eq!(VotingState::<Test>::get(vote_index), VoteState::Ended);
	})
}

#[test]
fn test_reset_voting() {
	new_test_ext().execute_with(|| {

        assert_ok!(Ballot::start_voting(
			Origin::signed(INITIAL_USER_ACCOUNT),
		));

        let vote_index = 0;

        assert_ok!(Ballot::reset_voting(
			Origin::signed(INITIAL_USER_ACCOUNT),
            vote_index,
		));

		assert_eq!(VotingState::<Test>::get(vote_index), VoteState::Idle);
		assert_eq!(ChiefCommissioner::<Test>::get(vote_index), None);
		assert_eq!(!Candidates::<Test>::contains_key(vote_index, INITIAL_USER_TWO_AADHAAR), true);
		assert_eq!(!Votes::<Test>::contains_key(vote_index, INITIAL_USER_AADHAAR), true);
	})
}
