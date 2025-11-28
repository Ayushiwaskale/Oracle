use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use anchor_lang::solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

declare_id!("Oracle1111111111111111111111111111111111111");

#[program]
pub mod oracle_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, symbol: String, pyth_feed: Pubkey, switch_agg: Pubkey) -> Result<()> {
        let conf = &mut ctx.accounts.config;
        conf.symbol = symbol;
        conf.pyth_feed = pyth_feed;
        conf.switchboard_aggregator = switch_agg;
        conf.max_staleness = 30;
        conf.max_confidence = 1_000_000;
        conf.max_deviation = 100;
        Ok(())
    }

    pub fn get_pyth_price(ctx: Context<GetPythPrice>, price_feed: Pubkey) -> Result<PriceData> {
        require!(ctx.accounts.price_account.key() == &price_feed, OracleError::InvalidFeedPubkey);
        let data = ctx.accounts.price_account.try_borrow_data()?;
        let (price, expo, conf, ts) = parse_pyth_price_stub(&data)?;
        let now = Clock::get()?.unix_timestamp;
        let max_stale = ctx.accounts.oracle_config.max_staleness;
        require!(now - ts < max_stale, OracleError::PriceStale);
        require!(conf <= ctx.accounts.oracle_config.max_confidence, OracleError::HighConfidence);
        Ok(PriceData { price, confidence: conf, expo, timestamp: ts, source: PriceSource::Pyth })
    }

    pub fn get_switchboard_price(ctx: Context<GetSwitchboardPrice>, aggregator: Pubkey) -> Result<PriceData> {
        require!(ctx.accounts.aggregator_account.key() == &aggregator, OracleError::InvalidFeedPubkey);
        let data = ctx.accounts.aggregator_account.try_borrow_data()?;
        let (price, expo, conf, ts) = parse_switchboard_stub(&data)?;
        let now = Clock::get()?.unix_timestamp;
        let max_stale = ctx.accounts.oracle_config.max_staleness;
        require!(now - ts < max_stale, OracleError::PriceStale);
        require!(conf <= ctx.accounts.oracle_config.max_confidence, OracleError::HighConfidence);
        Ok(PriceData { price, confidence: conf, expo, timestamp: ts, source: PriceSource::Switchboard })
    }

    pub fn validate_price_consensus(ctx: Context<ValidatePrice>, prices: Vec<PriceData>) -> Result<u64> {
        require!(!prices.is_empty(), OracleError::NoPricesProvided);
        let scale: i32 = 6;
        let mut norm: Vec<i128> = Vec::new();
        for p in &prices {
            let diff = scale - p.expo;
            let scaled = if diff >= 0 {
                (p.price as i128).checked_mul(10i128.pow(diff as u32)).ok_or(OracleError::MathError)?
            } else {
                (p.price as i128).checked_div(10i128.pow((-diff) as u32)).ok_or(OracleError::MathError)?
            };
            norm.push(scaled);
        }
        norm.sort();
        let median = if norm.len() % 2 == 1 {
            norm[norm.len()/2]
        } else {
            (norm[norm.len()/2 - 1] + norm[norm.len()/2]) / 2
        };
        let max_dev = ctx.accounts.oracle_config.max_deviation as i128;
        for v in &norm {
            let diff = if *v > median { *v - median } else { median - *v };
            if median == 0 { return Err(OracleError::ZeroPrice.into()); }
            let diff_bps = diff.checked_mul(10_000).ok_or(OracleError::MathError)?
                .checked_div(median.abs()).ok_or(OracleError::MathError)?;
            if diff_bps > max_dev { return Err(OracleError::DeviationTooLarge.into()); }
        }
        if median < 0 { return Err(OracleError::NegativeConsensus.into()); }
        Ok(median as u64)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 512)]
    pub oracle_config: Account<'info, OracleConfig>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetPythPrice<'info> {
    /// CHECK: parsed manually
    pub price_account: AccountInfo<'info>,
    pub oracle_config: Account<'info, OracleConfig>,
}

#[derive(Accounts)]
pub struct GetSwitchboardPrice<'info> {
    /// CHECK: parsed manually
    pub aggregator_account: AccountInfo<'info>,
    pub oracle_config: Account<'info, OracleConfig>,
}

#[derive(Accounts)]
pub struct ValidatePrice<'info> {
    pub oracle_config: Account<'info, OracleConfig>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PriceData {
    pub price: i64,
    pub confidence: u64,
    pub expo: i32,
    pub timestamp: i64,
    pub source: PriceSource,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum PriceSource {
    Pyth,
    Switchboard,
    Internal,
}

#[account]
pub struct OracleConfig {
    pub symbol: String,
    pub pyth_feed: Pubkey,
    pub switchboard_aggregator: Pubkey,
    pub max_staleness: i64,
    pub max_confidence: u64,
    pub max_deviation: u64,
}

#[error_code]
pub enum OracleError {
    #[msg("Invalid feed pubkey")]
    InvalidFeedPubkey,
    #[msg("Price stale")]
    PriceStale,
    #[msg("Confidence too high")]
    HighConfidence,
    #[msg("No prices provided")]
    NoPricesProvided,
    #[msg("Deviation too large")]
    DeviationTooLarge,
    #[msg("Zero price")]
    ZeroPrice,
    #[msg("Negative consensus price")]
    NegativeConsensus,
    #[msg("Math error")]
    MathError,
}

fn parse_pyth_price_stub(_data: &[u8]) -> Result<(i64,i32,u64,i64), ProgramError> {
    Err(ProgramError::Custom(6000))
}
fn parse_switchboard_stub(_data: &[u8]) -> Result<(i64,i32,u64,i64), ProgramError> {
    Err(ProgramError::Custom(6001))
}
