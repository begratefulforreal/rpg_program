use crate::state::Attributes;
use crate::constants::*;

/// Calculate damage with defense reduction
pub fn calculate_damage(attack: u16, defense: u16) -> u32 {
    let base_damage = attack as u32;
    let damage_reduction = (defense as u32) / (BASE_DAMAGE_REDUCTION as u32);
    std::cmp::max(1, base_damage.saturating_sub(damage_reduction))
}

/// Calculate critical hit damage
pub fn calculate_critical_damage(base_damage: u32) -> u32 {
    base_damage.saturating_mul(CRITICAL_MULTIPLIER as u32)
}

/// Calculate escape chance based on agility difference
pub fn calculate_escape_chance(hero_agility: u16, enemy_agility: u16) -> u16 {
    let base_chance = BASE_ESCAPE_CHANCE;
    
    let agility_diff = if hero_agility >= enemy_agility {
        hero_agility - enemy_agility
    } else {
        0 // Can't have negative escape chance bonus
    };
    
    let bonus = (agility_diff as u16).saturating_mul(AGILITY_ESCAPE_BONUS);
    let total_chance = base_chance.saturating_add(bonus);
    
    // Clamp between 10% and 80%
    std::cmp::min(std::cmp::max(total_chance, 1000), 8000)
}

/// Calculate HP from vitality
pub fn calculate_hp(vitality: u16) -> u32 {
    (vitality as u32).saturating_mul(BASE_HP_MULTIPLIER as u32)
}

/// Calculate XP required for a specific level
pub fn calculate_xp_required(level: u16) -> u32 {
    XP_PER_LEVEL_BASE.saturating_mul(level as u32 + 1)
}

/// Calculate effective attributes including equipment bonuses
pub fn calculate_effective_attributes(base: &Attributes, equipment_bonus: Option<&Attributes>) -> Attributes {
    let mut effective = *base;
    
    if let Some(bonus) = equipment_bonus {
        effective.add(bonus);
    }
    
    effective
}

/// Calculate battle priority (higher agility goes first)
pub fn calculate_battle_priority(hero_agility: u16, enemy_agility: u16) -> bool {
    hero_agility >= enemy_agility
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_damage() {
        let damage = calculate_damage(100, 20);
        assert_eq!(damage, 90); // 100 - 20/2 = 90
        
        let min_damage = calculate_damage(5, 20);
        assert_eq!(min_damage, 1); // Always at least 1 damage
    }

    #[test]
    fn test_calculate_critical_damage() {
        let crit_damage = calculate_critical_damage(50);
        assert_eq!(crit_damage, 100); // 50 * 2 = 100
    }

    #[test]
    fn test_calculate_escape_chance() {
        let chance = calculate_escape_chance(20, 10);
        assert!(chance > BASE_ESCAPE_CHANCE); // Should have bonus
        
        let chance_low = calculate_escape_chance(10, 20);
        assert_eq!(chance_low, BASE_ESCAPE_CHANCE); // No bonus for lower agility
    }

    #[test]
    fn test_calculate_hp() {
        let hp = calculate_hp(50);
        assert_eq!(hp, 500); // 50 * 10 = 500
    }

    #[test]
    fn test_calculate_xp_required() {
        let xp = calculate_xp_required(0);
        assert_eq!(xp, 1000); // Level 1 requires 1000 XP
        
        let xp_level_2 = calculate_xp_required(1);
        assert_eq!(xp_level_2, 2000); // Level 2 requires 2000 XP
    }
}