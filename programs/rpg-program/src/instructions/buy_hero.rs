use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::constants::*;
use crate::errors::RpgError;
use crate::state::*;
use crate::utils::validation::*;

#[derive(Accounts)]
#[instruction(hero_index: u8)]
pub struct BuyHero<'info> {
    #[account(mut)]
    pub player_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [PLAYER_SEED, player_authority.key().as_ref()],
        bump = player.bump,
        constraint = player.authority == player_authority.key() @ RpgError::Unauthorized,
        constraint = player.can_add_hero() @ RpgError::MaxHeroesReached
    )]
    pub player: Account<'info, Player>,

    #[account(
        init,
        payer = player_authority,
        space = Hero::LEN,
        seeds = [HERO_SEED, player.key().as_ref(), &hero_index.to_le_bytes()],
        bump
    )]
    pub hero: Account<'info, Hero>,

    #[account(
        mut,
        seeds = [TREASURY_SEED],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}

pub fn buy_hero(ctx: Context<BuyHero>, hero_index: u8) -> Result<()> {
    let clock = Clock::get()?;
    let player = &mut ctx.accounts.player;
    let hero = &mut ctx.accounts.hero;
    let treasury = &mut ctx.accounts.treasury;

    // Validate sufficient funds
    validate_sufficient_funds(
        ctx.accounts.player_authority.lamports(),
        HERO_PURCHASE_COST,
    )?;

    // Validate hero index matches player's next hero
    require!(
        hero_index == player.hero_count,
        RpgError::HeroIndexOutOfBounds
    );

    // Transfer SOL to treasury
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.player_authority.to_account_info(),
                to: treasury.to_account_info(),
            },
        ),
        HERO_PURCHASE_COST,
    )?;

    // Initialize hero with base attributes
    hero.owner = ctx.accounts.player_authority.key();
    hero.player = player.key();
    hero.index = hero_index;
    hero.level = 1;
    hero.xp = 0;
    hero.base_attributes = Attributes::new_base();
    hero.equipped_weapon = None;
    hero.equipped_armor = None;
    hero.equipped_accessory = None;
    hero.total_battles = 0;
    hero.total_victories = 0;
    hero.created_at = clock.unix_timestamp;
    hero.bump = ctx.bumps.hero;

    // Update player stats
    player.add_hero();
    player.add_sol_spent(HERO_PURCHASE_COST);

    // Update treasury stats
    treasury.add_revenue(HERO_PURCHASE_COST, crate::state::treasury::RevenueType::Hero);

    emit!(HeroPurchased {
        player: player.key(),
        hero: hero.key(),
        hero_index,
        cost: HERO_PURCHASE_COST,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct HeroPurchased {
    pub player: Pubkey,
    pub hero: Pubkey,
    pub hero_index: u8,
    pub cost: u64,
    pub timestamp: i64,
}