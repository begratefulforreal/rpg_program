use anchor_lang::prelude::*;
use crate::state::Attributes;
use crate::constants::*;

#[account]
pub struct Hero {
    pub owner: Pubkey,
    pub player: Pubkey,
    pub index: u8,
    pub level: u16,
    pub xp: u32,
    pub base_attributes: Attributes,
    pub equipped_weapon: Option<Pubkey>,
    pub equipped_armor: Option<Pubkey>,
    pub equipped_accessory: Option<Pubkey>,
    pub total_battles: u32,
    pub total_victories: u32,
    pub created_at: i64,
    pub bump: u8,
}

impl Hero {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        32 + // player
        1 +  // index
        2 +  // level
        4 +  // xp
        (2 * 8) + // base_attributes (8 u16s)
        (1 + 32) + // equipped_weapon (Option<Pubkey>)
        (1 + 32) + // equipped_armor (Option<Pubkey>)
        (1 + 32) + // equipped_accessory (Option<Pubkey>)
        4 +  // total_battles
        4 +  // total_victories
        8 +  // created_at
        1;   // bump

    pub fn xp_required_for_next_level(&self) -> u32 {
        XP_PER_LEVEL_BASE * (self.level as u32 + 1)
    }

    pub fn can_level_up(&self) -> bool {
        self.level < MAX_LEVEL && self.xp >= self.xp_required_for_next_level()
    }

    pub fn level_up(&mut self) -> Result<u16> {
        if !self.can_level_up() {
            return Err(crate::errors::RpgError::InsufficientXp.into());
        }

        self.xp = self.xp.saturating_sub(self.xp_required_for_next_level());
        self.level = self.level.saturating_add(1);
        
        Ok(ATTRIBUTE_POINTS_PER_LEVEL)
    }

    pub fn add_xp(&mut self, amount: u32) {
        self.xp = self.xp.saturating_add(amount);
    }

    pub fn calculate_hp(&self) -> u32 {
        (self.base_attributes.vitality as u32) * (BASE_HP_MULTIPLIER as u32)
    }

    pub fn calculate_total_attributes(&self) -> Attributes {
        // Start with base attributes
        let total = self.base_attributes;
        
        // TODO: Add equipment bonuses here when items are implemented
        // if let Some(weapon) = self.equipped_weapon { ... }
        // if let Some(armor) = self.equipped_armor { ... }
        // if let Some(accessory) = self.equipped_accessory { ... }
        
        total
    }

    pub fn add_battle_stats(&mut self, victory: bool) {
        self.total_battles = self.total_battles.saturating_add(1);
        if victory {
            self.total_victories = self.total_victories.saturating_add(1);
        }
    }

    pub fn equip_item(&mut self, item_type: u8, item_key: Pubkey) -> Result<()> {
        match item_type {
            ITEM_TYPE_WEAPON => {
                self.equipped_weapon = Some(item_key);
            }
            ITEM_TYPE_ARMOR => {
                self.equipped_armor = Some(item_key);
            }
            ITEM_TYPE_ACCESSORY => {
                self.equipped_accessory = Some(item_key);
            }
            _ => return Err(crate::errors::RpgError::WrongItemType.into()),
        }
        Ok(())
    }

    pub fn unequip_item(&mut self, item_type: u8) -> Result<Option<Pubkey>> {
        let removed = match item_type {
            ITEM_TYPE_WEAPON => {
                let old = self.equipped_weapon;
                self.equipped_weapon = None;
                old
            }
            ITEM_TYPE_ARMOR => {
                let old = self.equipped_armor;
                self.equipped_armor = None;
                old
            }
            ITEM_TYPE_ACCESSORY => {
                let old = self.equipped_accessory;
                self.equipped_accessory = None;
                old
            }
            _ => return Err(crate::errors::RpgError::WrongItemType.into()),
        };
        Ok(removed)
    }
}