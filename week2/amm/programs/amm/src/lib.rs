pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Fs2p9zZF6CeTEZuhthMnfUcCk5vrMzTZF6rq3GbFmPWS");

#[program]
pub mod amm {
    use super::*;
}
