use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;
pub mod instructions;
pub mod utils;

use errors::RpgError;
use instructions::*;

declare_id!("6cLSu9TdHVztKk2pykJGArnuhmXmrc1agezqVWAK9ubp");

#[program]
pub mod rpg_program {
    use super::*;

    /// Initialize a new player profile
    pub fn player_initialize(ctx: Context<PlayerInitialize>) -> Result<()> {
        instructions::player_initialize(ctx)
    }

    /// Purchase a new hero
    pub fn buy_hero(ctx: Context<BuyHero>, hero_index: u8) -> Result<()> {
        instructions::buy_hero(ctx, hero_index)
    }

    /// Level up a hero with attribute distribution
    pub fn level_up_hero(
        ctx: Context<LevelUpHero>,
        strength: u16,
        dexterity: u16,
        vitality: u16,
        intelligence: u16,
        wisdom: u16,
        agility: u16,
        precision: u16,
        luck: u16,
    ) -> Result<()> {
        instructions::level_up_hero(
            ctx,
            strength,
            dexterity,
            vitality,
            intelligence,
            wisdom,
            agility,
            precision,
            luck,
        )
    }

    /// Start a slot machine roll (commit phase)
    pub fn roll_start(ctx: Context<RollStart>, nonce: u64) -> Result<()> {
        instructions::roll_start(ctx, nonce)
    }

    /// Fulfill a slot machine roll with randomness (reveal phase)
    pub fn roll_fulfill(ctx: Context<RollFulfill>) -> Result<()> {
        instructions::roll_fulfill(ctx)
    }

    /// Start a battle against an enemy
    pub fn battle_start(ctx: Context<BattleStart>, nonce: u64) -> Result<()> {
        instructions::battle_start(ctx, nonce)
    }

    /// Execute a battle turn
    pub fn battle_turn(ctx: Context<BattleTurn>, action: u8) -> Result<()> {
        instructions::battle_turn(ctx, action)
    }

    /// Settle a completed battle
    pub fn battle_settle(ctx: Context<BattleSettle>) -> Result<()> {
        instructions::battle_settle(ctx)
    }

    /// Create a new enemy template (admin only)
    pub fn create_enemy_template(
        ctx: Context<CreateEnemyTemplate>,
        id: u32,
        name: [u8; 32],
        level: u16,
        strength: u16,
        dexterity: u16,
        vitality: u16,
        intelligence: u16,
        wisdom: u16,
        agility: u16,
        precision: u16,
        luck: u16,
        ai_flags: u8,
        xp_reward: u32,
    ) -> Result<()> {
        instructions::create_enemy_template(
            ctx,
            id,
            name,
            level,
            strength,
            dexterity,
            vitality,
            intelligence,
            wisdom,
            agility,
            precision,
            luck,
            ai_flags,
            xp_reward,
        )
    }

    /// Initialize the treasury (admin only)
    pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
        instructions::initialize_treasury(ctx)
    }

    /// Cancel a pending roll that has timed out
    pub fn cancel_pending_roll(ctx: Context<CancelPendingRoll>) -> Result<()> {
        instructions::cancel_pending_roll(ctx)
    }

    /// Cancel a pending battle that has timed out
    pub fn cancel_pending_battle(ctx: Context<CancelPendingBattle>) -> Result<()> {
        instructions::cancel_pending_battle(ctx)
    }
}
