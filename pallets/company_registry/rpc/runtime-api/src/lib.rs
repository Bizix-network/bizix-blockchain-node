#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
    pub trait CompanyRegistryApi<AccountId, Balance> where
        AccountId: Codec,
        Balance: Codec,
    {
        fn get_company_data(cui: Vec<u8>) -> Option<Vec<u8>>;
        fn get_query_fee() -> Balance;
    }
}