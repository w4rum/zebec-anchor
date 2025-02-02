//Zebec Anchor Program - https://docs.zebec.io/
use anchor_lang::prelude::*;
use solana_security_txt::security_txt;
declare_id!("zbcKGdAmXfthXY3rEPBzexVByT2cqRqCZb9NwWdGQ2T");

pub mod utils;
pub mod error;
pub mod constants;
pub mod processor;

pub use processor::*;

#[program]
mod zebec {
    use super::*;
    pub fn create_fee_account(
        ctx:Context<InitializeFeeVault>,
        fee_percentage:u64
    )->Result<()>{
        process_create_fee_account(ctx,fee_percentage)
    }
    pub fn withdraw_fees_token(
        ctx: Context<WithdrawFeesToken>,
    )->Result<()>{
        process_withdraw_fees_token(ctx)
    }
    pub fn withdraw_fees_sol(
        ctx: Context<WithdrawFeesSol>,
    )->Result<()>{
        process_withdraw_fees_sol(ctx)
    }
    pub fn update_fees(
        ctx: Context<UpdateFees>,
        fee_percentage:u64,
    )->Result<()>{
        process_update_fee(ctx, fee_percentage)
    }
    pub fn deposit_sol(
        ctx: Context<InitializeMasterPda>,
        amount: u64
    )-> Result<()> {
        process_deposit_sol(ctx,amount)
    }
    pub fn native_stream(
        ctx: Context<Initialize>,
        start_time: u64,
        end_time: u64,
        amount: u64,
        can_cancel:bool,
        can_update:bool,
    )-> Result<()> {
        process_native_stream(ctx,start_time,end_time,amount,can_cancel,can_update)
    }
    pub fn native_stream_update(
        ctx: Context<StreamUpdate>,
        start_time: u64,
        end_time: u64,
        amount: u64
    )-> Result<()> {
        process_update_native_stream(ctx,start_time,end_time,amount)
    }
    pub fn withdraw_stream(
        ctx: Context<Withdraw>,
    ) -> Result<()> {
        process_withdraw_stream(ctx)
    }
    pub fn pause_stream(
        ctx: Context<Pause>,
    ) -> Result<()> {
        process_pause_stream(ctx)
    }
    pub fn cancel_stream(
        ctx: Context<Cancel>,
    ) -> Result<()> {
        process_cancel_stream(ctx)
    }
    pub fn instant_native_transfer(
        ctx:Context<InstantTransfer>,
        amount:u64
    )->Result<()>{
        process_native_transfer(ctx, amount)
    }
    pub fn native_withdrawal(
        ctx: Context<InitializerWithdrawal>,
        amount: u64,
    ) ->Result<()>{
        process_native_withdrawal(ctx,amount)
    }
    pub fn send_sol_directly(
        ctx:Context<TransferDirect>,
        amount:u64,
    ) -> Result<()>{
        process_sol_directly(ctx, amount)
    }
        
    pub fn deposit_token(
        ctx: Context<TokenDeposit>,
        amount: u64,
    )   ->Result<()>{
        process_deposit_token(ctx,amount)
    }
    pub fn token_stream(
        ctx:Context<TokenStream>,
        start_time:u64,
        end_time:u64,
        amount:u64,
        can_cancel:bool,
        can_update:bool,
    ) ->Result<()>{
        process_token_stream(ctx,start_time,end_time,amount,can_cancel,can_update)
    }
    pub fn token_stream_update(
        ctx:Context<TokenStreamUpdate>,
        start_time:u64,
        end_time:u64,
        amount:u64,
    )->Result<()>{
        process_update_token_stream(ctx,start_time,end_time,amount)
    }
    pub fn withdraw_token_stream(
        ctx: Context<TokenWithdrawStream>,
    )   ->Result<()>{
        process_withdraw_token_stream(ctx)
    }
    pub fn pause_resume_token_stream(
        ctx: Context<PauseTokenStream>,
    ) -> Result<()> {
        process_pause_resume_token_stream(ctx)
    }
    pub fn cancel_token_stream(
        ctx: Context<CancelTokenStream>,
    )   ->Result<()>{
        process_cancel_token_stream(ctx)
    }
    pub fn instant_token_transfer(
        ctx: Context<TokenInstantTransfer>,
        amount:u64,
    )   ->Result<()>{
        process_instant_token_transfer(ctx, amount)
    }
    pub fn token_withdrawal(
        ctx: Context<InitializerTokenWithdrawal>,
        amount: u64,
    ) -> Result<()>{
        process_token_withdrawal(ctx,amount)
    }
    pub fn send_token_directly(
        ctx:Context<TokenDirectTransfer>,
        amount:u64,
    ) -> Result<()>{
        process_send_token_directly(ctx, amount)
    }
}

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Zebec Protocol",
    project_url: "https://zebec.io/",
    contacts: "email:security@zebec.io",
    policy: "https://docs.zebec.io/",
    preferred_languages: "en",
    source_code: "https://github.com/Zebec-protocol/zebec-anchor",
    auditors: "Trail of Bits, Inc (Anders Helsing and Troy Sargent)",
    acknowledgements: "This program is elgible for bug bounty program, we appreciate any contributions that enhances security.
                        -Zebec"
}
