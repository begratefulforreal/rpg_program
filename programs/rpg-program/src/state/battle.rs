use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct Battle {
    pub hero: Pubkey,
    pub enemy_template: Pubkey,
    pub hero_hp: u32,
    pub enemy_hp: u32,
    pub hero_max_hp: u32,
    pub enemy_max_hp: u32,
    pub rng_seed: [u8; 32],
    pub turn: u8,
    pub state: u8,
    pub nonce: u64,
    pub commit_slot: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl Battle {
    pub const LEN: usize = 8 + // discriminator
        32 + // hero
        32 + // enemy_template
        4 +  // hero_hp
        4 +  // enemy_hp
        4 +  // hero_max_hp
        4 +  // enemy_max_hp
        32 + // rng_seed
        1 +  // turn
        1 +  // state
        8 +  // nonce
        8 +  // commit_slot
        8 +  // created_at
        1;   // bump

    pub fn is_active(&self) -> bool {
        self.state == BATTLE_STATE_ACTIVE
    }

    pub fn is_hero_alive(&self) -> bool {
        self.hero_hp > 0
    }

    pub fn is_enemy_alive(&self) -> bool {
        self.enemy_hp > 0
    }

    pub fn set_state(&mut self, new_state: u8) {
        self.state = new_state;
    }

    pub fn damage_hero(&mut self, damage: u32) {
        self.hero_hp = self.hero_hp.saturating_sub(damage);
        if self.hero_hp == 0 {
            self.state = BATTLE_STATE_HERO_LOST;
        }
    }

    pub fn damage_enemy(&mut self, damage: u32) {
        self.enemy_hp = self.enemy_hp.saturating_sub(damage);
        if self.enemy_hp == 0 {
            self.state = BATTLE_STATE_HERO_WON;
        }
    }

    pub fn next_turn(&mut self) {
        self.turn = self.turn.saturating_add(1);
    }

    pub fn escape_battle(&mut self) {
        self.state = BATTLE_STATE_ESCAPED;
    }
}