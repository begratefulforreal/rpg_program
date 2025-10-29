use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::slot_hashes;
use crate::constants::*;
use crate::errors::RpgError;
use crate::state::*;
use crate::utils::validation::*;
use crate::utils::rng::*;

#[derive(Accounts)]
pub struct RollFulfill<'info> {
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
        seeds = [ROLL_SESSION_SEED, hero.key().as_ref(), &roll_session.nonce.to_le_bytes()],
        bump = roll_session.bump,
        constraint = roll_session.hero == hero.key() @ RpgError::RollSessionNotFound,
        constraint = roll_session.player == player.key() @ RpgError::RollSessionNotFound,
        constraint = roll_session.is_pending() @ RpgError::RollAlreadyCompleted
    )]
    pub roll_session: Account<'info, RollSession>,

    /// CHECK: Checked manually for slot hashes sysvar
    pub slot_hashes: AccountInfo<'info>,
}

pub fn roll_fulfill(ctx: Context<RollFulfill>) -> Result<()> {
    let clock = Clock::get()?;
    let player = &mut ctx.accounts.player;
    let hero = &mut ctx.accounts.hero;
    let roll_session = &mut ctx.accounts.roll_session;

    // Validate randomness delay
    validate_randomness_delay(clock.slot, roll_session.commit_slot)?;

    let reveal_slot = roll_session.commit_slot + MIN_RANDOMNESS_DELAY_SLOTS;

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

    // Convert slice to array for RNG functions
    let mut seed_array = [0u8; 32];
    seed_array.copy_from_slice(random_seed);

    // Determine reward rarity using ponzimon-style RNG
    let rarity = determine_roll_rarity(&seed_array, 0);

    // Get XP reward based on rarity
    let xp_reward = ROLL_REWARDS[rarity as usize];

    // Award XP to hero
    hero.add_xp(xp_reward);

    // Complete roll session
    roll_session.complete_roll(rarity, xp_reward);

    // Update player stats
    player.add_battle_stats(true, xp_reward as u64); // Count as victory for XP tracking

    emit!(RollCompleted {
        player: player.key(),
        hero: hero.key(),
        rarity,
        xp_reward,
        slot: clock.slot,
    });

    Ok(())
}

#[event]
pub struct RollCompleted {
    pub player: Pubkey,
    pub hero: Pubkey,
    pub rarity: u8,
    pub xp_reward: u32,
    pub slot: u64,
}