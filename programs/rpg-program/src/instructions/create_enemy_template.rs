use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::RpgError;
use crate::state::*;
use crate::utils::validation::*;

#[derive(Accounts)]
#[instruction(id: u32)]
pub struct CreateEnemyTemplate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = EnemyTemplate::LEN,
        seeds = [ENEMY_TEMPLATE_SEED, &id.to_le_bytes()],
        bump
    )]
    pub enemy_template: Account<'info, EnemyTemplate>,

    pub system_program: Program<'info, System>,
}

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
    let clock = Clock::get()?;
    let enemy_template = &mut ctx.accounts.enemy_template;

    // Validate enemy template data
    validate_enemy_template(level, &name, ai_flags)?;

    enemy_template.id = id;
    enemy_template.name = name;
    enemy_template.level = level;
    enemy_template.base_attributes = Attributes {
        strength,
        dexterity,
        vitality,
        intelligence,
        wisdom,
        agility,
        precision,
        luck,
    };
    enemy_template.ai_flags = ai_flags;
    enemy_template.xp_reward = xp_reward;
    enemy_template.created_at = clock.unix_timestamp;
    enemy_template.bump = ctx.bumps.enemy_template;

    emit!(EnemyTemplateCreated {
        enemy_template: enemy_template.key(),
        id,
        name: enemy_template.get_name_string(),
        level,
        xp_reward,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct EnemyTemplateCreated {
    pub enemy_template: Pubkey,
    pub id: u32,
    pub name: String,
    pub level: u16,
    pub xp_reward: u32,
    pub timestamp: i64,
}