use anchor_lang::prelude::*;
use crate::state::Attributes;

#[account]
pub struct EnemyTemplate {
    pub id: u32,
    pub name: [u8; 32],
    pub level: u16,
    pub base_attributes: Attributes,
    pub ai_flags: u8,
    pub xp_reward: u32,
    pub created_at: i64,
    pub bump: u8,
}

impl EnemyTemplate {
    pub const LEN: usize = 8 + // discriminator
        4 +  // id
        32 + // name
        2 +  // level
        (2 * 8) + // base_attributes (8 u16s)
        1 +  // ai_flags
        4 +  // xp_reward
        8 +  // created_at
        1;   // bump

    pub fn calculate_hp(&self) -> u32 {
        (self.base_attributes.vitality as u32) * (crate::constants::BASE_HP_MULTIPLIER as u32)
    }

    pub fn get_name_string(&self) -> String {
        String::from_utf8_lossy(&self.name)
            .trim_end_matches('\0')
            .to_string()
    }

    pub fn is_aggressive(&self) -> bool {
        self.ai_flags & crate::constants::AI_AGGRESSIVE != 0
    }

    pub fn is_defensive(&self) -> bool {
        self.ai_flags & crate::constants::AI_DEFENSIVE != 0
    }

    pub fn is_balanced(&self) -> bool {
        self.ai_flags & crate::constants::AI_BALANCED != 0
    }
}