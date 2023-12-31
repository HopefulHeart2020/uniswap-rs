use super::{Factory, Pair, Router};
use crate::{errors::Result, Amount, ProtocolType};
use ethers_contract::builders::ContractCall;
use ethers_core::types::{Address, Chain, H256, U256};
use ethers_providers::Middleware;
use std::{fmt, sync::Arc};

/// A Uniswap V2 protocol implementation.
pub struct Protocol<M> {
    /// The liquidity pair factory.
    factory: Factory<M>,

    /// The swap router.
    router: Router<M>,
}

impl<M> Clone for Protocol<M> {
    fn clone(&self) -> Self {
        Self { factory: self.factory.clone(), router: self.router.clone() }
    }
}

impl<M> fmt::Debug for Protocol<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Protocol")
            .field("factory", &self.factory)
            .field("router", &self.router)
            .finish()
    }
}

impl<M: Middleware> Protocol<M> {
    /// Creates a new instance using the provided client, factory and tokens' addresses.
    pub fn new(client: Arc<M>, factory: Address, router: Address, protocol: ProtocolType) -> Self {
        let factory = Factory::new(client.clone(), factory, protocol);
        let router = Router::new(client, router);
        Self { factory, router }
    }

    /// Creates a new instance by searching for the required addresses in the [addressbook].
    ///
    /// [addressbook]: crate::contracts::addresses
    #[cfg(feature = "addresses")]
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Option<Self> {
        if let (Some(factory), Some(router)) = protocol.try_addresses(chain) {
            let mut this = Self::new(client, factory, router, protocol);
            this.factory.set_chain(chain);
            Some(this)
        } else {
            None
        }
    }

    /// Returns a pointer to the client.
    pub fn client(&self) -> Arc<M> {
        self.factory.client()
    }

    /// Returns the protocol's chain.
    #[inline(always)]
    pub fn chain(&self) -> Option<Chain> {
        self.factory.chain()
    }

    /// Sets the protocol's chain.
    #[inline(always)]
    pub fn set_chain(&mut self, chain: Chain) {
        self.factory.set_chain(chain);
    }

    /* ----------------------------------------- Factory ---------------------------------------- */

    /// Returns a reference to the factory.
    #[inline(always)]
    pub fn factory(&self) -> &Factory<M> {
        &self.factory
    }

    /// The factory's `pair_codehash` method. See documentation of [Factory] for more details.
    #[inline(always)]
    pub fn pair_codehash(&self, chain: Option<Chain>) -> H256 {
        self.factory.pair_code_hash(chain)
    }

    /// The factory's `create_pair` method. See documentation of [Factory] for more details.
    #[inline(always)]
    pub fn create_pair(&self, token_a: Address, token_b: Address) -> ContractCall<M, Address> {
        self.factory.contract().create_pair(token_a, token_b)
    }

    /// The factory's `pair_for` method. See documentation of [Factory] for more details.
    #[inline(always)]
    pub fn pair_for(&self, token_a: Address, token_b: Address) -> Pair<M> {
        self.factory.pair_for(token_a, token_b)
    }

    /* ----------------------------------------- Router ----------------------------------------- */

    /// Returns the router.
    pub fn router(&self) -> &Router<M> {
        &self.router
    }

    /// The router's `add_liquidity` method. See documentation of [Router] for more details.
    #[inline(always)]
    pub fn add_liquidity(
        &self,
        token_a: Address,
        token_b: Address,
        amount_a_desired: U256,
        amount_b_desired: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Address,
        deadline: U256,
    ) -> Result<ContractCall<M, (U256, U256, U256)>> {
        self.router.add_liquidity(
            token_a,
            token_b,
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
            to,
            deadline,
        )
    }

    /// The router's `remove_liquidity` method. See documentation of [Router] for more details.
    #[inline(always)]
    pub fn remove_liquidity(
        &self,
        token_a: Address,
        token_b: Address,
        liquidity: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Address,
        deadline: U256,
    ) -> Result<ContractCall<M, (U256, U256)>> {
        self.router.remove_liquidity(
            token_a,
            token_b,
            liquidity,
            amount_a_min,
            amount_b_min,
            to,
            deadline,
        )
    }

    /// The router's `swap` method. See documentation of [Router] for more details.
    #[inline(always)]
    pub async fn swap(
        &self,
        amount: Amount,
        slippage_tolerance: f32,
        path: &[Address],
        to: Address,
        deadline: U256,
        weth: Address,
    ) -> Result<ContractCall<M, Vec<U256>>> {
        self.router.swap(&self.factory, amount, slippage_tolerance, path, to, deadline, weth).await
    }
}
