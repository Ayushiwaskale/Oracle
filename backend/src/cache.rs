use deadpool_redis::{Config, Pool, Runtime};
use deadpool_redis::redis::AsyncCommands;
use anyhow::Result;
use crate::types::PricePoint;
use serde_json;

pub struct Cache {
    pool: Pool,
}

impl Cache {
    pub fn new(redis_url: &str) -> Result<Self> {
        let cfg = Config::from_url(redis_url);
        // create_pool requires a runtime argument (Tokio1)
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        Ok(Self { pool })
    }

    pub async fn set_price(&self, symbol: &str, price: &PricePoint) -> Result<()> {
        let mut con = self.pool.get().await?;
        let key = format!("price:{}", symbol);
        let val = serde_json::to_string(price)?;
        // AsyncCommands trait provides `set`
        let _: () = con.set(key, val).await?;
        Ok(())
    }

    pub async fn get_price(&self, symbol: &str) -> Result<Option<PricePoint>> {
        let mut con = self.pool.get().await?;
        let key = format!("price:{}", symbol);
        let v: Option<String> = con.get(key).await?;
        if let Some(s) = v {
            let p: PricePoint = serde_json::from_str(&s)?;
            Ok(Some(p))
        } else {
            Ok(None)
        }
    }
}
