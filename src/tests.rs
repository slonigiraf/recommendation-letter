// Was generated with https://github.com/slonigiraf/recommendation-letter-testing
use super::*;

use crate as letters;
use frame_support::{assert_noop, assert_ok, parameter_types};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
        LettersModule: letters::{Pallet, Call, Storage, Event<T>, Config<Test>},
    }
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u64;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type MaxHolds = ();
}

parameter_types! {
	pub static MockRandom: H256 = Default::default();
}

parameter_types! {
	pub const MaxClassMetadata: u32 = 0;
	pub const MaxTokenMetadata: u32 = 0;
}

parameter_types! {
	pub const DefaultDifficulty: u32 = 3;
	pub const LettersPerChunk: u32 = 1000;
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type WeightInfo = ();
	type DefaultDifficulty = DefaultDifficulty;
	type LettersPerChunk = LettersPerChunk;
}


pub const REFEREE_ID: [u8; 32] = [212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125];
pub const WORKER_ID: [u8; 32] = [142,175,4,21,22,135,115,99,38,201,254,161,126,37,252,82,135,97,54,147,201,18,144,156,178,38,170,71,148,242,106,72];
pub const EMPLOYER_ID: [u8; 32] = [254,101,113,125,173,4,71,215,21,246,96,160,165,132,17,222,80,155,66,230,239,184,55,95,86,47,88,165,84,213,134,14];
pub const MALICIOUS_ID: [u8; 32] = [6,166,233,218,61,113,34,154,171,136,61,55,234,9,139,102,146,228,207,22,127,51,101,144,61,162,221,109,87,172,54,120];
pub const INITIAL_BALANCE: u64 = 1000;
pub const REFEREE_STAKE: u64 = 10;
pub const LETTER_ID: u32 = 1;
pub const LAST_VALID_BLOCK_NUMBER: u64 = 100;
pub const LAST_BLOCK_ALLOWED: u64 = 50;
pub const AFTER_VALID_BLOCK_NUMBER: u64 = 101;
pub const AFTER_LAST_BLOCK_ALLOWED: u64 = 51;


// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(
				AccountId::from(Public::from_raw(REFEREE_ID)).into_account(),
				INITIAL_BALANCE,
			),
			(
				AccountId::from(Public::from_raw(WORKER_ID)).into_account(),
				INITIAL_BALANCE,
			),
			(
				AccountId::from(Public::from_raw(EMPLOYER_ID)).into_account(),
				INITIAL_BALANCE,
			),
			(
				AccountId::from(Public::from_raw(MALICIOUS_ID)).into_account(),
				INITIAL_BALANCE,
			),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut t: sp_io::TestExternalities = t.into();

	t.execute_with(|| System::set_block_number(1));
	t
}

#[test]
fn coordinates_from_letter_index() {
	new_test_ext().execute_with(|| {
		let coordinates = LettersModule::coordinates_from_letter_index(0);
		assert_eq!(coordinates.chunk, 0);
		assert_eq!(coordinates.index, 0);
		//
		let coordinates = LettersModule::coordinates_from_letter_index(1);
		assert_eq!(coordinates.chunk, 0);
		assert_eq!(coordinates.index, 1);
		let coordinates = LettersModule::coordinates_from_letter_index(1001);
		assert_eq!(coordinates.chunk, 1);
		assert_eq!(coordinates.index, 1);
	});
}

#[test]
fn letter_index_from_coordinates() {
	new_test_ext().execute_with(|| {
		let number =
			LettersModule::letter_index_from_coordinates(LetterCoordinates { chunk: 0, index: 0 });
		assert_eq!(number, 0);
		//
		let number =
			LettersModule::letter_index_from_coordinates(LetterCoordinates { chunk: 0, index: 1 });
		assert_eq!(number, 1);

		let number =
			LettersModule::letter_index_from_coordinates(LetterCoordinates { chunk: 1, index: 1 });
		assert_eq!(number, 1001);
	});
}

#[test]
fn mint_chunk() {
	new_test_ext().execute_with(|| {
		let referee_hash = H256::from(REFEREE_ID);
		let chunk = 1;
		assert_ok!(LettersModule::mint_chunk(referee_hash.clone(), chunk));
		assert_noop!(
			LettersModule::mint_chunk(referee_hash.clone(), chunk),
			"Letter already contains_key"
		);

		assert_eq!(
			LettersModule::chunk_exists(referee_hash.clone(), chunk),
			true
		);
		assert_eq!(LettersModule::chunk_exists(referee_hash.clone(), 0), false);
		assert_eq!(LettersModule::chunk_exists(referee_hash.clone(), 2), false);
	});
}

#[test]
fn was_letter_canceled() {
	new_test_ext().execute_with(|| {
		let referee_hash = H256::from(REFEREE_ID);
		let number = 1;
		let coordinates = LettersModule::coordinates_from_letter_index(number);
		//Assert fresh letters are unused
		assert_ok!(LettersModule::mint_chunk(
			referee_hash.clone(),
			coordinates.chunk
		));
		assert_eq!(
			LettersModule::was_letter_canceled(referee_hash.clone(), number),
			false
		);
		//Use letters
		assert_ok!(LettersModule::mark_letter_as_fraud(
			referee_hash.clone(),
			number
		));
		assert_eq!(
			LettersModule::was_letter_canceled(referee_hash.clone(), number),
			true
		);
		//Assert letters in other chunks are unused
		assert_eq!(
			LettersModule::was_letter_canceled(referee_hash.clone(), 1001),
			false
		);
	});
}

#[test]
fn mark_letter_as_fraud() {
	new_test_ext().execute_with(|| {
		let referee_hash = H256::from(REFEREE_ID);
		let number = 1;
		assert_ok!(LettersModule::mark_letter_as_fraud(
			referee_hash.clone(),
			number
		));
		assert_eq!(
			LettersModule::was_letter_canceled(referee_hash.clone(), number),
			true
		);
	});
}


#[test]
fn signature_is_valid() {
    new_test_ext().execute_with(|| {
        let data_bytes: [u8; 124] = [69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,69,0,0,0,1,0,0,0,0,0,0,0,100,212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125,142,175,4,21,22,135,115,99,38,201,254,161,126,37,252,82,135,97,54,147,201,18,144,156,178,38,170,71,148,242,106,72,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,10];
        let signer_bytes: [u8; 32] = [212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125];
        let sign_bytes: [u8; 64] = [94,11,111,38,126,16,183,4,110,156,44,29,211,64,233,93,215,198,61,7,205,155,246,4,234,70,139,104,157,173,179,37,129,32,242,252,12,137,215,250,182,228,227,8,209,138,184,136,30,124,75,94,50,206,198,50,223,71,52,17,87,102,55,140];
        let mut data = Vec::new();
        data.extend_from_slice(&data_bytes);
        assert_eq!(
            LettersModule::signature_is_valid(
                H512::from(sign_bytes),
                data,
                H256::from(signer_bytes)
            ),
            true
        );
    });
}

#[test]
fn expired() {
    new_test_ext().execute_with(|| {
        let referee_signature: [u8; 64] = [94,11,111,38,126,16,183,4,110,156,44,29,211,64,233,93,215,198,61,7,205,155,246,4,234,70,139,104,157,173,179,37,129,32,242,252,12,137,215,250,182,228,227,8,209,138,184,136,30,124,75,94,50,206,198,50,223,71,52,17,87,102,55,140];
        let worker_signature: [u8; 64] = [84,205,41,9,130,222,42,244,90,151,83,62,215,209,24,226,173,147,42,83,79,41,197,162,183,125,84,227,67,214,236,91,124,104,189,7,154,101,94,53,63,255,72,14,189,5,20,216,228,92,5,33,13,90,49,81,131,79,39,157,232,172,199,135];
        frame_system::Pallet::<Test>::set_block_number(AFTER_VALID_BLOCK_NUMBER);
        
        assert_noop!(
            LettersModule::reimburse(
                RuntimeOrigin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
                LAST_BLOCK_ALLOWED,
                H256::from(REFEREE_ID),
                H256::from(WORKER_ID),
                H256::from(EMPLOYER_ID),
                REFEREE_STAKE,
                H512::from(referee_signature),
                H512::from(worker_signature)
            ),
            Error::<Test>::Expired
        );
    });
}

#[test]
fn wrong_genesis() {
    new_test_ext().execute_with(|| {
        let referee_signature: [u8; 64] = [130,30,230,22,25,98,11,235,43,166,38,128,181,18,252,90,206,41,50,109,159,122,12,165,87,140,115,247,161,85,130,91,30,88,18,19,67,225,35,254,0,191,55,31,117,9,242,246,36,107,154,60,162,84,249,135,228,207,72,85,198,101,223,135];
        let worker_signature: [u8; 64] = [192,49,124,129,210,165,86,165,215,114,234,250,194,198,74,198,127,221,32,161,93,166,136,92,157,79,44,254,176,253,125,89,148,206,197,4,50,195,84,185,10,174,74,8,82,167,91,137,110,36,163,229,177,31,181,233,158,22,160,167,29,156,144,133];
        frame_system::Pallet::<Test>::set_block_number(LAST_BLOCK_ALLOWED);
        
        assert_noop!(
            LettersModule::reimburse(
                RuntimeOrigin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
                LAST_BLOCK_ALLOWED,
                H256::from(REFEREE_ID),
                H256::from(WORKER_ID),
                H256::from(EMPLOYER_ID),
                REFEREE_STAKE,
                H512::from(referee_signature),
                H512::from(worker_signature)
            ),
            Error::<Test>::InvalidRefereeSign
        );
    });
}

#[test]
fn successful_reimburse() {
    new_test_ext().execute_with(|| {
        let referee_hash = H256::from(REFEREE_ID);

        let referee_signature: [u8; 64] = [94,11,111,38,126,16,183,4,110,156,44,29,211,64,233,93,215,198,61,7,205,155,246,4,234,70,139,104,157,173,179,37,129,32,242,252,12,137,215,250,182,228,227,8,209,138,184,136,30,124,75,94,50,206,198,50,223,71,52,17,87,102,55,140];
        let worker_signature: [u8; 64] = [84,205,41,9,130,222,42,244,90,151,83,62,215,209,24,226,173,147,42,83,79,41,197,162,183,125,84,227,67,214,236,91,124,104,189,7,154,101,94,53,63,255,72,14,189,5,20,216,228,92,5,33,13,90,49,81,131,79,39,157,232,172,199,135];
        frame_system::Pallet::<Test>::set_block_number(LAST_BLOCK_ALLOWED);
        
        assert_eq!(
            LettersModule::was_letter_canceled(referee_hash.clone(), LETTER_ID as usize),
            false
        );

        assert_ok!(LettersModule::reimburse(
            RuntimeOrigin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
            LETTER_ID,
            LAST_VALID_BLOCK_NUMBER,
            LAST_BLOCK_ALLOWED,
            H256::from(REFEREE_ID),
            H256::from(WORKER_ID),
            H256::from(EMPLOYER_ID),
            REFEREE_STAKE,
            H512::from(referee_signature),
            H512::from(worker_signature)
        ));

        assert_eq!(
            LettersModule::was_letter_canceled(referee_hash.clone(), LETTER_ID as usize),
            true
        );

        assert_noop!(
            LettersModule::reimburse(
                RuntimeOrigin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
                LAST_BLOCK_ALLOWED,
                H256::from(REFEREE_ID),
                H256::from(WORKER_ID),
                H256::from(EMPLOYER_ID),
                REFEREE_STAKE,
                H512::from(referee_signature),
                H512::from(worker_signature)
            ),
            Error::<Test>::LetterWasMarkedAsFraudBefore
        );
    });
}

#[test]
fn wrong_referee_sign() {
    new_test_ext().execute_with(|| {
        let referee_signature: [u8; 64] = [158,83,120,135,25,62,57,82,144,36,129,95,183,194,39,167,165,172,5,38,54,76,54,25,8,142,251,113,82,188,28,22,171,204,22,117,240,3,22,212,90,93,219,134,9,141,76,175,192,87,136,230,73,109,128,230,155,76,140,189,231,54,140,133];
        let worker_signature: [u8; 64] = [84,205,41,9,130,222,42,244,90,151,83,62,215,209,24,226,173,147,42,83,79,41,197,162,183,125,84,227,67,214,236,91,124,104,189,7,154,101,94,53,63,255,72,14,189,5,20,216,228,92,5,33,13,90,49,81,131,79,39,157,232,172,199,135];
        frame_system::Pallet::<Test>::set_block_number(LAST_BLOCK_ALLOWED);

        assert_noop!(
            LettersModule::reimburse(
                RuntimeOrigin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
                LAST_BLOCK_ALLOWED,
                H256::from(REFEREE_ID),
                H256::from(WORKER_ID),
                H256::from(EMPLOYER_ID),
                REFEREE_STAKE,
                H512::from(referee_signature),
                H512::from(worker_signature)
            ),
            Error::<Test>::InvalidRefereeSign
        );
    });
}

#[test]
fn referee_has_not_enough_balance() {
    new_test_ext().execute_with(|| {
        let referee_signature: [u8; 64] = [94,11,111,38,126,16,183,4,110,156,44,29,211,64,233,93,215,198,61,7,205,155,246,4,234,70,139,104,157,173,179,37,129,32,242,252,12,137,215,250,182,228,227,8,209,138,184,136,30,124,75,94,50,206,198,50,223,71,52,17,87,102,55,140];
        let worker_signature: [u8; 64] = [84,205,41,9,130,222,42,244,90,151,83,62,215,209,24,226,173,147,42,83,79,41,197,162,183,125,84,227,67,214,236,91,124,104,189,7,154,101,94,53,63,255,72,14,189,5,20,216,228,92,5,33,13,90,49,81,131,79,39,157,232,172,199,135];
        frame_system::Pallet::<Test>::set_block_number(LAST_BLOCK_ALLOWED);

        Balances::make_free_balance_be(
            &AccountId::from(Public::from_raw(REFEREE_ID)).into_account(),
            9,
        );

        assert_noop!(
            LettersModule::reimburse(
                RuntimeOrigin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
                LAST_BLOCK_ALLOWED,
                H256::from(REFEREE_ID),
                H256::from(WORKER_ID),
                H256::from(EMPLOYER_ID),
                REFEREE_STAKE,
                H512::from(referee_signature),
                H512::from(worker_signature)
            ),
            Error::<Test>::RefereeBalanceIsNotEnough
        );
    });
}

#[test]
fn wrong_worker_sign() {
    new_test_ext().execute_with(|| {
        let referee_signature: [u8; 64] = [94,11,111,38,126,16,183,4,110,156,44,29,211,64,233,93,215,198,61,7,205,155,246,4,234,70,139,104,157,173,179,37,129,32,242,252,12,137,215,250,182,228,227,8,209,138,184,136,30,124,75,94,50,206,198,50,223,71,52,17,87,102,55,140];
        let worker_signature: [u8; 64] = [70,176,161,23,138,181,173,155,99,251,163,18,79,23,114,14,144,7,20,124,3,193,11,85,134,68,235,251,200,16,12,44,205,129,103,7,79,207,166,76,4,176,67,176,83,172,96,218,147,255,177,133,168,183,167,95,27,200,243,166,72,227,19,130];
        frame_system::Pallet::<Test>::set_block_number(LAST_BLOCK_ALLOWED);

        assert_noop!(
            LettersModule::reimburse(
                RuntimeOrigin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
                LAST_BLOCK_ALLOWED,
                H256::from(REFEREE_ID),
                H256::from(WORKER_ID),
                H256::from(EMPLOYER_ID),
                REFEREE_STAKE,
                H512::from(referee_signature),
                H512::from(worker_signature)
            ),
            Error::<Test>::InvalidWorkerSign
        );
    });
}