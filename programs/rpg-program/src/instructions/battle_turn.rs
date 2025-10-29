use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::slot_hashes;
use crate::constants::*;
use crate::errors::RpgError;
use crate::state::*;
use crate::utils::validation::*;
use crate::utils::rng::*;
use crate::utils::math::*;

#[derive(Accounts)]
pub struct BattleTurn<'info> {
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
        constraint = battle.is_active() @ RpgError::BattleAlreadyCompleted
    )]
    pub battle: Account<'info, Battle>,

    /// CHECK: Checked manually for slot hashes sysvar
    pub slot_hashes: AccountInfo<'info>,
}

pub fn battle_turn(ctx: Context<BattleTurn>, action: u8) -> Result<()> {
    let clock = Clock::get()?;
    let hero = &ctx.accounts.hero;
    let enemy_template = &ctx.accounts.enemy_template;
    let battle = &mut ctx.accounts.battle;

    // Validate action
    validate_battle_action(action)?;

    // Check if battle participants are alive
    require!(battle.is_hero_alive(), RpgError::HeroIsDead);
    require!(battle.is_enemy_alive(), RpgError::EnemyIsDead);

    // For the first turn, we need to establish randomness using slot hashes
    if battle.turn == 0 {
        // Validate randomness delay has passed
        validate_randomness_delay(clock.slot, battle.commit_slot)?;

        let reveal_slot = battle.commit_slot + MIN_RANDOMNESS_DELAY_SLOTS;

        // Validate slot hashes sysvar
        let sysvar_slot_history = &ctx.accounts.slot_hashes;
        require!(
            sysvar_slot_history.key == &slot_hashes::id(),
            RpgError::InvalidSlotHashes
        );

        // Extract slot hash for randomness
        let data = sysvar_slot_history.try_borrow_data()?;
        let num_slot_hashes = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let mut pos = 8;
        let mut found_hash = None;

        for _ in 0..num_slot_hashes {
            let slot = u64::from_le_bytes(data[pos..pos + 8].try_into().unwrap());
            pos += 8;
            let hash = &data[pos..pos + 32];
            if slot == reveal_slot {
                found_hash = Some(hash);
                break;
            }
            pos += 32;
        }

        let random_seed = found_hash.ok_or(RpgError::SlotNotFound)?;
        
        // Set the battle's RNG seed from the slot hash
        battle.rng_seed.copy_from_slice(random_seed);
    }

    // Get hero and enemy attributes
    let hero_attrs = hero.calculate_total_attributes();
    let enemy_attrs = enemy_template.base_attributes;

    // Determine turn order (higher agility goes first)
    let hero_goes_first = calculate_battle_priority(hero_attrs.agility, enemy_attrs.agility);

    let mut hero_damage = 0u32;
    let mut enemy_damage = 0u32;

    if hero_goes_first {
        // Hero acts first
        if let Some(damage) = execute_hero_action(action, &hero_attrs, &enemy_attrs, battle)? {
            hero_damage = damage;
            battle.damage_enemy(damage);
        }

        // Enemy acts if still alive
        if battle.is_enemy_alive() {
            enemy_damage = execute_enemy_action(enemy_template, &enemy_attrs, &hero_attrs, battle)?;
            battle.damage_hero(enemy_damage);
        }
    } else {
        // Enemy acts first
        enemy_damage = execute_enemy_action(enemy_template, &enemy_attrs, &hero_attrs, battle)?;
        battle.damage_hero(enemy_damage);

        // Hero acts if still alive
        if battle.is_hero_alive() {
            if let Some(damage) = execute_hero_action(action, &hero_attrs, &enemy_attrs, battle)? {
                hero_damage = damage;
                battle.damage_enemy(damage);
            }
        }
    }

    // Increment turn counter
    battle.next_turn();

    emit!(BattleTurnExecuted {
        battle: battle.key(),
        turn: battle.turn,
        hero_action: action,
        hero_damage,
        enemy_damage,
        hero_hp: battle.hero_hp,
        enemy_hp: battle.enemy_hp,
        battle_state: battle.state,
    });

    Ok(())
}

fn execute_hero_action(
    action: u8,
    hero_attrs: &Attributes,
    enemy_attrs: &Attributes,
    battle: &mut Battle,
) -> Result<Option<u32>> {
    match action {
        ACTION_ATTACK => {
            let damage = calculate_damage(hero_attrs.strength, enemy_attrs.vitality);
            
            // Check for critical hit
            let crit_chance = (hero_attrs.precision as u32) * 100; // Convert to basis points
            if rng_check(&battle.rng_seed, battle.turn, crit_chance as u16) {
                let crit_damage = calculate_critical_damage(damage);
                Ok(Some(crit_damage))
            } else {
                Ok(Some(damage))
            }
        }
        ACTION_DEFEND => {
            // Defending reduces incoming damage by 50% this turn
            // This is handled in the enemy attack calculation
            Ok(None)
        }
        ACTION_SKILL => {
            // Magic attack using intelligence
            let damage = calculate_damage(hero_attrs.intelligence, enemy_attrs.wisdom);
            Ok(Some(damage))
        }
        ACTION_ESCAPE => {
            let escape_chance = calculate_escape_chance(hero_attrs.agility, enemy_attrs.agility);
            if rng_check(&battle.rng_seed, battle.turn, escape_chance) {
                battle.escape_battle();
            }
            Ok(None)
        }
        _ => Err(RpgError::InvalidBattleAction.into()),
    }
}

fn execute_enemy_action(
    enemy_template: &EnemyTemplate,
    enemy_attrs: &Attributes,
    hero_attrs: &Attributes,
    battle: &mut Battle,
) -> Result<u32> {
    // Simple AI: aggressive enemies always attack, defensive enemies defend if low HP
    let damage = if enemy_template.is_aggressive() {
        calculate_damage(enemy_attrs.strength, hero_attrs.vitality)
    } else if enemy_template.is_defensive() && battle.enemy_hp < battle.enemy_max_hp / 3 {
        // Defend when low on HP
        calculate_damage(enemy_attrs.strength, hero_attrs.vitality) / 2
    } else {
        // Balanced or normal attack
        calculate_damage(enemy_attrs.strength, hero_attrs.vitality)
    };

    Ok(damage)
}

#[event]
pub struct BattleTurnExecuted {
    pub battle: Pubkey,
    pub turn: u8,
    pub hero_action: u8,
    pub hero_damage: u32,
    pub enemy_damage: u32,
    pub hero_hp: u32,
    pub enemy_hp: u32,
    pub battle_state: u8,
}