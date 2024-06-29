pub use pallet_company_registry_rpc_runtime_api::CompanyRegistryApi as CompanyRegistryRuntimeApi;
use codec::Codec;
use jsonrpsee::{
	core::RpcResult,
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{traits::Block as BlockT};
use std::sync::Arc;

#[rpc(client, server)]
pub trait CompanyRegistryApi<BlockHash, AccountId, Balance> {
    #[method(name = "companyRegistry_getCompanyData")]
    fn get_company_data(&self, cui: Vec<u8>, at: Option<BlockHash>) -> RpcResult<Option<Vec<u8>>>;

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

impl<C, Block, AccountId, Balance> CompanyRegistryApiServer<<Block as BlockT>::Hash, AccountId, Balance>
    for CompanyRegistry<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: CompanyRegistryRuntimeApi<Block, AccountId, Balance>,
    AccountId: Codec,
    Balance: Codec,
{
    fn get_company_data(&self, cui: Vec<u8>, at: Option<<Block as BlockT>::Hash>) -> RpcResult<Option<Vec<u8>>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(|| self.client.info().best_hash);

        api.get_company_data(at, cui).map_err(|e| {
            CallError::Custom(ErrorObject::owned(
                Error::RuntimeError.into(),
                "Unable to query company data.",
                Some(format!("{:?}", e)),
            ))
            .into()
        })
    }

    fn get_query_fee(&self, at: Option<<Block as BlockT>::Hash>) -> RpcResult<Balance> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(|| self.client.info().best_hash);

        api.get_query_fee(at).map_err(|e| {
            CallError::Custom(ErrorObject::owned(
                Error::RuntimeError.into(),
                "Unable to query fee.",
                Some(format!("{:?}", e)),
            ))
            .into()
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Runtime error")]
    RuntimeError,
}

impl From<Error> for i32 {
    fn from(e: Error) -> i32 {
        match e {
            Error::RuntimeError => 1,
        }
    }
}

const RUNTIME_ERROR: i32 = 1;