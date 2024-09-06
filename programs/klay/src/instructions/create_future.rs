use anchor_lang::{prelude::*, solana_program:: system_instruction};
use anchor_spl::token::TokenAccount;
use crate::{AssetType, EscrowWallet, FutureContract};
use crate::errors::*;
use anchor_spl::token::{self, Mint, Token};
use anchor_spl::associated_token::*;

const FUTURES_CONTRACT_SIZE: usize = 64;
const ESCROW_WALLET_SIZE: usize = 16;

pub fn create_futures_contract(
    ctx: Context<CreateFuture>,
    seller: Pubkey,
    buyer: Pubkey,
    underlying_asset: String,
    underlying_asset_type: AssetType,
    exchange_asset: String,
    exchange_asset_type: AssetType,
    sell_amount: u64,
    buy_amount: u64,
    margin_collateral: u64,
    start_date: u64,
    expiration_date: u64,
) -> Result<()> {
    msg!("Running some checks...");
    
    require!(underlying_asset.len() != 0, ParameterError::InvalidAssetName);
    require!(buy_amount > 0, ParameterError::InvalidContractPrice);
    require!(expiration_date > 0, ParameterError::InvalidExpirationDate);

    msg!("Initializing a future contract...");

    let futures_contract = &mut ctx.accounts.futures_account;
    futures_contract.seller = seller; 
    futures_contract.buyer = buyer;
    futures_contract.underlying_asset = underlying_asset;
    futures_contract.underlying_asset_type = underlying_asset_type.clone();
    futures_contract.exchange_asset = exchange_asset;
    futures_contract.exchange_asset_type = exchange_asset_type.clone();
    futures_contract.sell_amount = sell_amount;
    futures_contract.buy_amount = buy_amount;
    futures_contract.margin_collateral = margin_collateral;
    futures_contract.start_date = start_date;
    futures_contract.expiration_date = expiration_date;
    futures_contract.settled = false;

    msg!("Contract details confirmed ✅");

    let sell_margin = (margin_collateral * sell_amount) / 100;
    match underlying_asset_type {   
        
        AssetType::SOL => {

            let ix = system_instruction::transfer(
                &ctx.accounts.seller.key(), 
                &ctx.accounts.escrow_wallet.key(), 
                sell_margin
            );

            anchor_lang::solana_program::program::invoke(
                &ix, 
                &[
                    ctx.accounts.seller.to_account_info(),
                    ctx.accounts.escrow_wallet.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;

            ctx.accounts.escrow_wallet.seller_sol_amount += sell_margin;
        }

        AssetType::SPL => {
            let cpi_accounts = token::Transfer {
                from: ctx.accounts.seller_ata.to_account_info(),
                to: ctx.accounts.seller_escrow_ata.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();

            token::transfer(CpiContext::new(
                cpi_program, cpi_accounts
            ), sell_margin)?;

            ctx.accounts.escrow_wallet.seller_spl_amount += sell_margin;
        }        
    }

    let buy_margin = (margin_collateral * buy_amount) / 100;

    match exchange_asset_type {

        AssetType::SOL => {
            let ix = system_instruction::transfer(
                &ctx.accounts.buyer.key(), 
                &ctx.accounts.escrow_wallet.key(), 
                buy_margin,
            );

            anchor_lang::solana_program::program::invoke(
                &ix, 
                &[
                    ctx.accounts.buyer.to_account_info(),
                    ctx.accounts.escrow_wallet.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;

            ctx.accounts.escrow_wallet.buyer_sol_amount += buy_margin;
        }

        AssetType::SPL => {
            let cpi_accounts = token::Transfer {
                from: ctx.accounts.buyer_ata.to_account_info(),
                to: ctx.accounts.buyer_escrow_ata.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();

            token::transfer(CpiContext::new(
                cpi_program, cpi_accounts
            ), buy_margin)?;

            ctx.accounts.escrow_wallet.buyer_spl_amount += buy_margin;
        }
    }

    msg!("Transaction successful ✅");
    msg!("Futures Contract Created 🗞️");
    Ok(())
}

#[derive(Accounts)]
pub struct CreateFuture<'info> {
    //Seller's Account to sign the tx for depositing marginal collateral
    #[account(mut)]
    pub seller: Signer<'info>,
    //Seller's Associated Token Account if the underlying asset is an SPL token
    #[account(mut)]
    pub seller_ata: Account<'info, TokenAccount>,
    //Buyer's Account to sign the tx for depositing marginal collateral
    #[account(mut)]
    pub buyer: Signer<'info>,
    //Buyer's Associated Token Account if the exchange asset is an SPL token
    #[account(mut)]
    pub buyer_ata: Account<'info, TokenAccount>,
    // Futures contract account
    // Seller pays for the initialization of the futures contract and buyer pays for the initialization of the escrow wallet
    #[account(
        init,
        payer = seller,
        space = FUTURES_CONTRACT_SIZE,
        seeds = [b"futures", seller.key().as_ref()],
        bump
    )]
    pub futures_account: Account <'info, FutureContract>,
    //An escrow wallet to hold on to the marginal collaterals from both parties
    #[account(
        init,
        payer = seller,
        space = 8 + 64, // I will fix you later I promise
        seeds = [b"escrow", seller.key().as_ref(), buyer.key().as_ref()],
        bump    
    )]
    pub escrow_wallet: Account<'info, EscrowWallet>, 
    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,
    //Initialize an ATA for escrow_wallet when seller deposits SPL tokens
    #[account(
        init,
        payer = seller,
        associated_token::mint = token_a_mint,
        associated_token::authority = seller 
    )]
    pub seller_escrow_ata: Account<'info, TokenAccount>,
    //Initialize an ATA for escrow_wallet when buyer deposits SPL tokens
    #[account(
        init,
        payer = buyer,
        associated_token::mint = token_b_mint,
        associated_token::authority = buyer 
    )]
    pub buyer_escrow_ata: Account<'info, TokenAccount>,
    
    //Needed for SPL token transfers
    pub token_program: Program<'info, Token>,
    //Needed for SOL token transfers
    pub system_program: Program<'info, System>,
    //Needed for initializing Associated Token Accounts
    pub associated_token_program: Program<'info, AssociatedToken>
}
