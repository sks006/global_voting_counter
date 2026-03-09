use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;


pub const ADMIN_CONFIG_SEED: &[u8] = b"admin_config";
pub const COUNTER_SEED: &[u8] = b"counter";

pub const TAG_UNINITIALIZED: u8 = 0;
pub const TAG_COUNTER: u8 = 1;
pub const TAG_ADMIN_CONFIG: u8 = 2;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Counter {
    pub tag: u8,   // Offset 0
    pub count: u64, // Offset 1
     pub last_voter: Pubkey,
}
impl Counter {
    pub const LEN: usize = 1 + 8+32;
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AdminConfig {
    pub tag: u8,
    pub admin: Pubkey,
    pub is_paused: bool,
}
impl AdminConfig {
    pub const LEN: usize = 1 + 32 + 1;
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct LastVoter {
    pub tag: u8,
    pub last_voter: Pubkey,
}
impl LastVoter {
    pub const LEN: usize = 1 + 32;
}
