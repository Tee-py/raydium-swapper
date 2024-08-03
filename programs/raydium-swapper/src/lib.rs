use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_spl::token::TokenAccount;

declare_id!("GqpnKEGRfwwisQxC8j9AHbtWf8BLzdo4mH3P5oJC7FiQ");

#[program]
pub mod raydium_swapper {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, data: Vec<u8>) -> Result<()> {
        let mut accounts: Vec<AccountMeta> = Vec::with_capacity(16);

        accounts.push(AccountMeta::new_readonly(ctx.accounts.token_program.key(), false));
        accounts.push(AccountMeta::new(ctx.accounts.amm_id.key(), false));
        accounts.push(AccountMeta::new_readonly(ctx.accounts.amm_authority.key(), false));
        accounts.push(AccountMeta::new(ctx.accounts.amm_open_orders.key(), false));
        accounts.push(AccountMeta::new(ctx.accounts.amm_base_vault.key(), false));
        accounts.push(AccountMeta::new(ctx.accounts.amm_quote_vault.key(), false));
        accounts.push(AccountMeta::new_readonly(ctx.accounts.amm_market_program_id.key(), false));
        accounts.push(AccountMeta::new(ctx.accounts.amm_market_id.key(), false));
        accounts.push(AccountMeta::new(ctx.accounts.amm_market_bids.key(), false));
        accounts.push(AccountMeta::new(ctx.accounts.amm_market_asks.key(), false));
        accounts.push(AccountMeta::new(ctx.accounts.amm_market_event_queue.key(), false));
        accounts.push(AccountMeta::new(ctx.accounts.amm_market_base_vault.key(), false));
        accounts.push(AccountMeta::new(ctx.accounts.amm_market_quote_vault.key(), false));
        accounts.push(AccountMeta::new_readonly(ctx.accounts.amm_market_authority.key(), false));
        accounts.push(AccountMeta::new(ctx.accounts.source_ata.key(), false));
        accounts.push(AccountMeta::new(ctx.accounts.dest_ata.key(), false));
        accounts.push(AccountMeta::new_readonly(ctx.accounts.signer.key(), true));

        let ix: Instruction = Instruction::new_with_bytes(ctx.accounts.amm_program.key(), data.as_slice(), accounts);
        anchor_lang::solana_program::program::invoke(&ix, &[ctx.accounts.signer.to_account_info()])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Swap<'info> {
    pub token_program: SystemAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    #[account(mut)]
    pub amm_id: UncheckedAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    #[account(mut)]
    pub amm_target_orders: UncheckedAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    #[account(mut)]
    pub amm_base_vault: UncheckedAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    #[account(mut)]
    pub amm_quote_vault: UncheckedAccount<'info>,
    pub amm_market_program_id: SystemAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    #[account(mut)]
    pub amm_market_id: UncheckedAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    #[account(mut)]
    pub amm_market_bids: UncheckedAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    #[account(mut)]
    pub amm_market_asks: UncheckedAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    #[account(mut)]
    pub amm_market_event_queue: UncheckedAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    #[account(mut)]
    pub amm_market_base_vault: UncheckedAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    #[account(mut)]
    pub amm_market_quote_vault: UncheckedAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    pub amm_market_authority: UncheckedAccount<'info>,
    /// CHECK: This will get checked on raydium anyways
    #[account(mut)]
    pub source_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dest_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub amm_program: SystemAccount<'info>
}
