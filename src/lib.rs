#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    pallet_prelude::*,
    traits::{Currency, ExistenceRequirement},
    transactional,
};
use frame_system::pallet_prelude::*;
use sp_std::{convert::TryInto, prelude::*};

use sp_core::sr25519::{Public, Signature};
use sp_core::{H256, H512};

use sp_runtime::traits::SaturatedConversion;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::AccountId32;

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod tests;
pub mod weights;

pub use weights::WeightInfo;

/// Struct for holding recommendation letter information.
pub struct LetterCoordinates {
    chunk: usize,
    index: usize,
}

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type WeightInfo: WeightInfo;
        #[pallet::constant]
        type DefaultDifficulty: Get<u32>;
        type LettersPerChunk: Get<u32>;
    }
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// A storage for recommendation letters
    /// Keeps track of what accounts issued which letters
    #[pallet::storage]
    #[pallet::getter(fn was_letter_used)]
    pub(super) type OwnedLetersArray<T: Config> =
        StorageMap<_, Twox64Concat, (H256, u64), BoundedVec<bool, T::LettersPerChunk>, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        _phantom: PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ReimbursementHappened(H256, u64),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidRefereeSign,
        InvalidWorkerSign,
        InvalidLetterAmount,
        RefereeBalanceIsNotEnough,
        LetterWasMarkedAsFraudBefore,
        Expired,
        NotAllowedBlock,
        WrongParaId,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // A reimbursement functionality. A referee should should pay initially defined Balance sum if employer thinks that the letter is wrong.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::reimburse())]
        #[transactional]
        pub fn reimburse(
            origin: OriginFor<T>,
            letter_id: u32,
            block_number: u64,
            block_allowed: u64,
            referee_id: H256,
            worker_id: H256,
            employer_id: H256,
            ask_price: BalanceOf<T>,
            referee_sign: H512,
            worker_sign: H512,
        ) -> DispatchResultWithPostInfo {
            let _sender = ensure_signed(origin)?;

            // Get genesis_hash
            let zero_block: BlockNumberFor<T> = 0u64.saturated_into::<BlockNumberFor<T>>();
            let genesis_hash = frame_system::Pallet::<T>::block_hash(zero_block);

            ensure!(
                frame_system::Pallet::<T>::block_number().saturated_into::<u64>() <= block_number,
                Error::<T>::Expired
            );

            ensure!(
                frame_system::Pallet::<T>::block_number().saturated_into::<u64>() <= block_allowed,
                Error::<T>::NotAllowedBlock
            );

            let genesis_hash_bytes = &genesis_hash.as_ref();
            let letter_id_bytes = &letter_id.to_be_bytes();
            let block_number_bytes = &block_number.to_be_bytes();
            let block_allowed_bytes = &block_allowed.to_be_bytes();
            let referee_id_bytes = referee_id.as_bytes();
            let employer_id_bytes = employer_id.as_bytes();
            let worker_id_bytes = worker_id.as_bytes();

            let ask_price_u128 = TryInto::<u128>::try_into(ask_price)
                .map_err(|_| Error::<T>::InvalidLetterAmount)?;
            let ask_price_bytes = &ask_price_u128.to_be_bytes();

            let mut skill_receipt_data = Vec::new();
            skill_receipt_data.extend_from_slice(genesis_hash_bytes);
            skill_receipt_data.extend_from_slice(letter_id_bytes);
            skill_receipt_data.extend_from_slice(block_number_bytes);
            skill_receipt_data.extend_from_slice(referee_id_bytes);
            skill_receipt_data.extend_from_slice(worker_id_bytes);
            skill_receipt_data.extend_from_slice(ask_price_bytes);

            ensure!(
                Self::signature_is_valid(
                    referee_sign.clone(),
                    skill_receipt_data.clone(),
                    referee_id.clone()
                ),
                Error::<T>::InvalidRefereeSign
            );

            let mut skill_letter_data = Vec::new();
            skill_letter_data.extend_from_slice(letter_id_bytes);
            skill_letter_data.extend_from_slice(block_number_bytes);
            skill_letter_data.extend_from_slice(block_allowed_bytes);
            skill_letter_data.extend_from_slice(referee_id_bytes);
            skill_letter_data.extend_from_slice(worker_id_bytes);
            skill_letter_data.extend_from_slice(ask_price_bytes);
            skill_letter_data.extend_from_slice(referee_sign.as_bytes());
            skill_letter_data.extend_from_slice(employer_id.as_bytes());
            
            ensure!(
                Self::signature_is_valid(worker_sign, skill_letter_data, worker_id.clone()),
                Error::<T>::InvalidWorkerSign
            );

            ensure!(
                !Self::was_letter_canceled(referee_id, letter_id as usize),
                Error::<T>::LetterWasMarkedAsFraudBefore
            );

            T::Currency::transfer(
                &Self::account_id_from(referee_id_bytes),
                &Self::account_id_from(employer_id_bytes),
                ask_price,
                ExistenceRequirement::KeepAlive,
            )
            .map_err(|_| Error::<T>::RefereeBalanceIsNotEnough)?;

            Self::mark_letter_as_fraud(referee_id, letter_id as usize)?;

            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {}

const INSURANCE_PER_CHUNK: usize = 1000;
impl<T: Config> Pallet<T> {
    /// A wrapper function to provide AccountId
    fn account_id_from(account_bytes: &[u8]) -> T::AccountId {
        //
        let referee_bytes_array: [u8; 32] = Self::slice_to_array(account_bytes);
        let referee: AccountId32 = AccountId32::new(referee_bytes_array);
        let mut referee_init_account32 = AccountId32::as_ref(&referee);
        T::AccountId::decode(&mut referee_init_account32).unwrap()
    }
    /// A wrapper function to validate signatures
    fn signature_is_valid(signature: H512, message: Vec<u8>, pubkey: H256) -> bool {
        let mut data_signed_by_extension = Vec::new();
        data_signed_by_extension.extend_from_slice(b"<Bytes>");
        data_signed_by_extension.extend_from_slice(&message);
        data_signed_by_extension.extend_from_slice(b"</Bytes>");

        sp_io::crypto::sr25519_verify(
            &Signature::from_raw(*signature.as_fixed_bytes()),
            &data_signed_by_extension,
            &Public::from_h256(pubkey),
        )
    }
    fn slice_to_array(barry: &[u8]) -> [u8; 32] {
        let mut array = [0u8; 32];
        for (&x, p) in barry.iter().zip(array.iter_mut()) {
            *p = x;
        }
        array
    }

    /// A helper function to mark recommendation letter as used.
    fn mint_chunk(to: H256, chunk: usize) -> DispatchResult {
        ensure!(
            !<OwnedLetersArray<T>>::contains_key((to.clone(), chunk as u64)),
            "Letter already contains_key"
        );
        let data: BoundedVec<bool, T::LettersPerChunk> =
            (vec![true; INSURANCE_PER_CHUNK]).try_into().unwrap();
        // Write Letter counting information to storage.
        <OwnedLetersArray<T>>::insert((to.clone(), chunk as u64), data);
        Ok(())
    }

    /// A helper function to find out if the storage contains a chunk
    fn chunk_exists(to: H256, chunk: usize) -> bool {
        <OwnedLetersArray<T>>::contains_key((to.clone(), chunk as u64))
    }
    /// Convert a letter id to coordinates to be used at the storage
    fn coordinates_from_letter_index(number: usize) -> LetterCoordinates {
        let chunk = number / INSURANCE_PER_CHUNK;
        let index = number % INSURANCE_PER_CHUNK;
        LetterCoordinates { chunk, index }
    }
    /// Convert coordinates of letter at storage to a letter id
    #[allow(dead_code)]
    fn letter_index_from_coordinates(coordinates: LetterCoordinates) -> usize {
        coordinates.chunk * INSURANCE_PER_CHUNK + coordinates.index
    }
    /// Shows if letter was used for referee penalization to exclude multiple penalizaions for the same letter
    /// Used letters are marked as false
    fn was_letter_canceled(referee: H256, number: usize) -> bool {
        let coordinates = Self::coordinates_from_letter_index(number);
        match Self::chunk_exists(referee, coordinates.chunk) {
            false => false,
            true => {
                let data = <OwnedLetersArray<T>>::get((referee.clone(), coordinates.chunk as u64));
                !data[coordinates.index] //used letters are marked as false
            }
        }
    }
    /// Mark a recommendation letter as fraud
    fn mark_letter_as_fraud(referee: H256, letter_number: usize) -> DispatchResult {
        let coordinates = Self::coordinates_from_letter_index(letter_number);
        if !Self::chunk_exists(referee, coordinates.chunk) {
            Self::mint_chunk(referee, coordinates.chunk)?;
        }
        let mut data = <OwnedLetersArray<T>>::get((referee.clone(), coordinates.chunk as u64));
        data[coordinates.index] = false;
        <OwnedLetersArray<T>>::remove((referee.clone(), coordinates.chunk as u64));
        <OwnedLetersArray<T>>::insert((referee.clone(), coordinates.chunk as u64), data);
        // Write `mint` event
        Self::deposit_event(Event::ReimbursementHappened(
            referee,
            letter_number as u64,
        ));
        Ok(())
    }
}
