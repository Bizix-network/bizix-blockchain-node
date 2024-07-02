#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_std::vec::Vec;
use scale_info::TypeInfo;

sp_api::decl_runtime_apis! {
    pub trait CompanyRegistryApi<AccountId, Balance> where
        AccountId: Codec + TypeInfo,
        Balance: Codec + TypeInfo,
    {
        fn get_company_data(cui: u16, caller: AccountId) -> Option<Company<AccountId>>;
        fn get_query_fee() -> Balance;
        fn has_paid_for_company_data(caller: AccountId, cui: u16) -> bool;
        fn get_company_data_if_paid(caller: AccountId, cui: u16) -> Option<Company<AccountId>>;
    }
}

// definim structura Company aici pentru a fi compatibilă cu API-ul runtime
#[derive(codec::Encode, codec::Decode, TypeInfo)]
pub struct Company<AccountId> {
    pub cui: u16,
    pub denumire: Vec<u8>,
    pub cod_inmatriculare: Vec<u8>,
    pub euid: Vec<u8>,
    pub stare_firma: Vec<u8>,
    pub adresa_completa: Vec<u8>,
    pub owner: Option<AccountId>,
}