use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::RpgError;
use crate::state::*;
use crate::utils::validation::*;

#[derive(Accounts)]
pub struct LevelUpHero<'info> {
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
        constraint = hero.player == player.key() @ RpgError::HeroNotFound,
        constraint = hero.can_level_up() @ RpgError::InsufficientXp
    )]
    pub hero: Account<'info, Hero>,
}

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
    let hero = &mut ctx.accounts.hero;

    // Level up the hero (this validates XP and deducts it)
    let available_points = hero.level_up()?;

    // Validate attribute distribution
    validate_attribute_distribution(
        strength,
        dexterity,
        vitality,
        intelligence,
        wisdom,
        agility,
        precision,
        luck,
        available_points,
    )?;

    // Apply attribute increases
    hero.base_attributes.strength = hero.base_attributes.strength.saturating_add(strength);
    hero.base_attributes.dexterity = hero.base_attributes.dexterity.saturating_add(dexterity);
    hero.base_attributes.vitality = hero.base_attributes.vitality.saturating_add(vitality);
    hero.base_attributes.intelligence = hero.base_attributes.intelligence.saturating_add(intelligence);
    hero.base_attributes.wisdom = hero.base_attributes.wisdom.saturating_add(wisdom);
    hero.base_attributes.agility = hero.base_attributes.agility.saturating_add(agility);
    hero.base_attributes.precision = hero.base_attributes.precision.saturating_add(precision);
    hero.base_attributes.luck = hero.base_attributes.luck.saturating_add(luck);

    emit!(HeroLeveledUp {
        player: ctx.accounts.player.key(),
        hero: hero.key(),
        new_level: hero.level,
        attributes_added: AttributeIncrease {
            strength,
            dexterity,
            vitality,
            intelligence,
            wisdom,
            agility,
            precision,
            luck,
        },
    });

    Ok(())
}

#[event]
pub struct HeroLeveledUp {
    pub player: Pubkey,
    pub hero: Pubkey,
    pub new_level: u16,
    pub attributes_added: AttributeIncrease,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AttributeIncrease {
    pub strength: u16,
    pub dexterity: u16,
    pub vitality: u16,
    pub intelligence: u16,
    pub wisdom: u16,
    pub agility: u16,
    pub precision: u16,
    pub luck: u16,
}