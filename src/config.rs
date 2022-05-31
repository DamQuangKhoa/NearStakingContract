use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Copy)] // Binary
#[serde(crate = "near_sdk::serde")] // JSON
pub struct Config {
    pub reward_numerator: u32,
    pub reward_demurator: u32
}

impl Default for Config {
    fn default() -> Self {
        Self { reward_numerator: 715, reward_demurator: 1000000000}
    }
}