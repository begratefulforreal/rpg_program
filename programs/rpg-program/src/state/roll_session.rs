use anchor_lang::prelude::*;

#[account]
pub struct RollSession {
    pub hero: Pubkey,
    pub player: Pubkey,
    pub nonce: u64,
    pub commit_slot: u64,
    pub state: u8,
    pub reward_rarity: Option<u8>,
    pub reward_xp: Option<u32>,
    pub created_at: i64,
    pub bump: u8,
}

impl RollSession {
    pub const LEN: usize = 8 + // discriminator
        32 + // hero
        32 + // player
        8 +  // nonce
        8 +  // commit_slot
        1 +  // state
        (1 + 1) + // reward_rarity (Option<u8>)
        (1 + 4) + // reward_xp (Option<u32>)
        8 +  // created_at
        1;   // bump

    pub fn is_pending(&self) -> bool {
        self.state == crate::constants::ROLL_STATE_PENDING
    }

    pub fn is_completed(&self) -> bool {
        self.state == crate::constants::ROLL_STATE_COMPLETED
    }

    pub fn complete_roll(&mut self, rarity: u8, xp: u32) {
        self.state = crate::constants::ROLL_STATE_COMPLETED;
        self.reward_rarity = Some(rarity);
        self.reward_xp = Some(xp);
    }
}