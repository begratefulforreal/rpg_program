use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::RpgError;
use crate::state::*;
use crate::utils::validation::*;

#[derive(Accounts)]
pub struct CancelPendingRoll<'info> {
    #[account(mut)]
    pub player_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [PLAYER_SEED, player_authority.key().as_ref()],
        bump = player.bump,
        constraint = player.authority == player_authority.key() @ RpgError::Unauthorized
    )]
    pub player: Account<'info, Player>,

    #[account(
        mut,
        seeds = [HERO_SEED, player.key().as_ref(), &hero.index.to_le_bytes()],
        bump = hero.bump,
        constraint = hero.owner == player_authority.key() @ RpgError::Unauthorized,
        constraint = hero.player == player.key() @ RpgError::HeroNotFound
    )]
    pub hero: Account<'info, Hero>,

    #[account(
        mut,
        close = player_authority,
        seeds = [ROLL_SESSION_SEED, hero.key().as_ref(), &roll_session.nonce.to_le_bytes()],
        bump = roll_session.bump,
        constraint = roll_session.hero == hero.key() @ RpgError::RollSessionNotFound,
        constraint = roll_session.player == player.key() @ RpgError::RollSessionNotFound,
        constraint = roll_session.is_pending() @ RpgError::RollAlreadyCompleted
    )]
    pub roll_session: Account<'info, RollSession>,
}

pub fn cancel_pending_roll(ctx: Context<CancelPendingRoll>) -> Result<()> {
    let clock = Clock::get()?;
    let roll_session = &ctx.accounts.roll_session;

    // Validate cancel timeout has passed
    validate_cancel_timeout(clock.slot, roll_session.commit_slot)?;

    // Roll session will be closed automatically by the close constraint
    
    emit!(PendingRollCanceled {
        player: ctx.accounts.player.key(),
        hero: ctx.accounts.hero.key(),
        roll_session: roll_session.key(),
        slot: clock.slot,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct CancelPendingBattle<'info> {
    #[account(mut)]
    pub player_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [PLAYER_SEED, player_authority.key().as_ref()],
        bump = player.bump,
        constraint = player.authority == player_authority.key() @ RpgError::Unauthorized
    )]
    pub player: Account<'info, Player>,

    #[account(
        mut,
        seeds = [HERO_SEED, player.key().as_ref(), &hero.index.to_le_bytes()],
        bump = hero.bump,
        constraint = hero.owner == player_authority.key() @ RpgError::Unauthorized,
        constraint = hero.player == player.key() @ RpgError::HeroNotFound
    )]
    pub hero: Account<'info, Hero>,

    #[account(
        mut,
        close = player_authority,
        seeds = [BATTLE_SEED, hero.key().as_ref(), &battle.nonce.to_le_bytes()],
        bump = battle.bump,
        constraint = battle.hero == hero.key() @ RpgError::BattleNotFound,
        constraint = battle.turn == 0 @ RpgError::BattleAlreadyCompleted  // Can only cancel before first turn
    )]
    pub battle: Account<'info, Battle>,
}

pub fn cancel_pending_battle(ctx: Context<CancelPendingBattle>) -> Result<()> {
    let clock = Clock::get()?;
    let battle = &ctx.accounts.battle;

    // Validate cancel timeout has passed
    validate_cancel_timeout(clock.slot, battle.commit_slot)?;

    // Battle account will be closed automatically by the close constraint
    
    emit!(PendingBattleCanceled {
        player: ctx.accounts.player.key(),
        hero: ctx.accounts.hero.key(),
        battle: battle.key(),
        slot: clock.slot,
    });

    Ok(())
}

#[event]
pub struct PendingRollCanceled {
    pub player: Pubkey,
    pub hero: Pubkey,
    pub roll_session: Pubkey,
    pub slot: u64,
}

#[event]
pub struct PendingBattleCanceled {
    pub player: Pubkey,
    pub hero: Pubkey,
    pub battle: Pubkey,
    pub slot: u64,
}