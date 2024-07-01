pub use pallet_company_registry_rpc_runtime_api::CompanyRegistryApi as CompanyRegistryRuntimeApi;
use codec::Codec;
use jsonrpsee::{
    core::RpcResult,
    proc_macros::rpc,
    types::error::ErrorObject,
};

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

use serde::{Serialize, Deserialize};
use sp_runtime::traits::MaybeDisplay;

#[rpc(client, server)]
pub trait CompanyRegistryApi<BlockHash, AccountId, Balance> {
    #[method(name = "companyRegistry_getCompanyData")]
    fn get_company_data(&self, cui: u16, caller: AccountId, at: Option<BlockHash>) -> RpcResult<Option<CompanyData>>;

    #[method(name = "companyRegistry_getQueryFee")]
    fn get_query_fee(&self, at: Option<BlockHash>) -> RpcResult<Balance>;
}

pub struct CompanyRegistry<C, Block> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, Block> CompanyRegistry<C, Block> {
    pub fn new(client: Arc<C>) -> Self {
        Self { client, _marker: Default::default() }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompanyData {
    pub cui: u16,
    pub denumire: String,
    pub cod_inmatriculare: String,
    pub euid: String,
    pub stare_firma: String,
    pub adresa_completa: String,
    pub owner: Option<String>, // Vom folosi String pentru a reprezenta AccountId
}

pub trait AccountIdSerialize: Codec + Clone + MaybeDisplay {}
impl<T: Codec + Clone + MaybeDisplay> AccountIdSerialize for T {}

impl<C, Block, AccountId, Balance> CompanyRegistryApiServer<<Block as BlockT>::Hash, AccountId, Balance>
    for CompanyRegistry<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: CompanyRegistryRuntimeApi<Block, AccountId, Balance>,
    AccountId: Codec + Clone + sp_std::fmt::Display + scale_info::TypeInfo,
    Balance: Codec + scale_info::TypeInfo,
{
     fn get_company_data(&self, cui: u16, caller: AccountId, at: Option<<Block as BlockT>::Hash>) -> RpcResult<Option<CompanyData>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(|| self.client.info().best_hash);

        let company = api.get_company_data(at, cui, caller).map_err(|err| {
            ErrorObject::owned(
                RUNTIME_ERROR,
                "Unable to query company data",
                Some(format!("{:?}", err)),
            )
        })?;

        Ok(company.map(|c| CompanyData {
            cui: c.cui,
            denumire: String::from_utf8_lossy(&c.denumire).into_owned(),
            cod_inmatriculare: String::from_utf8_lossy(&c.cod_inmatriculare).into_owned(),
            euid: String::from_utf8_lossy(&c.euid).into_owned(),
            stare_firma: String::from_utf8_lossy(&c.stare_firma).into_owned(),
            adresa_completa: String::from_utf8_lossy(&c.adresa_completa).into_owned(),
            owner: c.owner.map(|a| a.to_string()),
        }))
    }

    fn get_query_fee(&self, at: Option<<Block as BlockT>::Hash>) -> RpcResult<Balance> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(|| self.client.info().best_hash);

        api.get_query_fee(at).map_err(|err| {
            ErrorObject::owned(
                RUNTIME_ERROR,
                "Unable to query fee",
                Some(format!("{:?}", err)),
            )
        })
    }
}

const RUNTIME_ERROR: i32 = 1;