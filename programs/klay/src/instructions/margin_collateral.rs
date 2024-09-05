//! Deposits the marginal collateral. If the deposit is an SPL token then an Associated Token Account is created to escrow the collateral

use anchor_lang::{prelude::*, solana_program::system_instruction};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::states::EscrowWallet;
use anchor_spl::associated_token::*;

//We'll have to call this function twice. Once to depoist the seller's collateral and once to deposit the buyer's collateral
pub fn deposit_collateral(ctx: Context<MarginCollateral>, amount: u64, is_seller: bool, is_sol: bool) -> Result<()> {
    
    if is_seller {
        //If the seller's underlying asset is SOL
        if is_sol {
            let ix = system_instruction::transfer(
                &ctx.accounts.seller.key(), 
                &ctx.accounts.escrow_wallet.key(), 
                amount,
            );

            anchor_lang::solana_program::program::invoke(
                &ix, 
                &[
                    ctx.accounts.seller.to_account_info(),
                    ctx.accounts.escrow_wallet.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;

            ctx.accounts.escrow_wallet.seller_sol_amount += amount;
        } else {
            //If Seller is depositing SPL tokens

            let cpi_accounts = token::Transfer {
                from: ctx.accounts.seller_ata.to_account_info(),
                to: ctx.accounts.seller_escrow_ata.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();

            token::transfer(CpiContext::new(
                cpi_program, cpi_accounts
            ), amount)?;

            ctx.accounts.escrow_wallet.seller_spl_amount += amount;
        }
    } else {
        if is_sol {
            let ix = system_instruction::transfer(
                &ctx.accounts.buyer.key(), 
                &ctx.accounts.escrow_wallet.key(), 
                amount,
            );

            anchor_lang::solana_program::program::invoke(
                &ix, 
                &[
                    ctx.accounts.buyer.to_account_info(),
                    ctx.accounts.escrow_wallet.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;

            ctx.accounts.escrow_wallet.buyer_sol_amount += amount;
        } else {
            //If Buyer is depositing SPL tokens

            let cpi_accounts = token::Transfer {
                from: ctx.accounts.buyer_ata.to_account_info(),
                to: ctx.accounts.buyer_escrow_ata.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();

            token::transfer(CpiContext::new(
                cpi_program, cpi_accounts
            ), amount)?;

            ctx.accounts.escrow_wallet.buyer_spl_amount += amount;
        }
    }
    Ok(())
}

#[derive(Accounts)]

pub struct MarginCollateral<'info> {
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
