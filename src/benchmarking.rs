use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

pub const REFEREE_ID: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];
pub const WORKER_ID: [u8; 32] = [
    142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201,
    18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
];
pub const EMPLOYER_ID: [u8; 32] = [
    254, 101, 113, 125, 173, 4, 71, 215, 21, 246, 96, 160, 165, 132, 17, 222, 80, 155, 66, 230,
    239, 184, 55, 95, 86, 47, 88, 165, 84, 213, 134, 14,
];
pub const MALICIOUS_ID: [u8; 32] = [
    6, 166, 233, 218, 61, 113, 34, 154, 171, 136, 61, 55, 234, 9, 139, 102, 146, 228, 207, 22, 127,
    51, 101, 144, 61, 162, 221, 109, 87, 172, 54, 120,
];
pub const INITIAL_BALANCE: u32 = 1000;
pub const REFEREE_STAKE: u32 = 10;
pub const PARA_ID: u32 = 1;
pub const LETTER_ID: u32 = 1;
pub const LAST_VALID_BLOCK_NUMBER: u64 = 100;

benchmarks! {
    reimburse {
        let referee_signature: [u8; 64] = [216,163,30,143,81,31,127,209,105,35,237,107,180,150,128,121,166,124,79,247,98,190,97,211,154,50,146,127,246,177,57,5,204,56,21,52,78,158,254,146,128,63,126,181,50,45,46,9,96,32,200,236,244,25,100,169,213,236,67,172,140,66,232,139];
        let worker_signature: [u8; 64] = [168,174,81,192,173,33,161,63,219,108,119,65,205,98,248,17,248,180,216,88,217,168,129,79,11,151,82,9,19,250,17,95,145,12,117,145,145,6,96,37,240,65,79,8,179,109,8,110,110,215,221,35,100,45,219,34,170,196,28,17,68,102,111,135];


        let caller = whitelisted_caller();
        let para_id = 1 as u32;
        let letter_id = 1 as u32;
        let block_number = 100 as u64;
        let ask_price: BalanceOf<T> = REFEREE_STAKE.into();

        let referee_id = H256::from(REFEREE_ID);
        let worker_id = H256::from(WORKER_ID);
        let employer_id = H256::from(EMPLOYER_ID);
        let referee_sign = H512::from(referee_signature);
        let worker_sign = H512::from(worker_signature);
    }: _(RawOrigin::Signed(caller), PARA_ID,
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
