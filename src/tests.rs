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
        let sign_bytes: [u8; 64] = [186,113,112,231,103,180,169,160,159,206,125,143,33,140,75,171,112,254,216,62,133,202,238,29,191,155,147,2,87,224,198,94,124,52,87,54,172,55,181,167,195,185,160,29,248,99,116,131,74,198,91,154,249,14,37,246,86,74,56,136,230,60,58,142];
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
fn not_allowed_block() {
    new_test_ext().execute_with(|| {
        let referee_signature: [u8; 64] = [186,113,112,231,103,180,169,160,159,206,125,143,33,140,75,171,112,254,216,62,133,202,238,29,191,155,147,2,87,224,198,94,124,52,87,54,172,55,181,167,195,185,160,29,248,99,116,131,74,198,91,154,249,14,37,246,86,74,56,136,230,60,58,142];
        let worker_signature: [u8; 64] = [230,29,27,157,96,227,37,25,221,190,226,210,72,103,25,228,36,72,3,6,200,227,125,136,163,104,35,77,36,77,128,25,186,164,184,106,142,90,205,226,225,183,188,238,216,39,170,39,128,147,164,32,132,128,38,87,232,174,239,55,24,188,143,136];
        frame_system::Pallet::<Test>::set_block_number(AFTER_LAST_BLOCK_ALLOWED);
        
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
            Error::<Test>::NotAllowedBlock
        );
    });
}

#[test]
fn expired() {
    new_test_ext().execute_with(|| {
        let referee_signature: [u8; 64] = [186,113,112,231,103,180,169,160,159,206,125,143,33,140,75,171,112,254,216,62,133,202,238,29,191,155,147,2,87,224,198,94,124,52,87,54,172,55,181,167,195,185,160,29,248,99,116,131,74,198,91,154,249,14,37,246,86,74,56,136,230,60,58,142];
        let worker_signature: [u8; 64] = [230,29,27,157,96,227,37,25,221,190,226,210,72,103,25,228,36,72,3,6,200,227,125,136,163,104,35,77,36,77,128,25,186,164,184,106,142,90,205,226,225,183,188,238,216,39,170,39,128,147,164,32,132,128,38,87,232,174,239,55,24,188,143,136];
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
        let referee_signature: [u8; 64] = [72,155,131,199,70,217,206,9,238,243,125,131,233,16,113,147,237,45,44,8,48,49,5,235,95,141,51,157,99,102,73,81,94,49,55,4,243,76,93,17,67,246,78,242,13,156,69,160,210,146,111,237,74,111,133,199,194,222,172,33,60,151,150,141];
        let worker_signature: [u8; 64] = [40,231,175,248,39,202,48,98,84,109,168,192,214,225,76,100,225,33,211,223,70,3,202,197,34,141,124,58,113,227,161,32,102,21,211,182,228,105,189,232,252,2,128,22,49,253,250,139,190,246,182,10,127,169,36,84,154,246,246,75,55,123,31,133];
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

        let referee_signature: [u8; 64] = [186,113,112,231,103,180,169,160,159,206,125,143,33,140,75,171,112,254,216,62,133,202,238,29,191,155,147,2,87,224,198,94,124,52,87,54,172,55,181,167,195,185,160,29,248,99,116,131,74,198,91,154,249,14,37,246,86,74,56,136,230,60,58,142];
        let worker_signature: [u8; 64] = [230,29,27,157,96,227,37,25,221,190,226,210,72,103,25,228,36,72,3,6,200,227,125,136,163,104,35,77,36,77,128,25,186,164,184,106,142,90,205,226,225,183,188,238,216,39,170,39,128,147,164,32,132,128,38,87,232,174,239,55,24,188,143,136];
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
        let referee_signature: [u8; 64] = [200,54,61,255,184,90,244,170,144,70,170,44,204,125,4,91,122,173,171,216,201,155,63,104,65,233,199,85,227,73,205,40,62,19,39,180,76,135,20,111,64,78,114,72,124,60,12,183,12,116,70,102,31,121,40,170,206,137,249,163,133,83,211,143];
        let worker_signature: [u8; 64] = [230,29,27,157,96,227,37,25,221,190,226,210,72,103,25,228,36,72,3,6,200,227,125,136,163,104,35,77,36,77,128,25,186,164,184,106,142,90,205,226,225,183,188,238,216,39,170,39,128,147,164,32,132,128,38,87,232,174,239,55,24,188,143,136];
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
        let referee_signature: [u8; 64] = [186,113,112,231,103,180,169,160,159,206,125,143,33,140,75,171,112,254,216,62,133,202,238,29,191,155,147,2,87,224,198,94,124,52,87,54,172,55,181,167,195,185,160,29,248,99,116,131,74,198,91,154,249,14,37,246,86,74,56,136,230,60,58,142];
        let worker_signature: [u8; 64] = [230,29,27,157,96,227,37,25,221,190,226,210,72,103,25,228,36,72,3,6,200,227,125,136,163,104,35,77,36,77,128,25,186,164,184,106,142,90,205,226,225,183,188,238,216,39,170,39,128,147,164,32,132,128,38,87,232,174,239,55,24,188,143,136];
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
        let referee_signature: [u8; 64] = [186,113,112,231,103,180,169,160,159,206,125,143,33,140,75,171,112,254,216,62,133,202,238,29,191,155,147,2,87,224,198,94,124,52,87,54,172,55,181,167,195,185,160,29,248,99,116,131,74,198,91,154,249,14,37,246,86,74,56,136,230,60,58,142];
        let worker_signature: [u8; 64] = [108,158,183,7,159,239,212,10,206,4,178,112,204,71,190,137,61,245,190,105,76,164,155,170,57,21,220,55,145,108,177,10,141,41,157,208,8,91,160,110,132,171,201,169,64,163,148,255,228,227,181,9,54,134,122,79,86,155,159,253,3,34,68,137];
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