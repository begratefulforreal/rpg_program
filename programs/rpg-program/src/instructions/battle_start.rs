use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::constants::*;
use crate::errors::RpgError;
use crate::state::*;
use crate::utils::validation::*;
use crate::utils::math::*;

#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct BattleStart<'info> {
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
        seeds = [ENEMY_TEMPLATE_SEED, &enemy_template.id.to_le_bytes()],
        bump = enemy_template.bump
    )]
    pub enemy_template: Account<'info, EnemyTemplate>,

    #[account(
        init,
        payer = player_authority,
        space = Battle::LEN,
        seeds = [BATTLE_SEED, hero.key().as_ref(), &nonce.to_le_bytes()],
        bump
    )]
    pub battle: Account<'info, Battle>,

    #[account(
        mut,
        seeds = [TREASURY_SEED],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}

pub fn battle_start(ctx: Context<BattleStart>, nonce: u64) -> Result<()> {
    let clock = Clock::get()?;
    let player = &mut ctx.accounts.player;
    let hero = &ctx.accounts.hero;
    let enemy_template = &ctx.accounts.enemy_template;
    let battle = &mut ctx.accounts.battle;
    let treasury = &mut ctx.accounts.treasury;

    // Validate sufficient funds
    validate_sufficient_funds(
        ctx.accounts.player_authority.lamports(),
        BATTLE_ENTRY_COST,
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
        BATTLE_ENTRY_COST,
    )?;

    // Calculate starting HP for hero and enemy
    let hero_max_hp = hero.calculate_hp();
    let enemy_max_hp = enemy_template.calculate_hp();

    // Initialize empty seed - will be filled when randomness is revealed
    let rng_seed = [0u8; 32];

    // Initialize battle
    battle.hero = hero.key();
    battle.enemy_template = enemy_template.key();
    battle.hero_hp = hero_max_hp;
    battle.enemy_hp = enemy_max_hp;
    battle.hero_max_hp = hero_max_hp;
    battle.enemy_max_hp = enemy_max_hp;
    battle.rng_seed = rng_seed;
    battle.turn = 0;
    battle.state = BATTLE_STATE_ACTIVE;
    battle.nonce = nonce;
    battle.commit_slot = clock.slot;
    battle.created_at = clock.unix_timestamp;
    battle.bump = ctx.bumps.battle;

    // Update treasury stats
    treasury.add_revenue(BATTLE_ENTRY_COST, crate::state::treasury::RevenueType::Battle);

    // Update player stats
    player.add_sol_spent(BATTLE_ENTRY_COST);

    emit!(BattleStarted {
        player: player.key(),
        hero: hero.key(),
        enemy_template: enemy_template.key(),
        battle: battle.key(),
        hero_hp: hero_max_hp,
        enemy_hp: enemy_max_hp,
        slot: clock.slot,
    });

    Ok(())
}

#[event]
pub struct BattleStarted {
    pub player: Pubkey,
    pub hero: Pubkey,
    pub enemy_template: Pubkey,
    pub battle: Pubkey,
    pub hero_hp: u32,
    pub enemy_hp: u32,
    pub slot: u64,
}