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

use hex_literal::hex;

pub const REFEREE_ID: [u8; 32] = hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
pub const WORKER_ID: [u8; 32] = hex!("8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48");
pub const EMPLOYER_ID: [u8; 32] = hex!("fe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e");
pub const MALICIOUS_ID: [u8; 32] = hex!("badbadbadd0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e");
pub const INITIAL_BALANCE: u64 = 1000;
pub const REFEREE_STAKE: u64 = 10;
pub const LETTER_ID: u32 = 1;

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
fn referee_has_not_enough_balance() {
	new_test_ext().execute_with(|| {
		//Data to be signed is represented as u8 array
		//letter_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)

		// letter_id (1): [0, 0, 0, 1] // println!("letter_id (1 as u32): {:?}", (1 as u32).to_be_bytes());//
		// letter_id (2): [0, 0, 0, 2] // println!("letter_id (2 as u32): {:?}", (2 as u32).to_be_bytes());

		// amount (10 as u128): [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10] // println!("amount (10 as u128): {:?}", (10 as u128).to_be_bytes());

		// Data to be signed by referee:
		// letter_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)
		// 1 , REFEREE_ID, WORKER_ID, 10 - see below:
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		//
		// Referee signature: [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		//
		// DATA TO BE SIGNED BY STUDENT
		// 1 , REFEREE_ID, WORKER_ID, 10, referee_signATURE, EMPLOYER_ID
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		// [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		// [166, 82, 220, 58, 28, 232, 181, 15, 154, 161, 152, 109, 179, 47, 157, 32, 202, 28, 33, 243, 219, 161, 164, 110, 173, 174, 79, 180, 188, 244, 227, 86]
		//

		let referee_signature: [u8; 64] = hex!("2e4e320dd4e6a289795cf51f60bc385dd19c41ccaa0f77c1f7c5c10cd2583a4c8ca01899e3720f5dd4974f695389c9bea6e5839dd692bdebd30c3220f740fb8a");
		let worker_signature: [u8; 64] = hex!("3e244a3e0ea0b261ed7bd6bd4c538ee9e1e13ab797d4c245c9fc94e98e36eb79b4366380262e9d609257af9b55afbfc9afc72bfb8f860b7e0522db1f02ed9588");

		Balances::make_free_balance_be(
			&AccountId::from(Public::from_raw(REFEREE_ID)).into_account(),
			9,
		);
		assert_noop!(
			LettersModule::reimburse(
				Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
				LETTER_ID,
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
fn wrong_referee_sign() {
	new_test_ext().execute_with(|| {
		//Data to be signed is represented as u8 array
		//letter_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)

		// letter_id (1): [0, 0, 0, 1] // println!("letter_id (1 as u32): {:?}", (1 as u32).to_be_bytes());//
		// letter_id (2): [0, 0, 0, 2] // println!("letter_id (2 as u32): {:?}", (2 as u32).to_be_bytes());

		// amount (10 as u128): [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10] // println!("amount (10 as u128): {:?}", (10 as u128).to_be_bytes());

		// Data to be signed by referee:
		// letter_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)
		// 1 , REFEREE_ID, WORKER_ID, 10 - see below:
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		//
		// Referee signature: [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		//
		// DATA TO BE SIGNED BY STUDENT
		// 1 , REFEREE_ID, WORKER_ID, 10, referee_signATURE, EMPLOYER_ID
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		// [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		// [166, 82, 220, 58, 28, 232, 181, 15, 154, 161, 152, 109, 179, 47, 157, 32, 202, 28, 33, 243, 219, 161, 164, 110, 173, 174, 79, 180, 188, 244, 227, 86]
		//

		// let referee_signature: [u8; 64] = [
		// 	96, 20, 15, 21, 11, 137, 10, 192, 129, 3, 154, 34, 203, 118, 28, 19, 176, 54, 165, 181,
		// 	227, 156, 70, 197, 73, 86, 226, 111, 137, 243, 69, 95, 41, 74, 25, 254, 228, 34, 212,
		// 	189, 141, 134, 194, 44, 229, 172, 27, 43, 67, 73, 73, 58, 61, 63, 37, 176, 120, 195,
		// 	153, 198, 46, 42, 231, 129,
		// ];

		let wrong_referee_signature: [u8; 64] = hex!("bade320dd4e6a289795cf51f60bc385dd19c41ccaa0f77c1f7c5c10cd2583a4c8ca01899e3720f5dd4974f695389c9bea6e5839dd692bdebd30c3220f740fb8a");
		let worker_signature: [u8; 64] = hex!("3e244a3e0ea0b261ed7bd6bd4c538ee9e1e13ab797d4c245c9fc94e98e36eb79b4366380262e9d609257af9b55afbfc9afc72bfb8f860b7e0522db1f02ed9588");


		assert_noop!(
			LettersModule::reimburse(
				Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
				LETTER_ID,
				H256::from(REFEREE_ID),
				H256::from(WORKER_ID),
				H256::from(EMPLOYER_ID),
				REFEREE_STAKE,
				H512::from(wrong_referee_signature),
				H512::from(worker_signature)
			),
			Error::<Test>::InvalidRefereeSign
		);
	});
}

#[test]
fn wrong_worker_sign() {
	new_test_ext().execute_with(|| {
		//Data to be signed is represented as u8 array
		//letter_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)

		// letter_id (1): [0, 0, 0, 1] // println!("letter_id (1 as u32): {:?}", (1 as u32).to_be_bytes());//
		// letter_id (2): [0, 0, 0, 2] // println!("letter_id (2 as u32): {:?}", (2 as u32).to_be_bytes());

		// amount (10 as u128): [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10] // println!("amount (10 as u128): {:?}", (10 as u128).to_be_bytes());

		// Data to be signed by referee:
		// letter_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)
		// 1 , REFEREE_ID, WORKER_ID, 10 - see below:
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		//
		// Referee signature: [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		//
		// DATA TO BE SIGNED BY STUDENT
		// 1 , REFEREE_ID, WORKER_ID, 10, referee_signATURE, EMPLOYER_ID
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		// [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		// [166, 82, 220, 58, 28, 232, 181, 15, 154, 161, 152, 109, 179, 47, 157, 32, 202, 28, 33, 243, 219, 161, 164, 110, 173, 174, 79, 180, 188, 244, 227, 86]
		//

		let referee_signature: [u8; 64] = hex!("2e4e320dd4e6a289795cf51f60bc385dd19c41ccaa0f77c1f7c5c10cd2583a4c8ca01899e3720f5dd4974f695389c9bea6e5839dd692bdebd30c3220f740fb8a");
		let wrong_worker_signature: [u8; 64] = hex!("bad44a3e0ea0b261ed7bd6bd4c538ee9e1e13ab797d4c245c9fc94e98e36eb79b4366380262e9d609257af9b55afbfc9afc72bfb8f860b7e0522db1f02ed9588");


		assert_noop!(
			LettersModule::reimburse(
				Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
				LETTER_ID,
				H256::from(REFEREE_ID),
				H256::from(WORKER_ID),
				H256::from(EMPLOYER_ID),
				REFEREE_STAKE,
				H512::from(referee_signature),
				H512::from(wrong_worker_signature)
			),
			Error::<Test>::InvalidWorkerSign
		);
	});
}


#[test]
fn successful_reimburce() {
	new_test_ext().execute_with(|| {
		let referee_hash = H256::from(REFEREE_ID);

		//Data to be signed is represented as u8 array
		//letter_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)

		// letter_id (1): [0, 0, 0, 1] // println!("letter_id (1 as u32): {:?}", (1 as u32).to_be_bytes());//
		// letter_id (2): [0, 0, 0, 2] // println!("letter_id (2 as u32): {:?}", (2 as u32).to_be_bytes());

		// amount (10 as u128): [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10] // println!("amount (10 as u128): {:?}", (10 as u128).to_be_bytes());

		// Data to be signed by referee:
		// letter_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)
		// 1 , REFEREE_ID, WORKER_ID, 10 - see below:
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		//
		// Referee signature: [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		//
		// DATA TO BE SIGNED BY STUDENT
		// 1 , REFEREE_ID, WORKER_ID, 10, referee_signATURE, EMPLOYER_ID
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		// [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		// [166, 82, 220, 58, 28, 232, 181, 15, 154, 161, 152, 109, 179, 47, 157, 32, 202, 28, 33, 243, 219, 161, 164, 110, 173, 174, 79, 180, 188, 244, 227, 86]
		//

		let referee_signature: [u8; 64] = hex!("2e4e320dd4e6a289795cf51f60bc385dd19c41ccaa0f77c1f7c5c10cd2583a4c8ca01899e3720f5dd4974f695389c9bea6e5839dd692bdebd30c3220f740fb8a");
		let worker_signature: [u8; 64] = hex!("3e244a3e0ea0b261ed7bd6bd4c538ee9e1e13ab797d4c245c9fc94e98e36eb79b4366380262e9d609257af9b55afbfc9afc72bfb8f860b7e0522db1f02ed9588");

		let number = 1;
		
		assert_eq!(
			LettersModule::was_letter_canceled(referee_hash.clone(), number),
			false
		);

		assert_ok!(LettersModule::reimburse(
			Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
			LETTER_ID,
			H256::from(REFEREE_ID),
			H256::from(WORKER_ID),
			H256::from(EMPLOYER_ID),
			REFEREE_STAKE,
			H512::from(referee_signature),
			H512::from(worker_signature)
		));

		assert_eq!(
			LettersModule::was_letter_canceled(referee_hash.clone(), number),
			true
		);

		assert_noop!(
			LettersModule::reimburse(
				Origin::signed(AccountId::from(Public::from_raw(REFEREE_ID)).into_account()),
				LETTER_ID,
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
fn right_polkadot_js_local_referee_sign() {
	new_test_ext().execute_with(|| {
		//--------

		let letter_id: u32 = 1;
let ask_price_u128: u128 = 1000000000000000;
let referee_id_bytes: [u8; 32] = [212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125];
let worker_id_bytes: [u8; 32] = [142,175,4,21,22,135,115,99,38,201,254,161,126,37,252,82,135,97,54,147,201,18,144,156,178,38,170,71,148,242,106,72];
let referee_sign_bytes: [u8; 64] = [38,37,182,5,92,234,12,251,164,188,57,56,244,223,246,188,162,80,118,9,226,46,229,207,32,10,217,103,16,11,209,39,100,224,70,17,164,142,222,171,127,82,133,172,139,115,108,143,165,15,2,164,234,120,46,156,104,57,188,69,144,187,91,135];

		//--------
		let ask_price_bytes = &ask_price_u128.to_be_bytes();
		let letter_id_bytes = &letter_id.to_be_bytes();
		let mut skill_receipt_data = Vec::new();
		skill_receipt_data.extend_from_slice(letter_id_bytes);
		skill_receipt_data.extend_from_slice(&referee_id_bytes);
		skill_receipt_data.extend_from_slice(&worker_id_bytes);
		skill_receipt_data.extend_from_slice(ask_price_bytes);

		assert_eq!(
			LettersModule::signature_is_valid(
				H512::from(referee_sign_bytes),
				skill_receipt_data,
				H256::from(referee_id_bytes)
			),
			true
		);
	});
}

#[test]
fn right_polkadot_js_extension_referee_sign() {
	new_test_ext().execute_with(|| {
		//--------
		let letter_id: u32 = 23;
		let ask_price_u128: u128 = 1000000000000000;
		let referee_id_bytes: [u8; 32] = [
			202, 112, 158, 97, 34, 240, 209, 219, 93, 46, 189, 180, 28, 113, 25, 197, 205, 6, 81,
			50, 184, 168, 77, 159, 24, 205, 125, 9, 110, 129, 98, 22,
		];
		let worker_id_bytes: [u8; 32] = [
			142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54,
			147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
		];
		let referee_sign_bytes: [u8; 64] = [
			46, 98, 203, 32, 13, 16, 69, 158, 4, 224, 203, 206, 205, 18, 44, 113, 74, 154, 131, 90,
			154, 30, 71, 181, 186, 130, 120, 30, 8, 253, 177, 25, 26, 56, 200, 13, 48, 180, 5, 9,
			30, 190, 171, 221, 146, 79, 231, 151, 59, 47, 1, 177, 117, 99, 119, 23, 69, 68, 27,
			219, 112, 27, 245, 132,
		];
		//--------
		let ask_price_bytes = &ask_price_u128.to_be_bytes();
		let letter_id_bytes = &letter_id.to_be_bytes();
		let mut skill_receipt_data = Vec::new();
		skill_receipt_data.extend_from_slice(letter_id_bytes);
		skill_receipt_data.extend_from_slice(&referee_id_bytes);
		skill_receipt_data.extend_from_slice(&worker_id_bytes);
		skill_receipt_data.extend_from_slice(ask_price_bytes);

		assert_eq!(
			LettersModule::signature_is_valid(
				H512::from(referee_sign_bytes),
				skill_receipt_data,
				H256::from(referee_id_bytes)
			),
			true
		);
	});
}