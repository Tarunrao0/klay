use anchor_lang::prelude::*;

#[account]
#[derive(Default)]

pub struct SolFutureContract {
    // Item that will be delivered or settled at the expiration of the contract
    pub underlying_asset: String, 
    // Predetermined price at which the underlying asset will be bought or sold when the futures contract reaches its expiration date
    pub contract_price: f64,
    // Date of execution (in seconds)
    pub expiration_date: u64,
    // Current market price of the underlying asset
    // pub current_price: u64, we'lll let this be done in typescript with real time price-feeds
}

pub struct SplFutureContract {

}