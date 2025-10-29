use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::RpgError;
use crate::state::*;

#[derive(Accounts)]
pub struct PlayerInitialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Player::LEN,
        seeds = [PLAYER_SEED, authority.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,

    pub system_program: Program<'info, System>,
}

pub fn player_initialize(ctx: Context<PlayerInitialize>) -> Result<()> {
    let clock = Clock::get()?;
    let player = &mut ctx.accounts.player;

    player.authority = ctx.accounts.authority.key();
    player.hero_count = 0;
    player.total_battles = 0;
    player.total_victories = 0;
    player.total_xp_earned = 0;
    player.total_sol_spent = 0;
    player.created_at = clock.unix_timestamp;
    player.bump = ctx.bumps.player;

    emit!(PlayerInitialized {
        player: player.key(),
        authority: player.authority,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct PlayerInitialized {
    pub player: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}