use anchor_lang::prelude::*;

#[account]
pub struct Treasury {
    pub authority: Pubkey,
    pub total_collected: u64,
    pub total_battles: u64,
    pub total_rolls: u64,
    pub total_heroes_created: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl Treasury {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        8 +  // total_collected
        8 +  // total_battles
        8 +  // total_rolls
        8 +  // total_heroes_created
        8 +  // created_at
        1;   // bump

    pub fn add_revenue(&mut self, amount: u64, revenue_type: RevenueType) {
        self.total_collected = self.total_collected.saturating_add(amount);
        
        match revenue_type {
            RevenueType::Battle => {
                self.total_battles = self.total_battles.saturating_add(1);
            }
            RevenueType::Roll => {
                self.total_rolls = self.total_rolls.saturating_add(1);
            }
            RevenueType::Hero => {
                self.total_heroes_created = self.total_heroes_created.saturating_add(1);
            }
        }
    }
}

pub enum RevenueType {
    Battle,
    Roll,
    Hero,
}