//! # Trade Data MCP Server
//!
//! Fetches and analyzes trade executions from external APIs or synthetic data.
//! Provides tools for Icarus to query trade data, detect anomalies, and analyze patterns.
//!
//! ## Features
//! - Fetch trades by symbol, account, or multiple accounts
//! - Volume analysis and anomaly detection
//! - Top trader identification for concentration analysis
//! - Large order detection for front-running analysis
//!
//! ## External Dependencies
//! - Configured API endpoint (Zerodha Kite, synthetic data server, etc.)
//! - Dashboard contract for cross-contract alert pushing

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use weil_macros::{constructor, query, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::http::{HttpClient, HttpMethod};

// ===== CONFIGURATION =====

#[derive(Debug, Serialize, Deserialize, WeilType, Default, Clone)]
pub struct TradeDataConfig {
    pub api_endpoint: String,
    pub api_key: String,
    pub dashboard_contract_id: String,
}

// ===== DATA STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct Trade {
    pub trade_id: String,
    pub symbol: String,
    pub account_id: String,
    pub trade_type: String,       // "BUY" or "SELL"
    pub quantity: u64,
    pub price: String,
    pub value: String,
    pub exchange: String,         // "NSE" or "BSE"
    pub segment: String,          // "EQUITY", "FNO", "CURRENCY"
    pub timestamp: u64,
    pub order_id: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct TradeAnalysis {
    pub symbol: String,
    pub total_volume: u64,
    pub avg_price: String,
    pub high_price: String,
    pub low_price: String,
    pub buy_volume: u64,
    pub sell_volume: u64,
    pub trade_count: u32,
    pub concentration_ratio: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct VolumeAnomaly {
    pub symbol: String,
    pub current_volume: u64,
    pub avg_volume_30d: u64,
    pub volume_ratio: String,
    pub is_anomaly: bool,
    pub anomaly_score: u32,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct AccountActivity {
    pub account_id: String,
    pub symbol: String,
    pub buy_quantity: u64,
    pub sell_quantity: u64,
    pub net_position: i64,
    pub trade_count: u32,
    pub first_trade_time: u64,
    pub last_trade_time: u64,
}

// ===== TRAIT DEFINITION =====

trait TradeData {
    fn new() -> Result<Self, String> where Self: Sized;
    async fn get_trade(&self, trade_id: String) -> Result<Trade, String>;
    async fn get_trades_by_symbol(&self, symbol: String, from_timestamp: u64, to_timestamp: u64, limit: u32) -> Result<Vec<Trade>, String>;
    async fn get_trades_by_account(&self, account_id: String, from_timestamp: u64, to_timestamp: u64, limit: u32) -> Result<Vec<Trade>, String>;
    async fn get_trades_by_accounts(&self, account_ids: String, symbol: String, from_timestamp: u64, to_timestamp: u64) -> Result<Vec<Trade>, String>;
    async fn analyze_volume(&self, symbol: String, from_timestamp: u64, to_timestamp: u64) -> Result<TradeAnalysis, String>;
    async fn detect_volume_anomaly(&self, symbol: String, date_timestamp: u64) -> Result<VolumeAnomaly, String>;
    async fn get_top_traders(&self, symbol: String, from_timestamp: u64, to_timestamp: u64, limit: u32) -> Result<Vec<AccountActivity>, String>;
    async fn get_large_orders(&self, min_value: u64, from_timestamp: u64, to_timestamp: u64) -> Result<Vec<Trade>, String>;
    async fn get_account_profile(&self, account_id: String, days_back: u32) -> Result<Vec<AccountActivity>, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

// ===== CONTRACT STATE =====

#[derive(Serialize, Deserialize, WeilType)]
pub struct TradeDataContractState {
    secrets: Secrets<TradeDataConfig>,
}

// ===== HELPER FUNCTIONS =====

impl TradeDataContractState {
    /// Make HTTP request to the trade data API
    async fn fetch_trades(&self, endpoint: &str) -> Result<Vec<Trade>, String> {
        let config = self.secrets.config();
        let url = format!("{}/{}", config.api_endpoint, endpoint);
        
        let response = HttpClient::request(&url, HttpMethod::Get)
            .header("Authorization", &format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .send()
            .map_err(|e| format!("HTTP request failed: {}", e))?;
        
        let response_text = response.text();
        let trades: Vec<Trade> = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse trades: {}", e))?;
        
        Ok(trades)
    }
    
    /// Calculate volume statistics from a list of trades
    fn calculate_volume_stats(&self, trades: &[Trade], symbol: &str) -> TradeAnalysis {
        let mut total_volume = 0u64;
        let mut buy_volume = 0u64;
        let mut sell_volume = 0u64;
        let mut prices: Vec<f64> = Vec::new();
        let mut account_volumes: HashMap<String, u64> = HashMap::new();
        
        for trade in trades {
            total_volume += trade.quantity;
            if trade.trade_type == "BUY" {
                buy_volume += trade.quantity;
            } else {
                sell_volume += trade.quantity;
            }
            
            if let Ok(price) = trade.price.parse::<f64>() {
                prices.push(price);
            }
            
            *account_volumes.entry(trade.account_id.clone()).or_insert(0) += trade.quantity;
        }
        
        // Calculate price stats
        let avg_price = if !prices.is_empty() {
            prices.iter().sum::<f64>() / prices.len() as f64
        } else {
            0.0
        };
        let high_price = prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let low_price = prices.iter().cloned().fold(f64::INFINITY, f64::min);
        
        // Calculate concentration ratio (top 5 accounts % of volume)
        let mut volumes: Vec<u64> = account_volumes.values().cloned().collect();
        volumes.sort_by(|a, b| b.cmp(a));
        let top5_volume: u64 = volumes.iter().take(5).sum();
        let concentration = if total_volume > 0 {
            (top5_volume as f64 / total_volume as f64) * 100.0
        } else {
            0.0
        };
        
        TradeAnalysis {
            symbol: symbol.to_string(),
            total_volume,
            avg_price: format!("{:.2}", avg_price),
            high_price: format!("{:.2}", if high_price.is_finite() { high_price } else { 0.0 }),
            low_price: format!("{:.2}", if low_price.is_finite() { low_price } else { 0.0 }),
            buy_volume,
            sell_volume,
            trade_count: trades.len() as u32,
            concentration_ratio: format!("{:.2}%", concentration),
        }
    }
}

// ===== CONTRACT IMPLEMENTATION =====

#[smart_contract]
impl TradeData for TradeDataContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(TradeDataContractState {
            secrets: Secrets::new(),
        })
    }

    /// Fetch a single trade by ID
    #[query]
    async fn get_trade(&self, trade_id: String) -> Result<Trade, String> {
        let trades = self.fetch_trades(&format!("trades/{}", trade_id)).await?;
        trades.into_iter().next()
            .ok_or_else(|| format!("Trade {} not found", trade_id))
    }

    /// Fetch trades for a symbol within a date range
    #[query]
    async fn get_trades_by_symbol(
        &self, 
        symbol: String, 
        from_timestamp: u64, 
        to_timestamp: u64, 
        limit: u32
    ) -> Result<Vec<Trade>, String> {
        let endpoint = format!(
            "trades?symbol={}&from={}&to={}&limit={}",
            symbol, from_timestamp, to_timestamp, limit
        );
        self.fetch_trades(&endpoint).await
    }

    /// Fetch trades for a specific account
    #[query]
    async fn get_trades_by_account(
        &self, 
        account_id: String, 
        from_timestamp: u64, 
        to_timestamp: u64, 
        limit: u32
    ) -> Result<Vec<Trade>, String> {
        let endpoint = format!(
            "trades?account={}&from={}&to={}&limit={}",
            account_id, from_timestamp, to_timestamp, limit
        );
        self.fetch_trades(&endpoint).await
    }

    /// Get trades by multiple accounts (for entity relationship checks)
    #[query]
    async fn get_trades_by_accounts(
        &self, 
        account_ids: String, 
        symbol: String, 
        from_timestamp: u64, 
        to_timestamp: u64
    ) -> Result<Vec<Trade>, String> {
        let accounts: Vec<&str> = account_ids.split(',').map(|s| s.trim()).collect();
        let mut all_trades = Vec::new();
        
        for account in accounts {
            let endpoint = format!(
                "trades?account={}&symbol={}&from={}&to={}",
                account, symbol, from_timestamp, to_timestamp
            );
            if let Ok(trades) = self.fetch_trades(&endpoint).await {
                all_trades.extend(trades);
            }
        }
        
        // Sort by timestamp descending
        all_trades.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(all_trades)
    }

    /// Analyze volume for a symbol
    #[query]
    async fn analyze_volume(
        &self, 
        symbol: String, 
        from_timestamp: u64, 
        to_timestamp: u64
    ) -> Result<TradeAnalysis, String> {
        let trades = self.get_trades_by_symbol(symbol.clone(), from_timestamp, to_timestamp, 10000).await?;
        Ok(self.calculate_volume_stats(&trades, &symbol))
    }

    /// Detect volume anomalies - compares current volume against 30-day average
    #[query]
    async fn detect_volume_anomaly(
        &self, 
        symbol: String, 
        date_timestamp: u64
    ) -> Result<VolumeAnomaly, String> {
        // Get current day's volume
        let day_start = date_timestamp - (date_timestamp % 86400000); // Start of day in ms
        let day_end = day_start + 86400000;
        
        let current_trades = self.get_trades_by_symbol(symbol.clone(), day_start, day_end, 100000).await?;
        let current_volume: u64 = current_trades.iter().map(|t| t.quantity).sum();
        
        // Get 30-day historical average
        let thirty_days_ago = day_start - (30 * 86400000);
        let historical_trades = self.get_trades_by_symbol(symbol.clone(), thirty_days_ago, day_start, 100000).await?;
        let historical_volume: u64 = historical_trades.iter().map(|t| t.quantity).sum();
        let avg_volume_30d = if historical_volume > 0 { historical_volume / 30 } else { 1 };
        
        // Calculate ratio and anomaly score
        let volume_ratio = current_volume as f64 / avg_volume_30d as f64;
        let is_anomaly = volume_ratio > 3.0; // 3x normal is anomaly
        let anomaly_score = if volume_ratio > 5.0 {
            100
        } else if volume_ratio > 3.0 {
            ((volume_ratio - 3.0) / 2.0 * 50.0 + 50.0) as u32
        } else {
            (volume_ratio / 3.0 * 50.0) as u32
        };
        
        Ok(VolumeAnomaly {
            symbol,
            current_volume,
            avg_volume_30d,
            volume_ratio: format!("{:.2}x", volume_ratio),
            is_anomaly,
            anomaly_score: anomaly_score.min(100),
        })
    }

    /// Get top traders for a symbol (for concentration analysis)
    #[query]
    async fn get_top_traders(
        &self, 
        symbol: String, 
        from_timestamp: u64, 
        to_timestamp: u64, 
        limit: u32
    ) -> Result<Vec<AccountActivity>, String> {
        let trades = self.get_trades_by_symbol(symbol.clone(), from_timestamp, to_timestamp, 100000).await?;
        
        // Aggregate by account
        let mut account_stats: HashMap<String, AccountActivity> = HashMap::new();
        
        for trade in trades {
            let activity = account_stats.entry(trade.account_id.clone()).or_insert(AccountActivity {
                account_id: trade.account_id.clone(),
                symbol: symbol.clone(),
                buy_quantity: 0,
                sell_quantity: 0,
                net_position: 0,
                trade_count: 0,
                first_trade_time: trade.timestamp,
                last_trade_time: trade.timestamp,
            });
            
            if trade.trade_type == "BUY" {
                activity.buy_quantity += trade.quantity;
                activity.net_position += trade.quantity as i64;
            } else {
                activity.sell_quantity += trade.quantity;
                activity.net_position -= trade.quantity as i64;
            }
            activity.trade_count += 1;
            activity.first_trade_time = activity.first_trade_time.min(trade.timestamp);
            activity.last_trade_time = activity.last_trade_time.max(trade.timestamp);
        }
        
        // Sort by total volume and take top N
        let mut activities: Vec<AccountActivity> = account_stats.into_values().collect();
        activities.sort_by(|a, b| {
            let vol_a = a.buy_quantity + a.sell_quantity;
            let vol_b = b.buy_quantity + b.sell_quantity;
            vol_b.cmp(&vol_a)
        });
        
        Ok(activities.into_iter().take(limit as usize).collect())
    }

    /// Fetch large institutional orders (for front-running detection)
    #[query]
    async fn get_large_orders(
        &self, 
        min_value: u64, 
        from_timestamp: u64, 
        to_timestamp: u64
    ) -> Result<Vec<Trade>, String> {
        let endpoint = format!(
            "trades?min_value={}&from={}&to={}",
            min_value, from_timestamp, to_timestamp
        );
        self.fetch_trades(&endpoint).await
    }

    /// Get trading activity for an account across all symbols
    #[query]
    async fn get_account_profile(
        &self, 
        account_id: String, 
        days_back: u32
    ) -> Result<Vec<AccountActivity>, String> {
        let now = 0u64; // Would use Runtime::timestamp() in production
        let from_timestamp = now - (days_back as u64 * 86400000);
        
        let trades = self.get_trades_by_account(account_id.clone(), from_timestamp, now, 100000).await?;
        
        // Aggregate by symbol
        let mut symbol_stats: HashMap<String, AccountActivity> = HashMap::new();
        
        for trade in trades {
            let activity = symbol_stats.entry(trade.symbol.clone()).or_insert(AccountActivity {
                account_id: account_id.clone(),
                symbol: trade.symbol.clone(),
                buy_quantity: 0,
                sell_quantity: 0,
                net_position: 0,
                trade_count: 0,
                first_trade_time: trade.timestamp,
                last_trade_time: trade.timestamp,
            });
            
            if trade.trade_type == "BUY" {
                activity.buy_quantity += trade.quantity;
                activity.net_position += trade.quantity as i64;
            } else {
                activity.sell_quantity += trade.quantity;
                activity.net_position -= trade.quantity as i64;
            }
            activity.trade_count += 1;
            activity.first_trade_time = activity.first_trade_time.min(trade.timestamp);
            activity.last_trade_time = activity.last_trade_time.max(trade.timestamp);
        }
        
        Ok(symbol_stats.into_values().collect())
    }

    /// Returns JSON schema of available tools
    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_trade",
      "description": "Fetch a single trade by ID\n",
      "parameters": {
        "type": "object",
        "properties": {
          "trade_id": {
            "type": "string",
            "description": "Unique trade identifier\n"
          }
        },
        "required": ["trade_id"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_trades_by_symbol",
      "description": "Fetch trades for a symbol within a date range. Returns trades sorted by timestamp descending.\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": { "type": "string", "description": "Stock symbol (e.g., TATASTEEL, RELIANCE)\n" },
          "from_timestamp": { "type": "integer", "description": "Start timestamp in milliseconds\n" },
          "to_timestamp": { "type": "integer", "description": "End timestamp in milliseconds\n" },
          "limit": { "type": "integer", "description": "Maximum number of trades to return\n" }
        },
        "required": ["symbol", "from_timestamp", "to_timestamp", "limit"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_trades_by_account",
      "description": "Fetch trades for a specific account. Used for tracking individual trader activity.\n",
      "parameters": {
        "type": "object",
        "properties": {
          "account_id": { "type": "string", "description": "Trading account ID\n" },
          "from_timestamp": { "type": "integer", "description": "Start timestamp in milliseconds\n" },
          "to_timestamp": { "type": "integer", "description": "End timestamp in milliseconds\n" },
          "limit": { "type": "integer", "description": "Maximum number of trades to return\n" }
        },
        "required": ["account_id", "from_timestamp", "to_timestamp", "limit"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_trades_by_accounts",
      "description": "Get trades by multiple accounts (for entity relationship checks). Used to find trades by connected entities.\n",
      "parameters": {
        "type": "object",
        "properties": {
          "account_ids": { "type": "string", "description": "Comma-separated list of account IDs\n" },
          "symbol": { "type": "string", "description": "Stock symbol to filter by\n" },
          "from_timestamp": { "type": "integer", "description": "Start timestamp in milliseconds\n" },
          "to_timestamp": { "type": "integer", "description": "End timestamp in milliseconds\n" }
        },
        "required": ["account_ids", "symbol", "from_timestamp", "to_timestamp"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "analyze_volume",
      "description": "Analyze volume for a symbol. Returns aggregated volume statistics including buy/sell ratio and concentration.\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": { "type": "string", "description": "Stock symbol\n" },
          "from_timestamp": { "type": "integer", "description": "Start timestamp\n" },
          "to_timestamp": { "type": "integer", "description": "End timestamp\n" }
        },
        "required": ["symbol", "from_timestamp", "to_timestamp"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "detect_volume_anomaly",
      "description": "Detect volume anomalies. Compares current volume against 30-day average. Returns anomaly score 0-100.\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": { "type": "string", "description": "Stock symbol\n" },
          "date_timestamp": { "type": "integer", "description": "Current day timestamp\n" }
        },
        "required": ["symbol", "date_timestamp"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_top_traders",
      "description": "Get top traders for a symbol (for concentration analysis). Returns accounts with highest trading volume.\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": { "type": "string", "description": "Stock symbol\n" },
          "from_timestamp": { "type": "integer", "description": "Start timestamp\n" },
          "to_timestamp": { "type": "integer", "description": "End timestamp\n" },
          "limit": { "type": "integer", "description": "Number of top traders to return\n" }
        },
        "required": ["symbol", "from_timestamp", "to_timestamp", "limit"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_large_orders",
      "description": "Fetch large institutional orders. Used for front-running detection.\n",
      "parameters": {
        "type": "object",
        "properties": {
          "min_value": { "type": "integer", "description": "Minimum order value in rupees\n" },
          "from_timestamp": { "type": "integer", "description": "Start timestamp\n" },
          "to_timestamp": { "type": "integer", "description": "End timestamp\n" }
        },
        "required": ["min_value", "from_timestamp", "to_timestamp"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_account_profile",
      "description": "Get trading activity for an account across all symbols. Used for account profiling.\n",
      "parameters": {
        "type": "object",
        "properties": {
          "account_id": { "type": "string", "description": "Trading account ID\n" },
          "days_back": { "type": "integer", "description": "Number of days to analyze\n" }
        },
        "required": ["account_id", "days_back"]
      }
    }
  }
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{ "prompts": [] }"#.to_string()
    }
}
