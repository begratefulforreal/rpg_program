// Game constants

// PDA seeds
pub const PLAYER_SEED: &[u8] = b"player";
pub const HERO_SEED: &[u8] = b"hero";
pub const BATTLE_SEED: &[u8] = b"battle";
pub const ROLL_SESSION_SEED: &[u8] = b"roll";
pub const ENEMY_TEMPLATE_SEED: &[u8] = b"enemy_template";
pub const TREASURY_SEED: &[u8] = b"treasury";

// Random number generation (matching ponzimon-program)
pub const MIN_RANDOMNESS_DELAY_SLOTS: u64 = 2;
pub const CANCEL_TIMEOUT_SLOTS: u64 = 24;

// Game economics (in lamports)
pub const HERO_PURCHASE_COST: u64 = 1_000_000_000; // 1 SOL
pub const ROLL_COST: u64 = 100_000_000; // 0.1 SOL
pub const BATTLE_ENTRY_COST: u64 = 50_000_000; // 0.05 SOL

// Hero system
pub const BASE_ATTRIBUTE_POINTS: u16 = 10;
pub const ATTRIBUTE_POINTS_PER_LEVEL: u16 = 5;
pub const XP_PER_LEVEL_BASE: u32 = 1000;
pub const MAX_LEVEL: u16 = 100;
pub const MAX_HEROES_PER_PLAYER: u8 = 5;

// Battle system
pub const BASE_HP_MULTIPLIER: u16 = 10; // HP = vitality * multiplier
pub const BASE_DAMAGE_REDUCTION: u16 = 2; // damage = max(1, damage - defense/2)
pub const CRITICAL_MULTIPLIER: u16 = 2;
pub const BASE_ESCAPE_CHANCE: u16 = 3000; // 30% out of 10000
pub const AGILITY_ESCAPE_BONUS: u16 = 100; // 1% per agility point difference

// Roll rewards (XP amounts)
pub const ROLL_REWARDS: [u32; 6] = [
    100,  // Common
    200,  // Uncommon
    500,  // Rare
    1000, // Epic
    2000, // Legendary
    5000, // Mythic
];

// Equipment system
pub const MAX_ITEMS_PER_HERO: u8 = 3; // Weapon, Armor, Accessory

// Enemy AI flags
pub const AI_AGGRESSIVE: u8 = 1;
pub const AI_DEFENSIVE: u8 = 2;
pub const AI_BALANCED: u8 = 3;

// Battle states
pub const BATTLE_STATE_ACTIVE: u8 = 0;
pub const BATTLE_STATE_HERO_WON: u8 = 1;
pub const BATTLE_STATE_HERO_LOST: u8 = 2;
pub const BATTLE_STATE_ESCAPED: u8 = 3;

// Roll states
pub const ROLL_STATE_PENDING: u8 = 0;
pub const ROLL_STATE_COMPLETED: u8 = 1;

// Item rarities (matching roll rewards)
pub const RARITY_COMMON: u8 = 0;
pub const RARITY_UNCOMMON: u8 = 1;
pub const RARITY_RARE: u8 = 2;
pub const RARITY_EPIC: u8 = 3;
pub const RARITY_LEGENDARY: u8 = 4;
pub const RARITY_MYTHIC: u8 = 5;

// Item types
pub const ITEM_TYPE_WEAPON: u8 = 0;
pub const ITEM_TYPE_ARMOR: u8 = 1;
pub const ITEM_TYPE_ACCESSORY: u8 = 2;

// Battle actions
pub const ACTION_ATTACK: u8 = 0;
pub const ACTION_DEFEND: u8 = 1;
pub const ACTION_SKILL: u8 = 2;
pub const ACTION_ESCAPE: u8 = 3;