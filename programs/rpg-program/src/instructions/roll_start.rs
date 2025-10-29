use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::constants::*;
use crate::errors::RpgError;
use crate::state::*;
use crate::utils::validation::*;

#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct RollStart<'info> {
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
        init,
        payer = player_authority,
        space = RollSession::LEN,
        seeds = [ROLL_SESSION_SEED, hero.key().as_ref(), &nonce.to_le_bytes()],
        bump
    )]
    pub roll_session: Account<'info, RollSession>,

    #[account(
        mut,
        seeds = [TREASURY_SEED],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}

pub fn roll_start(ctx: Context<RollStart>, nonce: u64) -> Result<()> {
    let clock = Clock::get()?;
    let player = &mut ctx.accounts.player;
    let hero = &ctx.accounts.hero;
    let roll_session = &mut ctx.accounts.roll_session;
    let treasury = &mut ctx.accounts.treasury;

    // Validate sufficient funds
    validate_sufficient_funds(
        ctx.accounts.player_authority.lamports(),
        ROLL_COST,
    )?;

    // Transfer SOL to treasury
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.player_authority.to_account_info(),
                to: treasury.to_account_info(),
            },
        ),
        ROLL_COST,
    )?;

    // Initialize roll session
    roll_session.hero = hero.key();
    roll_session.player = player.key();
    roll_session.nonce = nonce;
    roll_session.commit_slot = clock.slot;
    roll_session.state = ROLL_STATE_PENDING;
    roll_session.reward_rarity = None;
    roll_session.reward_xp = None;
    roll_session.created_at = clock.unix_timestamp;
    roll_session.bump = ctx.bumps.roll_session;

    // Update treasury stats
    treasury.add_revenue(ROLL_COST, crate::state::treasury::RevenueType::Roll);

    // Update player stats
    player.add_sol_spent(ROLL_COST);

    Ok(())
}