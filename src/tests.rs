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
        let sign_bytes: [u8; 64] = [236,248,246,90,12,255,65,1,143,28,160,59,127,89,42,123,199,209,173,83,87,211,95,100,224,58,129,138,82,121,52,91,1,63,4,172,237,124,208,224,32,131,230,225,253,225,42,11,239,38,96,100,58,220,245,227,102,144,160,121,73,141,179,137];
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

        let referee_signature: [u8; 64] = [236,248,246,90,12,255,65,1,143,28,160,59,127,89,42,123,199,209,173,83,87,211,95,100,224,58,129,138,82,121,52,91,1,63,4,172,237,124,208,224,32,131,230,225,253,225,42,11,239,38,96,100,58,220,245,227,102,144,160,121,73,141,179,137];
        let worker_signature: [u8; 64] = [168,145,29,168,193,87,29,77,98,33,165,76,22,42,22,12,184,169,29,127,230,78,95,65,244,81,195,175,114,125,96,1,26,95,130,62,40,133,87,21,171,49,1,247,211,192,122,36,1,197,75,21,20,12,118,114,65,56,146,67,62,98,221,131];
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
        let referee_signature: [u8; 64] = [76,198,19,92,57,98,172,137,16,108,184,198,192,130,30,170,109,174,213,243,43,224,184,204,184,106,152,37,141,134,190,39,111,154,223,103,56,68,67,243,69,102,114,85,181,211,120,228,113,75,81,209,15,108,196,228,94,38,18,247,86,140,144,130];
        let worker_signature: [u8; 64] = [216,114,164,42,223,225,240,190,16,117,7,239,106,53,43,102,163,197,60,215,41,33,53,110,140,221,215,248,176,236,191,112,155,250,64,4,111,39,115,211,193,104,90,115,228,186,148,114,141,53,251,152,38,124,137,223,50,117,167,38,71,38,183,138];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);
        
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

        let referee_signature: [u8; 64] = [236,248,246,90,12,255,65,1,143,28,160,59,127,89,42,123,199,209,173,83,87,211,95,100,224,58,129,138,82,121,52,91,1,63,4,172,237,124,208,224,32,131,230,225,253,225,42,11,239,38,96,100,58,220,245,227,102,144,160,121,73,141,179,137];
        let worker_signature: [u8; 64] = [168,145,29,168,193,87,29,77,98,33,165,76,22,42,22,12,184,169,29,127,230,78,95,65,244,81,195,175,114,125,96,1,26,95,130,62,40,133,87,21,171,49,1,247,211,192,122,36,1,197,75,21,20,12,118,114,65,56,146,67,62,98,221,131];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);
        
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
        let referee_signature: [u8; 64] = [34,185,176,103,161,135,36,197,164,4,81,200,65,156,249,216,45,68,119,236,225,165,92,231,54,182,104,74,75,39,106,125,21,101,8,31,109,127,253,95,33,103,225,78,105,242,214,221,89,201,159,228,60,130,199,132,47,251,16,41,167,214,125,131];
        let worker_signature: [u8; 64] = [168,145,29,168,193,87,29,77,98,33,165,76,22,42,22,12,184,169,29,127,230,78,95,65,244,81,195,175,114,125,96,1,26,95,130,62,40,133,87,21,171,49,1,247,211,192,122,36,1,197,75,21,20,12,118,114,65,56,146,67,62,98,221,131];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);

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
        let referee_signature: [u8; 64] = [236,248,246,90,12,255,65,1,143,28,160,59,127,89,42,123,199,209,173,83,87,211,95,100,224,58,129,138,82,121,52,91,1,63,4,172,237,124,208,224,32,131,230,225,253,225,42,11,239,38,96,100,58,220,245,227,102,144,160,121,73,141,179,137];
        let worker_signature: [u8; 64] = [168,145,29,168,193,87,29,77,98,33,165,76,22,42,22,12,184,169,29,127,230,78,95,65,244,81,195,175,114,125,96,1,26,95,130,62,40,133,87,21,171,49,1,247,211,192,122,36,1,197,75,21,20,12,118,114,65,56,146,67,62,98,221,131];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);

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
        let referee_signature: [u8; 64] = [236,248,246,90,12,255,65,1,143,28,160,59,127,89,42,123,199,209,173,83,87,211,95,100,224,58,129,138,82,121,52,91,1,63,4,172,237,124,208,224,32,131,230,225,253,225,42,11,239,38,96,100,58,220,245,227,102,144,160,121,73,141,179,137];
        let worker_signature: [u8; 64] = [126,253,185,82,150,252,167,105,202,69,194,199,10,227,122,23,148,175,109,164,168,103,117,80,20,45,235,74,219,173,49,8,16,226,98,165,0,175,148,238,10,125,243,19,97,135,29,167,239,34,16,57,203,57,246,60,218,222,246,30,213,198,134,132];
        frame_system::Pallet::<Test>::set_block_number(LAST_VALID_BLOCK_NUMBER);

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