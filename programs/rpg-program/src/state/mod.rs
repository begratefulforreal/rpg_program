pub mod player;
pub mod hero;
pub mod battle;
pub mod roll_session;
pub mod enemy_template;
pub mod treasury;
pub mod item;

pub use player::*;
pub use hero::*;
pub use battle::*;
pub use roll_session::*;
pub use enemy_template::*;
pub use treasury::*;
pub use item::*;

use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
pub struct Attributes {
    pub strength: u16,     // Physical damage
    pub dexterity: u16,    // Attack speed, dodge
    pub vitality: u16,     // HP, defense
    pub intelligence: u16, // Magic damage
    pub wisdom: u16,       // Magic defense, mana
    pub agility: u16,      // Evasion chance
    pub precision: u16,    // Critical hit chance
    pub luck: u16,         // Loot quality, rare drops
}

impl Attributes {
    pub fn new_base() -> Self {
        Self {
            strength: crate::constants::BASE_ATTRIBUTE_POINTS,
            dexterity: crate::constants::BASE_ATTRIBUTE_POINTS,
            vitality: crate::constants::BASE_ATTRIBUTE_POINTS,
            intelligence: crate::constants::BASE_ATTRIBUTE_POINTS,
            wisdom: crate::constants::BASE_ATTRIBUTE_POINTS,
            agility: crate::constants::BASE_ATTRIBUTE_POINTS,
            precision: crate::constants::BASE_ATTRIBUTE_POINTS,
            luck: crate::constants::BASE_ATTRIBUTE_POINTS,
        }
    }

    pub fn add(&mut self, other: &Attributes) {
        self.strength = self.strength.saturating_add(other.strength);
        self.dexterity = self.dexterity.saturating_add(other.dexterity);
        self.vitality = self.vitality.saturating_add(other.vitality);
        self.intelligence = self.intelligence.saturating_add(other.intelligence);
        self.wisdom = self.wisdom.saturating_add(other.wisdom);
        self.agility = self.agility.saturating_add(other.agility);
        self.precision = self.precision.saturating_add(other.precision);
        self.luck = self.luck.saturating_add(other.luck);
    }

    pub fn total(&self) -> u32 {
        self.strength as u32
            + self.dexterity as u32
            + self.vitality as u32
            + self.intelligence as u32
            + self.wisdom as u32
            + self.agility as u32
            + self.precision as u32
            + self.luck as u32
    }
}