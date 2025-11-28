use sqlx::PgPool;
use crate::types::PricePoint;
use anyhow::Result;
use time::OffsetDateTime;
use chrono::Utc;

pub struct Persistence {
    pool: PgPool,
}

impl Persistence {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn insert_price(&self, p: &PricePoint) -> Result<()> {
        let unix_ts = p.timestamp.timestamp();
        let ts: OffsetDateTime = OffsetDateTime::from_unix_timestamp(unix_ts)?;
        
        sqlx::query!(
            r#"INSERT INTO price_history (symbol, price_scaled, source, confidence, expo, timestamp)
               VALUES ($1, $2, $3, $4, $5, $6)"#,
            p.symbol,
            p.price_scaled,
            p.source,
            p.confidence,
            p.expo,
            ts
        ).execute(&self.pool).await?;
        Ok(())
    }
}
