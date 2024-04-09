
pub mod pack;
pub mod processor;
pub mod state;
pub mod string;

use crate::swap_raydium::handle_swap_raydium;
use anchor_lang::prelude::*;
pub use pack::*;
pub use processor::*;
pub use state::*;

declare_id!("DHz1jSwTuKqeN41NAEVzmUYFmaZT7UyhG323YRyfEomT");

#[program]
pub mod icrosschain_swap_solana {

    use super::*;

    pub fn swap_solana(
        ctx: Context<SwapAccount>,
        amount_in: u64,
        minimum_amount_out: u64
    ) -> Result<()> {
        handle_swap_raydium(&ctx, amount_in, minimum_amount_out).map_err(Into::into)
    }
}
