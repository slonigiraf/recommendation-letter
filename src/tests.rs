// Was generated with https://github.com/slonigiraf/recommendation-letter-testing
use super::*;

use crate as letters;
use frame_support::{assert_noop, assert_ok, parameter_types};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		LettersModule: letters::{Pallet, Call, Storage, Event<T>, Config},
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
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
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
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = ();
}

parameter_types! {
	pub static MockRandom: H256 = Default::default();
}

impl Randomness<H256, u64> for MockRandom {
	fn random(_subject: &[u8]) -> (H256, u64) {
		(MockRandom::get(), 0)
	}
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
	type Event = Event;
	type Randomness = MockRandom;
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
pub const AFTER_VALID_BLOCK_NUMBER: u64 = 101;


// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

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

	<crate::GenesisConfig as GenesisBuild<Test>>::assimilate_storage(
		&crate::GenesisConfig::default(),
		&mut t,
	)
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
        let sign_bytes: [u8; 64] = [230,254,200,182,235,226,12,249,184,181,112,226,175,39,242,54,250,54,30,153,11,6,223,63,200,15,30,172,155,236,184,90,228,93,35,32,112,131,38,174,64,123,134,191,21,104,192,26,39,254,160,62,107,122,113,224,15,139,105,188,15,194,0,136];
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
        let referee_hash = H256::from(REFEREE_ID);

        let referee_signature: [u8; 64] = [230,254,200,182,235,226,12,249,184,181,112,226,175,39,242,54,250,54,30,153,11,6,223,63,200,15,30,172,155,236,184,90,228,93,35,32,112,131,38,174,64,123,134,191,21,104,192,26,39,254,160,62,107,122,113,224,15,139,105,188,15,194,0,136];
        let worker_signature: [u8; 64] = [226,142,94,47,218,226,248,250,176,143,168,97,3,93,18,23,44,244,103,153,34,158,101,73,154,49,215,64,235,69,164,28,9,106,65,132,118,73,83,1,185,164,144,144,1,242,217,189,151,90,160,232,18,133,106,125,213,237,117,24,244,60,194,131];
        frame_system::Pallet::<Test>::set_block_number(AFTER_VALID_BLOCK_NUMBER);
        
        assert_noop!(
            LettersModule::reimburse(
                Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
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
        let referee_signature: [u8; 64] = [82,254,101,221,254,83,229,40,65,235,118,8,3,173,243,4,146,137,233,186,246,194,162,23,151,19,132,21,101,100,164,0,145,155,37,47,44,151,131,53,121,130,203,62,83,39,3,205,35,190,111,92,208,155,5,71,37,10,237,188,228,189,159,137];
        let worker_signature: [u8; 64] = [168,100,52,20,145,240,89,153,236,224,32,129,136,3,48,249,189,119,114,201,98,124,177,97,124,127,114,54,194,136,253,78,254,160,34,61,84,241,5,45,135,11,175,156,105,123,65,250,125,174,13,15,217,7,156,227,13,160,32,59,145,56,51,135];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);
        
        assert_noop!(
            LettersModule::reimburse(
                Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
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

        let referee_signature: [u8; 64] = [230,254,200,182,235,226,12,249,184,181,112,226,175,39,242,54,250,54,30,153,11,6,223,63,200,15,30,172,155,236,184,90,228,93,35,32,112,131,38,174,64,123,134,191,21,104,192,26,39,254,160,62,107,122,113,224,15,139,105,188,15,194,0,136];
        let worker_signature: [u8; 64] = [226,142,94,47,218,226,248,250,176,143,168,97,3,93,18,23,44,244,103,153,34,158,101,73,154,49,215,64,235,69,164,28,9,106,65,132,118,73,83,1,185,164,144,144,1,242,217,189,151,90,160,232,18,133,106,125,213,237,117,24,244,60,194,131];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);
        
        assert_eq!(
            LettersModule::was_letter_canceled(referee_hash.clone(), LETTER_ID as usize),
            false
        );

        assert_ok!(LettersModule::reimburse(
            Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
            LETTER_ID,
            LAST_VALID_BLOCK_NUMBER,
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
                Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
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
        let referee_signature: [u8; 64] = [244,198,108,203,90,152,218,204,245,58,155,207,30,12,172,242,184,156,50,205,217,18,31,90,55,11,166,26,253,40,10,43,31,146,247,183,63,80,177,88,217,35,47,252,66,39,6,18,154,113,212,66,81,188,179,199,138,216,103,252,100,46,104,131];
        let worker_signature: [u8; 64] = [226,142,94,47,218,226,248,250,176,143,168,97,3,93,18,23,44,244,103,153,34,158,101,73,154,49,215,64,235,69,164,28,9,106,65,132,118,73,83,1,185,164,144,144,1,242,217,189,151,90,160,232,18,133,106,125,213,237,117,24,244,60,194,131];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);

        assert_noop!(
            LettersModule::reimburse(
                Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
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
        let referee_signature: [u8; 64] = [230,254,200,182,235,226,12,249,184,181,112,226,175,39,242,54,250,54,30,153,11,6,223,63,200,15,30,172,155,236,184,90,228,93,35,32,112,131,38,174,64,123,134,191,21,104,192,26,39,254,160,62,107,122,113,224,15,139,105,188,15,194,0,136];
        let worker_signature: [u8; 64] = [226,142,94,47,218,226,248,250,176,143,168,97,3,93,18,23,44,244,103,153,34,158,101,73,154,49,215,64,235,69,164,28,9,106,65,132,118,73,83,1,185,164,144,144,1,242,217,189,151,90,160,232,18,133,106,125,213,237,117,24,244,60,194,131];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);

        Balances::make_free_balance_be(
            &AccountId::from(Public::from_raw(REFEREE_ID)).into_account(),
            9,
        );

        assert_noop!(
            LettersModule::reimburse(
                Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
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
        let referee_signature: [u8; 64] = [230,254,200,182,235,226,12,249,184,181,112,226,175,39,242,54,250,54,30,153,11,6,223,63,200,15,30,172,155,236,184,90,228,93,35,32,112,131,38,174,64,123,134,191,21,104,192,26,39,254,160,62,107,122,113,224,15,139,105,188,15,194,0,136];
        let worker_signature: [u8; 64] = [36,13,35,67,79,172,158,188,93,157,29,134,84,123,109,0,97,71,183,250,239,55,31,13,215,9,174,246,170,151,174,80,136,198,134,217,65,2,159,218,129,232,199,46,213,135,131,121,183,3,151,96,218,128,175,131,50,53,153,78,192,60,132,138];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);

        assert_noop!(
            LettersModule::reimburse(
                Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
                LETTER_ID,
                LAST_VALID_BLOCK_NUMBER,
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