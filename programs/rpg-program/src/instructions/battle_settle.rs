use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::RpgError;
use crate::state::*;

#[derive(Accounts)]
pub struct BattleSettle<'info> {
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
        mut,
        seeds = [BATTLE_SEED, hero.key().as_ref(), &battle.nonce.to_le_bytes()],
        bump = battle.bump,
        constraint = battle.hero == hero.key() @ RpgError::BattleNotFound,
        constraint = battle.enemy_template == enemy_template.key() @ RpgError::BattleNotFound,
        constraint = !battle.is_active() @ RpgError::InvalidBattleAction
    )]
    pub battle: Account<'info, Battle>,
}

pub fn battle_settle(ctx: Context<BattleSettle>) -> Result<()> {
    let player = &mut ctx.accounts.player;
    let hero = &mut ctx.accounts.hero;
    let enemy_template = &ctx.accounts.enemy_template;
    let battle = &ctx.accounts.battle;

    let mut xp_gained = 0u32;
    let victory = battle.state == BATTLE_STATE_HERO_WON;

    if victory {
        // Hero won - award XP
        xp_gained = enemy_template.xp_reward;
        hero.add_xp(xp_gained);
    }

    // Update battle statistics
    hero.add_battle_stats(victory);
    player.add_battle_stats(victory, xp_gained as u64);

    emit!(BattleSettled {
        player: player.key(),
        hero: hero.key(),
        enemy_template: enemy_template.key(),
        battle: battle.key(),
        victory,
        xp_gained,
        hero_hp_remaining: battle.hero_hp,
        turns_taken: battle.turn,
    });

    Ok(())
}

#[event]
pub struct BattleSettled {
    pub player: Pubkey,
    pub hero: Pubkey,
    pub enemy_template: Pubkey,
    pub battle: Pubkey,
    pub victory: bool,
    pub xp_gained: u32,
    pub hero_hp_remaining: u32,
    pub turns_taken: u8,
}