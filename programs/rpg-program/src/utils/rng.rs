/// Random number generation utilities using slot hashes
/// Based on ponzimon-program's secure RNG system

/// Generate a pseudo-random u16 from seed and turn counter
pub fn rng_u16(seed: &[u8; 32], turn: u8) -> u16 {
    let mut result = [0u8; 2];
    result[0] = seed[0] ^ seed[16] ^ turn;
    result[1] = seed[1] ^ seed[17] ^ turn.wrapping_mul(3);
    
    u16::from_le_bytes(result)
}

/// Generate a pseudo-random u32 from seed and turn counter
pub fn rng_u32(seed: &[u8; 32], turn: u8) -> u32 {
    let mut result = [0u8; 4];
    result[0] = seed[0] ^ seed[8] ^ turn;
    result[1] = seed[1] ^ seed[9] ^ turn.wrapping_mul(3);
    result[2] = seed[2] ^ seed[10] ^ turn.wrapping_mul(5);
    result[3] = seed[3] ^ seed[11] ^ turn.wrapping_mul(7);
    
    u32::from_le_bytes(result)
}

/// Generate random number in range [min, max]
pub fn rng_range(seed: &[u8; 32], turn: u8, min: u32, max: u32) -> u32 {
    if min >= max {
        return min;
    }
    
    let range = max - min + 1;
    let random = rng_u32(seed, turn);
    min + (random % range)
}

/// Check if random event occurs (percentage out of 10000 for precision)
pub fn rng_check(seed: &[u8; 32], turn: u8, threshold: u16) -> bool {
    let roll = rng_u16(seed, turn) % 10000;
    roll < threshold
}

/// Generate random percentage (0-100)
pub fn rng_percentage(seed: &[u8; 32], turn: u8) -> u8 {
    (rng_u16(seed, turn) % 101) as u8
}

/// Determine roll reward rarity based on random value
pub fn determine_roll_rarity(seed: &[u8; 32], turn: u8) -> u8 {
    let random_percent = (rng_u32(seed, turn) as u64 * 1000 / (u32::MAX as u64 + 1)) as u32;
    
    match random_percent {
        0..=499 => crate::constants::RARITY_COMMON,        // 50.0%
        500..=749 => crate::constants::RARITY_UNCOMMON,    // 25.0%
        750..=899 => crate::constants::RARITY_RARE,        // 15.0%
        900..=959 => crate::constants::RARITY_EPIC,        // 6.0%
        960..=989 => crate::constants::RARITY_LEGENDARY,   // 3.0%
        _ => crate::constants::RARITY_MYTHIC,              // 1.0%
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng_u16() {
        let seed = [1u8; 32];
        let result1 = rng_u16(&seed, 0);
        let result2 = rng_u16(&seed, 1);
        
        // Different turns should give different results
        assert_ne!(result1, result2);
    }

    #[test]
    fn test_rng_range() {
        let seed = [1u8; 32];
        let result = rng_range(&seed, 0, 10, 20);
        
        assert!(result >= 10 && result <= 20);
    }

    #[test]
    fn test_rng_percentage() {
        let seed = [1u8; 32];
        let result = rng_percentage(&seed, 0);
        
        assert!(result <= 100);
    }

    #[test]
    fn test_determine_roll_rarity() {
        let seed = [1u8; 32];
        let rarity = determine_roll_rarity(&seed, 0);
        
        assert!(rarity <= crate::constants::RARITY_MYTHIC);
    }
}