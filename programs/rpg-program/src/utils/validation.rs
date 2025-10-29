use anchor_lang::prelude::*;
use crate::errors::RpgError;
use crate::constants::*;

/// Validate hero index is within bounds
pub fn validate_hero_index(index: u8, max_count: u8) -> Result<()> {
    require!(index < max_count, RpgError::HeroIndexOutOfBounds);
    Ok(())
}

/// Validate attribute distribution for level up
pub fn validate_attribute_distribution(
    strength: u16,
    dexterity: u16,
    vitality: u16,
    intelligence: u16,
    wisdom: u16,
    agility: u16,
    precision: u16,
    luck: u16,
    available_points: u16,
) -> Result<()> {
    let total_spent = strength + dexterity + vitality + intelligence + wisdom + agility + precision + luck;
    
    require!(
        total_spent == available_points,
        RpgError::InvalidAttributeDistribution
    );
    
    Ok(())
}

/// Validate battle action is valid
pub fn validate_battle_action(action: u8) -> Result<()> {
    require!(
        action <= ACTION_ESCAPE,
        RpgError::InvalidBattleAction
    );
    Ok(())
}

/// Validate enemy template data
pub fn validate_enemy_template(
    level: u16,
    name: &[u8; 32],
    ai_flags: u8,
) -> Result<()> {
    require!(level > 0 && level <= MAX_LEVEL, RpgError::InvalidEnemyTemplate);
    
    // Ensure name is not empty
    let name_str = String::from_utf8_lossy(name);
    require!(!name_str.trim_matches('\0').is_empty(), RpgError::InvalidEnemyTemplate);
    
    // Validate AI flags
    require!(
        ai_flags == AI_AGGRESSIVE || ai_flags == AI_DEFENSIVE || ai_flags == AI_BALANCED,
        RpgError::InvalidEnemyTemplate
    );
    
    Ok(())
}

/// Validate item data
pub fn validate_item_data(
    item_type: u8,
    rarity: u8,
    name: &[u8; 32],
) -> Result<()> {
    require!(
        item_type <= ITEM_TYPE_ACCESSORY,
        RpgError::WrongItemType
    );
    
    require!(
        rarity <= RARITY_MYTHIC,
        RpgError::InvalidItemRarity
    );
    
    // Ensure name is not empty
    let name_str = String::from_utf8_lossy(name);
    require!(!name_str.trim_matches('\0').is_empty(), RpgError::ItemNotFound);
    
    Ok(())
}

/// Validate sufficient funds for operation
pub fn validate_sufficient_funds(available: u64, required: u64) -> Result<()> {
    require!(available >= required, RpgError::InsufficientFunds);
    Ok(())
}

/// Validate randomness delay has passed
pub fn validate_randomness_delay(current_slot: u64, commit_slot: u64) -> Result<()> {
    require!(
        current_slot >= commit_slot + MIN_RANDOMNESS_DELAY_SLOTS,
        RpgError::RandomnessNotResolved
    );
    Ok(())
}

/// Validate cancel timeout has passed
pub fn validate_cancel_timeout(current_slot: u64, commit_slot: u64) -> Result<()> {
    require!(
        current_slot > commit_slot + CANCEL_TIMEOUT_SLOTS,
        RpgError::CancelTimeoutNotExpired
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_hero_index() {
        assert!(validate_hero_index(0, 5).is_ok());
        assert!(validate_hero_index(4, 5).is_ok());
        assert!(validate_hero_index(5, 5).is_err());
    }

    #[test]
    fn test_validate_attribute_distribution() {
        // Valid distribution
        assert!(validate_attribute_distribution(1, 1, 1, 1, 1, 0, 0, 0, 5).is_ok());
        
        // Invalid distribution (too many points)
        assert!(validate_attribute_distribution(2, 2, 2, 2, 2, 0, 0, 0, 5).is_err());
        
        // Invalid distribution (too few points)
        assert!(validate_attribute_distribution(1, 1, 1, 0, 0, 0, 0, 0, 5).is_err());
    }

    #[test]
    fn test_validate_battle_action() {
        assert!(validate_battle_action(ACTION_ATTACK).is_ok());
        assert!(validate_battle_action(ACTION_ESCAPE).is_ok());
        assert!(validate_battle_action(99).is_err());
    }

    #[test]
    fn test_validate_sufficient_funds() {
        assert!(validate_sufficient_funds(1000, 500).is_ok());
        assert!(validate_sufficient_funds(500, 1000).is_err());
        assert!(validate_sufficient_funds(1000, 1000).is_ok());
    }
}