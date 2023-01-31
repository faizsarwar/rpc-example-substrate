use sp_api;
// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait AmmEstimateApi {
        fn getEquivalentToken1Estimate(
            pool_Id: u32,
            _amountToken2: u128,
        ) -> u128;    
	}
}