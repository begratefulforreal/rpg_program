use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct Player {
    pub authority: Pubkey,
    pub hero_count: u8,
    pub total_battles: u64,
    pub total_victories: u64,
    pub total_xp_earned: u64,
    pub total_sol_spent: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl Player {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        1 +  // hero_count
        8 +  // total_battles
        8 +  // total_victories
        8 +  // total_xp_earned
        8 +  // total_sol_spent
        8 +  // created_at
        1;   // bump

    pub fn can_add_hero(&self) -> bool {
        self.hero_count < MAX_HEROES_PER_PLAYER
    }

    pub fn add_hero(&mut self) {
        self.hero_count = self.hero_count.saturating_add(1);
    }

    pub fn add_battle_stats(&mut self, victory: bool, xp_gained: u64) {
        self.total_battles = self.total_battles.saturating_add(1);
        if victory {
            self.total_victories = self.total_victories.saturating_add(1);
        }
        self.total_xp_earned = self.total_xp_earned.saturating_add(xp_gained);
    }

    pub fn add_sol_spent(&mut self, amount: u64) {
        self.total_sol_spent = self.total_sol_spent.saturating_add(amount);
    }
}