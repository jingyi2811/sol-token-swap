use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct SwapAccount<'info> {
    /// CHECK: account checked in CPI
    pub pool_program_id: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub token_program: Program<'info, Token>,
    /// CHECK: account checked in CPI
        #[account(mut)]
        pub amm_id: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI
        pub amm_authority: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI
        #[account(mut)]
        pub amm_open_orders: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI
        #[account(mut)]
        pub ato_or_mda: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI : To token
        #[account(mut)]
        pub pool_coin_token_account: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI: From token
        #[account(mut)]
        pub pool_pc_token_account: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI
        pub serum_program_id: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI
        #[account(mut)]
        pub serum_market: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI
        #[account(mut)]
        pub serum_bids: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI
        #[account(mut)]
        pub serum_asks: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI
        #[account(mut)]
        pub serum_event_queue: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI
        #[account(mut)]
        pub serum_coin_vault_account: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI
        #[account(mut)]
        pub serum_pc_vault_account: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI
        pub serum_vault_signer: UncheckedAccount<'info>,
        /// CHECK: account checked in CPI : Source account of token
    #[account(mut)]
    pub uer_source_token_account: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub uer_destination_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    // pub user_source_owner: UncheckedAccount<'info>,
    pub user_source_owner: Signer<'info>,
}

#[account]
pub struct SourceTx {
    pub author: Pubkey,
    pub content: Vec<String>,
    pub bump: u8,
}
