use crate::types::PricePoint;
use anyhow::Result;

pub struct Aggregator {}

impl Aggregator {
    pub fn new() -> Self { Self {} }

    pub fn consensus(prices: Vec<PricePoint>, max_dev_bps: i64) -> Result<PricePoint> {
        if prices.is_empty() { anyhow::bail!("no prices"); }
        let mut vals: Vec<i128> = prices.iter().map(|p| p.price_scaled as i128).collect();
        vals.sort();
        let median_i128 = if vals.len() % 2 == 1 {
            vals[vals.len()/2]
        } else {
            (vals[vals.len()/2 - 1] + vals[vals.len()/2]) / 2
        };

        for v in &vals {
            let diff = if *v > median_i128 { *v - median_i128 } else { median_i128 - *v };
            if median_i128 == 0 { anyhow::bail!("zero median"); }
            let diff_bps = diff * 10_000 / median_i128.abs();
            if diff_bps > max_dev_bps as i128 { anyhow::bail!("deviation too large"); }
        }

        let chosen = prices.into_iter().min_by_key(|p| (p.price_scaled as i128 - median_i128).abs())
            .ok_or_else(|| anyhow::anyhow!("no chosen price"))?;
        Ok(chosen)
    }
}
