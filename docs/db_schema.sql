CREATE TABLE IF NOT EXISTS price_history (
  id BIGSERIAL PRIMARY KEY,
  symbol TEXT NOT NULL,
  price_scaled BIGINT NOT NULL,
  source TEXT NOT NULL,
  confidence BIGINT,
  expo INT,
  timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_price_history_symbol_ts ON price_history(symbol, timestamp DESC);
