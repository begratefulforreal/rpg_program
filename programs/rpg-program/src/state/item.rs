use anchor_lang::prelude::*;
use crate::state::Attributes;

#[account]
pub struct Item {
    pub hero: Pubkey,
    pub item_type: u8, // Weapon, Armor, Accessory
    pub rarity: u8,
    pub name: [u8; 32],
    pub bonus_attributes: Attributes,
    pub created_at: i64,
    pub bump: u8,
}

impl Item {
    pub const LEN: usize = 8 + // discriminator
        32 + // hero
        1 +  // item_type
        1 +  // rarity
        32 + // name
        (2 * 8) + // bonus_attributes (8 u16s)
        8 +  // created_at
        1;   // bump

    pub fn get_name_string(&self) -> String {
        String::from_utf8_lossy(&self.name)
            .trim_end_matches('\0')
            .to_string()
    }

    pub fn is_weapon(&self) -> bool {
        self.item_type == crate::constants::ITEM_TYPE_WEAPON
    }

    pub fn is_armor(&self) -> bool {
        self.item_type == crate::constants::ITEM_TYPE_ARMOR
    }

    pub fn is_accessory(&self) -> bool {
        self.item_type == crate::constants::ITEM_TYPE_ACCESSORY
    }
}