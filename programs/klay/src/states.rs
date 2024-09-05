use anchor_lang::prelude::*;

#[account]
pub struct FutureContract {
    // User selling the asset
    pub seller: Pubkey,
    // User interested in buying the asset being sold
    pub buyer: Pubkey,
    // Item that will be delivered or settled at the expiration of the contract
    pub underlying_asset: String, 
    // Checks if the underlying asset is SOL/SPL token
    pub underlying_asset_type: AssetType,
    // Asset with which the underlying_asset will be bought
    pub exchange_asset: String,
    // Checks if the underlying asset is SOL/SPL token
    pub exchange_asset_type: AssetType,
    // Amount of SOL being sold
    pub sell_amount: f64,
    // Predetermined price at which the underlying asset will be bought when the futures contract reaches its expiration date
    pub buy_amount: f64,
    // Margin collateral required. This will be percentage like 0.05 would be 5%
    pub margin_collateral: f64,
    // Start date (in seconds)
    pub start_date: u64,
    // Date of execution (in seconds)
    pub expiration_date: u64,
    // Flag for settlement
    pub settled: bool
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Clone)]
pub enum AssetType {
    SOL,
    SPL,
}

#[account]
pub struct EscrowWallet {
    // Amount of SOL deposited by the seller
    pub seller_sol_amount: u64, 
    // Amount of SPL tokens deposited by the seller
    pub seller_spl_amount: u64,  
    // Amount of SOL deposited by the buyer
    pub buyer_sol_amount: u64,  
    // Amount of SPL tokens deposited by the buyer 
    pub buyer_spl_amount: u64,  
}