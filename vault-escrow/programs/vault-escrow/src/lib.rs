pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

// This matches the instructor's example. If you generate your own program
// keypair, replace this with that keypair's public key.
declare_id!("aNksHVU3gU1mjCPtTBsVk9S7qokAUvXfotB2jBxQQvv");

#[program]
pub mod q3_26_vault {
    use super::*;
}
