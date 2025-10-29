use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Treasury::LEN,
        seeds = [TREASURY_SEED],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
    let clock = Clock::get()?;
    let treasury = &mut ctx.accounts.treasury;

    treasury.authority = ctx.accounts.authority.key();
    treasury.total_collected = 0;
    treasury.total_battles = 0;
    treasury.total_rolls = 0;
    treasury.total_heroes_created = 0;
    treasury.created_at = clock.unix_timestamp;
    treasury.bump = ctx.bumps.treasury;

    emit!(TreasuryInitialized {
        treasury: treasury.key(),
        authority: treasury.authority,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct TreasuryInitialized {
    pub treasury: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}