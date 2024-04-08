
pub mod errors;
pub mod pack;
pub mod processor;
pub mod state;
pub mod string;

use crate::swap_raydium::handle_swap_raydium;
use anchor_lang::prelude::*;
pub use errors::*;
pub use pack::*;
pub use processor::*;
pub use state::*;

declare_id!("oPWcieLyDu4Fa3fdf2hGhKbAVyFUoTruesbFpV4oKJs");

#[program]
pub mod icrosschain_swap_solana {

    use super::*;

    pub fn swap_solana(
        ctx: Context<SwapAccount>,
        amount_in: u64,
        minimum_amount_out: u64,
        version: u8,
    ) -> Result<()> {
        handle_swap_raydium(&ctx, amount_in, minimum_amount_out, version).map_err(Into::into)
    }
}
