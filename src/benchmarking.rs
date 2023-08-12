use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

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

benchmarks! {
    reimburse {
        let referee_signature: [u8; 64] = [192,168,180,59,190,113,70,166,190,179,57,76,77,117,102,239,61,19,115,5,206,204,106,78,195,76,187,17,111,149,132,125,194,79,19,81,157,96,17,185,11,18,174,193,130,130,124,192,26,64,79,233,29,51,167,49,232,157,232,88,70,245,187,133];
        let worker_signature: [u8; 64] = [96,34,94,60,39,205,167,96,214,38,19,133,118,122,208,8,251,75,140,183,135,32,36,149,178,108,236,142,43,104,82,88,112,254,106,45,212,164,87,93,76,77,8,59,37,35,184,18,125,214,237,166,248,170,190,159,27,38,126,224,240,208,134,138];
        

        let caller = whitelisted_caller();
        let letter_id = 1 as u32;
        let block_number = 100 as u64;
        let ask_price: BalanceOf<T> = REFEREE_STAKE.into();

        let referee_id = H256::from(REFEREE_ID);
        let worker_id = H256::from(WORKER_ID);
        let employer_id = H256::from(EMPLOYER_ID);
        let referee_sign = H512::from(referee_signature);
        let worker_sign = H512::from(worker_signature);
    }: _(RawOrigin::Signed(caller),
    LETTER_ID,
    LAST_VALID_BLOCK_NUMBER,
    H256::from(REFEREE_ID),
    H256::from(WORKER_ID),
    H256::from(EMPLOYER_ID),
    ask_price,
    H512::from(referee_signature),
    H512::from(worker_signature))
}

impl_benchmark_test_suite!(Pallet, crate::tests::new_test_ext(), crate::tests::Test,);
