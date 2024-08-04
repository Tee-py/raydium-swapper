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

    pub fn swap<'info>(ctx: Context<Swap<'info>>, data: Vec<u8>) -> Result<()> {
        msg!("Started swap...");
        perform_swap(ctx.accounts.into(), data)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts, Clone)]
pub struct Swap<'info> {
    /// CHECK: This will get checked on raydium anyways
    pub token_program: UncheckedAccount<'info>,
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
    /// CHECK: This will get checked on raydium anyways
    pub amm_market_program_id: UncheckedAccount<'info>,
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
    /// CHECK: This will get checked on raydium anyways
    #[account(mut)]
    pub amm_program: UncheckedAccount<'info>,
}

impl<'a, 'b, 'c, 'info> From<&mut Swap<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Swap<'info>>
{
    fn from(
        accounts: &mut Swap<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, Swap<'info>> {
        let cpi_accounts = Swap {
            token_program: accounts.token_program.clone(),
            amm_id: accounts.amm_id.clone(),
            amm_target_orders: accounts.amm_target_orders.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            amm_base_vault: accounts.amm_base_vault.clone(),
            amm_quote_vault: accounts.amm_quote_vault.clone(),
            amm_market_program_id: accounts.amm_market_program_id.clone(),
            amm_market_id: accounts.amm_market_id.clone(),
            amm_market_bids: accounts.amm_market_bids.clone(),
            amm_market_asks: accounts.amm_market_asks.clone(),
            amm_market_event_queue: accounts.amm_market_event_queue.clone(),
            amm_market_base_vault: accounts.amm_market_base_vault.clone(),
            amm_market_quote_vault: accounts.amm_market_quote_vault.clone(),
            amm_market_authority: accounts.amm_market_authority.clone(),
            amm_program: accounts.amm_program.clone(),
            source_ata: accounts.source_ata.clone(),
            dest_ata: accounts.dest_ata.clone(),
            signer: accounts.signer.clone()
        };
        let cpi_program = accounts.amm_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

fn perform_swap<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Swap<'info>>, 
    data: Vec<u8>
) -> Result<()> {
    let mut accounts: Vec<AccountMeta> = Vec::with_capacity(16);

    accounts.push(AccountMeta::new_readonly(ctx.accounts.token_program.key(), false));
    accounts.push(AccountMeta::new(ctx.accounts.amm_id.key(), false));
    accounts.push(AccountMeta::new_readonly(ctx.accounts.amm_authority.key(), false));
    accounts.push(AccountMeta::new(ctx.accounts.amm_open_orders.key(), false));
    accounts.push(AccountMeta::new(ctx.accounts.amm_target_orders.key(), false));
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

    let (_, _) = data.split_first().unwrap();
    let mut ix_data = vec![9];
    ix_data.extend(ctx.accounts.source_ata.amount.to_le_bytes());
    ix_data.extend(0_u64.to_le_bytes());
    let ix: Instruction = Instruction {
        program_id: ctx.accounts.amm_program.key(), 
        data: ix_data.clone(), 
        accounts
    };
    msg!("About to invoke raydium swap instruction");
    msg!("Instruction Data: {:?}", ix_data);
    anchor_lang::solana_program::program::invoke_signed(
        &ix, 
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds
    )?;
    Ok(())
}
