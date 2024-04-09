
pub mod pack;
pub mod processor;
pub mod state;
pub mod string;

use crate::swap_raydium::{handle_swap_raydium_in, handle_swap_raydium_out};
use anchor_lang::prelude::*;
pub use pack::*;
pub use processor::*;
pub use state::*;

declare_id!("J3pQMpuJShpYRv7gTQGshhC7rUu3pcXU2nn6zZKABzZU");

#[program]
pub mod icrosschain_swap_solana {

    use super::*;

    pub fn swap_solana_in(
        ctx: Context<SwapAccount>,
        amount_in: u64,
        minimum_amount_out: u64
    ) -> Result<()> {
        handle_swap_raydium_in(&ctx, amount_in, minimum_amount_out).map_err(Into::into)
    }

    pub fn swap_solana_out(
        ctx: Context<SwapAccount>,
        maximum_amount_in: u64,
        amount_out: u64,
    ) -> Result<()> {
        handle_swap_raydium_out(&ctx, maximum_amount_in, amount_out).map_err(Into::into)
    }
}
