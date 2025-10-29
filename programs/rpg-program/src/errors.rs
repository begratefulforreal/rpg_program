use anchor_lang::prelude::*;

#[error_code]
pub enum RpgError {
    #[msg("Player already initialized")]
    PlayerAlreadyInitialized,

    #[msg("Hero not found")]
    HeroNotFound,

    #[msg("Hero index out of bounds")]
    HeroIndexOutOfBounds,

    #[msg("Maximum heroes reached")]
    MaxHeroesReached,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Insufficient XP")]
    InsufficientXp,

    #[msg("Hero already at max level")]
    HeroAtMaxLevel,

    #[msg("Battle not found")]
    BattleNotFound,

    #[msg("Battle already completed")]
    BattleAlreadyCompleted,

    #[msg("Invalid battle action")]
    InvalidBattleAction,

    #[msg("Roll session not found")]
    RollSessionNotFound,

    #[msg("Roll already completed")]
    RollAlreadyCompleted,

    #[msg("Randomness not resolved yet")]
    RandomnessNotResolved,

    #[msg("Invalid slot hashes sysvar")]
    InvalidSlotHashes,

    #[msg("Slot not found in history")]
    SlotNotFound,

    #[msg("Enemy template not found")]
    EnemyTemplateNotFound,

    #[msg("Invalid enemy template")]
    InvalidEnemyTemplate,

    #[msg("Item not found")]
    ItemNotFound,

    #[msg("Item already equipped")]
    ItemAlreadyEquipped,

    #[msg("Wrong item type for slot")]
    WrongItemType,

    #[msg("Invalid item rarity")]
    InvalidItemRarity,

    #[msg("Unauthorized operation")]
    Unauthorized,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,

    #[msg("Invalid treasury state")]
    InvalidTreasuryState,

    #[msg("Cancel timeout not expired")]
    CancelTimeoutNotExpired,

    #[msg("Invalid attribute distribution")]
    InvalidAttributeDistribution,

    #[msg("Hero is dead")]
    HeroIsDead,

    #[msg("Enemy is dead")]
    EnemyIsDead,
}