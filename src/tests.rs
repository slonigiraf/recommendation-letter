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
pub const BEFORE_VALID_BLOCK_NUMBER: u64 = 99;
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
        let sign_bytes: [u8; 64] = [192,168,180,59,190,113,70,166,190,179,57,76,77,117,102,239,61,19,115,5,206,204,106,78,195,76,187,17,111,149,132,125,194,79,19,81,157,96,17,185,11,18,174,193,130,130,124,192,26,64,79,233,29,51,167,49,232,157,232,88,70,245,187,133];
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

        let referee_signature: [u8; 64] = [192,168,180,59,190,113,70,166,190,179,57,76,77,117,102,239,61,19,115,5,206,204,106,78,195,76,187,17,111,149,132,125,194,79,19,81,157,96,17,185,11,18,174,193,130,130,124,192,26,64,79,233,29,51,167,49,232,157,232,88,70,245,187,133];
        let worker_signature: [u8; 64] = [96,34,94,60,39,205,167,96,214,38,19,133,118,122,208,8,251,75,140,183,135,32,36,149,178,108,236,142,43,104,82,88,112,254,106,45,212,164,87,93,76,77,8,59,37,35,184,18,125,214,237,166,248,170,190,159,27,38,126,224,240,208,134,138];
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
        let referee_hash = H256::from(REFEREE_ID);

        let referee_signature: [u8; 64] = [2,7,43,143,107,54,50,158,19,197,40,144,99,78,63,238,79,9,216,233,139,243,132,181,240,80,21,188,172,72,207,41,130,141,156,45,41,65,40,114,202,198,136,27,10,180,82,227,144,62,31,227,92,109,20,43,37,215,15,74,237,174,128,134];
        let worker_signature: [u8; 64] = [4,178,220,184,67,145,7,155,94,167,140,30,229,11,252,225,71,215,8,100,160,134,253,154,236,186,149,76,163,159,230,39,2,178,163,133,193,120,86,231,22,151,78,89,253,10,110,177,29,153,252,2,66,99,144,241,164,1,124,243,229,70,13,141];
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

        let referee_signature: [u8; 64] = [192,168,180,59,190,113,70,166,190,179,57,76,77,117,102,239,61,19,115,5,206,204,106,78,195,76,187,17,111,149,132,125,194,79,19,81,157,96,17,185,11,18,174,193,130,130,124,192,26,64,79,233,29,51,167,49,232,157,232,88,70,245,187,133];
        let worker_signature: [u8; 64] = [96,34,94,60,39,205,167,96,214,38,19,133,118,122,208,8,251,75,140,183,135,32,36,149,178,108,236,142,43,104,82,88,112,254,106,45,212,164,87,93,76,77,8,59,37,35,184,18,125,214,237,166,248,170,190,159,27,38,126,224,240,208,134,138];
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
        let referee_hash = H256::from(REFEREE_ID);

        let referee_signature: [u8; 64] = [214,173,116,56,77,76,150,87,104,229,184,21,46,176,184,78,132,24,229,90,200,202,25,111,211,234,194,194,86,42,208,93,2,198,79,70,147,198,50,177,108,163,233,130,17,205,35,125,140,152,155,96,114,114,19,130,117,169,104,99,170,175,213,134];
        let worker_signature: [u8; 64] = [96,34,94,60,39,205,167,96,214,38,19,133,118,122,208,8,251,75,140,183,135,32,36,149,178,108,236,142,43,104,82,88,112,254,106,45,212,164,87,93,76,77,8,59,37,35,184,18,125,214,237,166,248,170,190,159,27,38,126,224,240,208,134,138];
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
        let referee_hash = H256::from(REFEREE_ID);

        let referee_signature: [u8; 64] = [192,168,180,59,190,113,70,166,190,179,57,76,77,117,102,239,61,19,115,5,206,204,106,78,195,76,187,17,111,149,132,125,194,79,19,81,157,96,17,185,11,18,174,193,130,130,124,192,26,64,79,233,29,51,167,49,232,157,232,88,70,245,187,133];
        let worker_signature: [u8; 64] = [96,34,94,60,39,205,167,96,214,38,19,133,118,122,208,8,251,75,140,183,135,32,36,149,178,108,236,142,43,104,82,88,112,254,106,45,212,164,87,93,76,77,8,59,37,35,184,18,125,214,237,166,248,170,190,159,27,38,126,224,240,208,134,138];
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
        let referee_hash = H256::from(REFEREE_ID);

        let referee_signature: [u8; 64] = [192,168,180,59,190,113,70,166,190,179,57,76,77,117,102,239,61,19,115,5,206,204,106,78,195,76,187,17,111,149,132,125,194,79,19,81,157,96,17,185,11,18,174,193,130,130,124,192,26,64,79,233,29,51,167,49,232,157,232,88,70,245,187,133];
        let worker_signature: [u8; 64] = [136,208,175,50,176,162,127,86,167,88,145,96,42,52,167,211,171,210,212,233,164,119,155,95,199,245,48,227,189,129,63,60,185,234,84,48,227,214,0,142,149,58,163,159,235,68,195,190,76,245,108,96,155,31,58,45,211,237,203,152,39,75,193,139];
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