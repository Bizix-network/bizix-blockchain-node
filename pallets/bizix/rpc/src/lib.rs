pub use bizix_core_runtime_api::BizixApi as BizixRuntimeApi;
use jsonrpsee::{
	core::RpcResult,
	proc_macros::rpc,
	types::error::ErrorObject,
};

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{traits::Block as BlockT};
use std::sync::Arc;

#[rpc(client, server)]
pub trait BizixApi<BlockHash> {
	#[method(name = "bizix_getValue")]
	fn get_value(&self, at: Option<BlockHash>) -> RpcResult<u32>;
}

/// A struct that implements the `BizixApi`.
pub struct BizixPallet<C, Block> {
	// If you have more generics, no need to BizixPallet<C, M, N, P, ...>
	// just use a tuple like BizixPallet<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<Block>,	
}

impl<C, Block> BizixPallet<C, Block> {
	/// Create new `BizixPallet` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block> BizixApiServer<<Block as BlockT>::Hash> for BizixPallet<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: BizixRuntimeApi<Block>,
{
    fn get_value(&self, at: Option<<Block as BlockT>::Hash>) -> RpcResult<u32> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(||self.client.info().best_hash);

        //api.get_value(at).map_err(runtime_error_into_rpc_err)
		api.get_value(at).map_err(|err| {
            ErrorObject::owned(
                RUNTIME_ERROR,
                "Runtime error",
                Some(format!("{:?}", err)),
            )
        })
    }
}

const RUNTIME_ERROR: i32 = 1;