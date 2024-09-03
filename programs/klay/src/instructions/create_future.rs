use anchor_lang::{prelude::*, solana_program::{native_token::sol_to_lamports, system_instruction}};
use crate::SolFutureContract;
use crate::errors::*;

const FUTURES_CONTRACT_SIZE: usize = 64;
const ESCROW_WALLET_SIZE: usize = 16;

const INIT_MARGIN_PERCENTAGE: u64 = 10; //10%

pub fn create_futures_contract(
    ctx: Context<CreateFuture>,
    underlying_asset: String,
    contract_price: f64,
    expiration_date: u64,
) -> Result<()> {
    msg!("Running some checks...");
    
    require!(underlying_asset.len() != 0, ParameterError::InvalidAssetName);
    require!(contract_price > 0.0, ParameterError::InvalidContractPrice);
    require!(expiration_date > 0, ParameterError::InvalidExpirationDate);

    msg!("Initializing a future contract...");

    let futures_contract = &mut ctx.accounts.futures_account;
    futures_contract.underlying_asset = underlying_asset;
    futures_contract.contract_price = contract_price;
    futures_contract.expiration_date = expiration_date;

    msg!("Contract details confirmed ✅");
    msg!("Calculating deposit margin...");
    //This is the collateral the creator will have to deposit into the escrow wallet to ensure trust 💰
    // deposit margin will be in lamports
    let deposit_margin = (sol_to_lamports(contract_price) * INIT_MARGIN_PERCENTAGE) / 100;

    msg!("Amount to deposit as margin: {}", deposit_margin);
    msg!("Transferring funds 🚚");
    //After entering the details for the futures contract now we need to deposit the initial margin 
    //Creating the transfer instruction
    let tx_ix = system_instruction::transfer(
        &ctx.accounts.creator.key(), 
        &ctx.accounts.escrow_wallet.key(), 
        deposit_margin 
    );

    //Invoke the transfer instruction
    anchor_lang::solana_program::program::invoke_signed(
        &tx_ix, 
        &[
            ctx.accounts.creator.to_account_info(),
            ctx.accounts.escrow_wallet.clone(),
            ctx.accounts.system_program.to_account_info(),
        ], 
        &[]
    )?;

    msg!("Transaction successful ✅");
    msg!("Futures Contract Created 🗞️");
    Ok(())
}

#[derive(Accounts)]
pub struct CreateFuture<'info> {
    // User creating a futures contract
    #[account(mut, signer)]
    pub creator: Signer<'info>,
    // Futures contract account
    #[account(
        init,
        payer = creator,
        space = FUTURES_CONTRACT_SIZE,
        seeds = [b"futures", creator.key().as_ref()],
        bump
    )]
    pub futures_account: Account <'info, SolFutureContract>,
    //Escrow wallet to hold the margin amount
    #[account(
        init,
        payer = creator,
        space = ESCROW_WALLET_SIZE,
        seeds = [b"escrow"],
        bump
    )]
    pub escrow_wallet: AccountInfo<'info>,    
    //Initializes the future contract account
    pub system_program: Program<'info, System>,
}
