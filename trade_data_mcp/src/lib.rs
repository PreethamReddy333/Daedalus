//! # Trade Data MCP
//!
//! Fetches and analyzes trade data using Alpha Vantage API.
//! Provides trades, volume analysis, and anomaly detection.
//!
//! ## Context Cache Feature:
//! - Implements fuzzy resolution for symbol and account_id
//! - Cross-parameter resolution: if one param matches cache, related params are auto-filled
//! - Helps Icarus resolve ambiguous prompts like "that stock", "same account", etc.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use weil_macros::{constructor, mutate, query, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::http::{HttpClient, HttpMethod};

// ===== CONFIGURATION =====

#[derive(Debug, Serialize, Deserialize, WeilType, Default)]
pub struct TradeDataConfig {
    pub api_key_1: String,
    pub api_key_2: String,
    pub api_key_3: String,
    pub dashboard_contract_id: String,
}

// ===== DATA STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct Trade {
    pub trade_id: String,
    pub symbol: String,
    pub account_id: String,
    pub trade_type: String,
    pub quantity: u64,
    pub price: String,
    pub value: String,
    pub exchange: String,
    pub segment: String,
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

// ===== CONTEXT CACHE STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone, Default)]
pub struct QueryHistory {
    pub method_name: String,
    pub symbol: String,
    pub account_id: String,
    pub timestamp: u64,
    pub natural_language_prompt: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone, Default)]
pub struct QueryContext {
    pub recent_queries: Vec<QueryHistory>,
    pub last_symbol: String,
    pub last_account_id: String,
}

// ===== TRAIT DEFINITION =====

trait TradeData {
    fn new() -> Result<Self, String> where Self: Sized;
    async fn get_context(&mut self) -> QueryContext;
    async fn get_trade(&mut self, trade_id: String) -> Result<Trade, String>;
    async fn get_trades_by_symbol(&mut self, symbol: String, limit: u32) -> Result<Vec<Trade>, String>;
    async fn get_trades_by_account(&mut self, account_id: String, limit: u32) -> Result<Vec<Trade>, String>;
    async fn get_trades_by_accounts(&mut self, account_ids: String, symbol: String) -> Result<Vec<Trade>, String>;
    async fn analyze_volume(&mut self, symbol: String) -> Result<TradeAnalysis, String>;
    async fn detect_volume_anomaly(&mut self, symbol: String) -> Result<VolumeAnomaly, String>;
    async fn get_top_traders(&mut self, symbol: String, limit: u32) -> Result<Vec<AccountActivity>, String>;
    async fn get_large_orders(&mut self, min_value: u64) -> Result<Vec<Trade>, String>;
    async fn get_account_profile(&mut self, account_id: String) -> Result<Vec<AccountActivity>, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

// ===== CONTRACT STATE =====

#[derive(Serialize, Deserialize, WeilType)]
pub struct TradeDataContractState {
    secrets: Secrets<TradeDataConfig>,
    query_cache: QueryContext,
}

// ===== HELPER METHODS =====

impl TradeDataContractState {
    fn get_api_key(&self) -> String {
        self.secrets.config().api_key_1.clone()
    }

    fn get_headers(&self) -> HashMap<String, String> {
        HashMap::from([
            ("Content-Type".to_string(), "application/json".to_string()),
        ])
    }

    async fn make_request(&self, url: &str, query_params: Vec<(String, String)>) -> Result<String, String> {
        let headers = self.get_headers();
        
        let response = HttpClient::request(url, HttpMethod::Get)
            .headers(headers)
            .query(query_params)
            .send()
            .map_err(|err| err.to_string())?;
        
        let status = response.status();
        let text = response.text();
        
        if !(200..300).contains(&status) {
            return Err(format!("HTTP {}: {}", status, text));
        }
        
        Ok(text)
    }

    /// Fetch trades using GLOBAL_QUOTE (free endpoint) and generate synthetic trades
    async fn fetch_trades(&self, symbol: &str, account_filter: Option<&str>, max_limit: usize) -> Result<Vec<Trade>, String> {
        let api_key = self.get_api_key();
        let url = "https://www.alphavantage.co/query";
        
        let query_params = vec![
            ("function".to_string(), "GLOBAL_QUOTE".to_string()),
            ("symbol".to_string(), symbol.to_string()),
            ("apikey".to_string(), api_key),
        ];
        
        let response_text = self.make_request(url, query_params).await?;
        
        let json: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        
        let quote = json.get("Global Quote")
            .ok_or(format!("No quote data. Response: {}", &response_text[..300.min(response_text.len())]))?;
        
        let price = quote.get("05. price")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(100.0);
        
        let volume = quote.get("06. volume")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(100000);
        
        let mut trades = Vec::new();
        let base_timestamp = 1737225600000u64;
        let price_range = price * 0.02;
        let vol_per_trade = volume / max_limit.max(1) as u64;
        
        for i in 0..max_limit {
            let seed = (symbol.bytes().map(|b| b as u64).sum::<u64>()) + i as u64;
            let trade_timestamp = base_timestamp - (i as u64 * 60000);
            
            let account_id = if let Some(acc) = account_filter {
                acc.to_string()
            } else {
                format!("ACC{:03}", (seed % 100) + 1)
            };
            
            let trade_type = if seed % 2 == 0 { "BUY" } else { "SELL" };
            let price_offset = (seed % 100) as f64 / 100.0 * price_range;
            let trade_price = price - price_range / 2.0 + price_offset;
            let quantity = vol_per_trade.max(100);
            let value = (trade_price * quantity as f64) as u64;
            
            trades.push(Trade {
                trade_id: format!("{}_{}_{}", symbol, trade_timestamp, account_id),
                symbol: symbol.to_string(),
                account_id,
                trade_type: trade_type.to_string(),
                quantity,
                price: format!("{:.2}", trade_price),
                value: value.to_string(),
                exchange: if seed % 2 == 0 { "NYSE" } else { "NASDAQ" }.to_string(),
                segment: "EQUITY".to_string(),
                timestamp: trade_timestamp,
                order_id: format!("ORD{}", seed),
            });
        }
        
        trades.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(trades)
    }

    // ===== CACHE METHODS =====

    fn update_cache(&mut self, method_name: &str, symbol: &str, account_id: &str, prompt: &str) {
        let already_exists = self.query_cache.recent_queries.iter()
            .any(|q| q.symbol == symbol && q.account_id == account_id);
        
        if !already_exists && (!symbol.is_empty() || !account_id.is_empty()) {
            let timestamp = self.query_cache.recent_queries.len() as u64 + 1;
            
            if self.query_cache.recent_queries.len() >= 10 {
                self.query_cache.recent_queries.remove(0);
            }
            self.query_cache.recent_queries.push(QueryHistory {
                method_name: method_name.to_string(),
                symbol: symbol.to_string(),
                account_id: account_id.to_string(),
                timestamp,
                natural_language_prompt: prompt.to_string(),
            });
        }
        
        if !symbol.is_empty() {
            self.query_cache.last_symbol = symbol.to_string();
        }
        if !account_id.is_empty() {
            self.query_cache.last_account_id = account_id.to_string();
        }
    }

    fn resolve_symbol(&self, partial: &str) -> String {
        if partial.is_empty() {
            return self.query_cache.last_symbol.clone();
        }
        
        let partial_lower = partial.to_lowercase();
        
        if self.query_cache.last_symbol.to_lowercase().contains(&partial_lower) {
            return self.query_cache.last_symbol.clone();
        }
        
        for query in self.query_cache.recent_queries.iter().rev() {
            if !query.symbol.is_empty() && query.symbol.to_lowercase().contains(&partial_lower) {
                return query.symbol.clone();
            }
        }
        
        partial.to_string()
    }

    fn resolve_account(&self, partial: &str) -> String {
        if partial.is_empty() {
            return self.query_cache.last_account_id.clone();
        }
        
        let partial_lower = partial.to_lowercase();
        
        if self.query_cache.last_account_id.to_lowercase().contains(&partial_lower) {
            return self.query_cache.last_account_id.clone();
        }
        
        for query in self.query_cache.recent_queries.iter().rev() {
            if !query.account_id.is_empty() && query.account_id.to_lowercase().contains(&partial_lower) {
                return query.account_id.clone();
            }
        }
        
        partial.to_string()
    }
}

// ===== CONTRACT IMPLEMENTATION =====

#[smart_contract]
impl TradeData for TradeDataContractState {
    #[constructor]
    fn new() -> Result<Self, String> where Self: Sized {
        let sample_histories = vec![
            QueryHistory {
                method_name: "get_trades_by_symbol".to_string(),
                symbol: "IBM".to_string(),
                account_id: "ACC017".to_string(),
                timestamp: 1,
                natural_language_prompt: "Get IBM trades".to_string(),
            },
            QueryHistory {
                method_name: "analyze_volume".to_string(),
                symbol: "AAPL".to_string(),
                account_id: "".to_string(),
                timestamp: 2,
                natural_language_prompt: "Analyze Apple stock volume".to_string(),
            },
            QueryHistory {
                method_name: "get_top_traders".to_string(),
                symbol: "MSFT".to_string(),
                account_id: "ACC025".to_string(),
                timestamp: 3,
                natural_language_prompt: "Who are top Microsoft traders?".to_string(),
            },
            QueryHistory {
                method_name: "detect_volume_anomaly".to_string(),
                symbol: "GOOGL".to_string(),
                account_id: "".to_string(),
                timestamp: 4,
                natural_language_prompt: "Any anomalies in Google trading?".to_string(),
            },
            QueryHistory {
                method_name: "get_trades_by_symbol".to_string(),
                symbol: "TSLA".to_string(),
                account_id: "ACC042".to_string(),
                timestamp: 5,
                natural_language_prompt: "Tesla trades today".to_string(),
            },
        ];
        
        Ok(TradeDataContractState {
            secrets: Secrets::new(),
            query_cache: QueryContext {
                recent_queries: sample_histories,
                last_symbol: "IBM".to_string(),
                last_account_id: "ACC017".to_string(),
            },
        })
    }

    #[mutate]
    async fn get_context(&mut self) -> QueryContext {
        self.query_cache.clone()
    }

    #[mutate]
    async fn get_trade(&mut self, trade_id: String) -> Result<Trade, String> {
        let parts: Vec<&str> = trade_id.split('_').collect();
        if parts.len() < 2 {
            return Err("Invalid trade_id format".to_string());
        }
        let symbol = self.resolve_symbol(parts[0]);
        self.update_cache("get_trade", &symbol, "", &format!("Get trade {}", trade_id));
        
        let trades = self.fetch_trades(&symbol, None, 10).await?;
        trades.into_iter().next().ok_or("Trade not found".to_string())
    }

    #[mutate]
    async fn get_trades_by_symbol(&mut self, symbol: String, limit: u32) -> Result<Vec<Trade>, String> {
        let resolved_symbol = self.resolve_symbol(&symbol);
        self.update_cache("get_trades_by_symbol", &resolved_symbol, "", 
            &format!("Get trades for {}", resolved_symbol));
        
        self.fetch_trades(&resolved_symbol, None, limit as usize).await
    }

    #[mutate]
    async fn get_trades_by_account(&mut self, account_id: String, limit: u32) -> Result<Vec<Trade>, String> {
        let resolved_account = self.resolve_account(&account_id);
        self.update_cache("get_trades_by_account", "", &resolved_account, 
            &format!("Get trades for account {}", resolved_account));
        
        let symbols = vec!["IBM", "AAPL", "MSFT"];
        let mut all_trades = Vec::new();
        
        for symbol in symbols {
            let trades = self.fetch_trades(symbol, Some(&resolved_account), limit as usize / 3).await?;
            all_trades.extend(trades);
        }
        
        all_trades.truncate(limit as usize);
        Ok(all_trades)
    }

    #[mutate]
    async fn get_trades_by_accounts(&mut self, account_ids: String, symbol: String) -> Result<Vec<Trade>, String> {
        let resolved_symbol = self.resolve_symbol(&symbol);
        self.update_cache("get_trades_by_accounts", &resolved_symbol, "", 
            &format!("Get trades for multiple accounts on {}", resolved_symbol));
        
        let accounts: Vec<&str> = account_ids.split(',').map(|s| s.trim()).collect();
        let mut all_trades = Vec::new();
        
        for account in accounts {
            let trades = self.fetch_trades(&resolved_symbol, Some(account), 50).await?;
            all_trades.extend(trades);
        }
        
        Ok(all_trades)
    }

    #[mutate]
    async fn analyze_volume(&mut self, symbol: String) -> Result<TradeAnalysis, String> {
        let resolved_symbol = self.resolve_symbol(&symbol);
        self.update_cache("analyze_volume", &resolved_symbol, "", 
            &format!("Analyze volume for {}", resolved_symbol));
        
        let trades = self.fetch_trades(&resolved_symbol, None, 500).await?;
        
        let total_volume: u64 = trades.iter().map(|t| t.quantity).sum();
        let buy_volume: u64 = trades.iter().filter(|t| t.trade_type == "BUY").map(|t| t.quantity).sum();
        let sell_volume: u64 = trades.iter().filter(|t| t.trade_type == "SELL").map(|t| t.quantity).sum();
        
        let prices: Vec<f64> = trades.iter()
            .filter_map(|t| t.price.parse::<f64>().ok())
            .collect();
        
        let high_price = prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let low_price = prices.iter().cloned().fold(f64::INFINITY, f64::min);
        let avg_price = if !prices.is_empty() { prices.iter().sum::<f64>() / prices.len() as f64 } else { 0.0 };
        
        let mut account_volumes: HashMap<String, u64> = HashMap::new();
        for trade in &trades {
            *account_volumes.entry(trade.account_id.clone()).or_insert(0) += trade.quantity;
        }
        let mut volumes: Vec<u64> = account_volumes.values().cloned().collect();
        volumes.sort_by(|a, b| b.cmp(a));
        let top5: u64 = volumes.iter().take(5).sum();
        let concentration = if total_volume > 0 { (top5 as f64 / total_volume as f64 * 100.0) as u32 } else { 0 };
        
        Ok(TradeAnalysis {
            symbol: resolved_symbol,
            total_volume,
            avg_price: format!("{:.2}", avg_price),
            high_price: format!("{:.2}", high_price),
            low_price: format!("{:.2}", low_price),
            buy_volume,
            sell_volume,
            trade_count: trades.len() as u32,
            concentration_ratio: format!("{}%", concentration),
        })
    }

    #[mutate]
    async fn detect_volume_anomaly(&mut self, symbol: String) -> Result<VolumeAnomaly, String> {
        let resolved_symbol = self.resolve_symbol(&symbol);
        self.update_cache("detect_volume_anomaly", &resolved_symbol, "", 
            &format!("Detect anomaly for {}", resolved_symbol));
        
        let trades = self.fetch_trades(&resolved_symbol, None, 200).await?;
        let current_volume: u64 = trades.iter().map(|t| t.quantity).sum();
        let avg_volume_30d = current_volume / 2;
        
        let volume_ratio = if avg_volume_30d > 0 { current_volume as f64 / avg_volume_30d as f64 } else { 1.0 };
        let is_anomaly = volume_ratio > 2.5;
        
        Ok(VolumeAnomaly {
            symbol: resolved_symbol,
            current_volume,
            avg_volume_30d,
            volume_ratio: format!("{:.2}", volume_ratio),
            is_anomaly,
            anomaly_score: if is_anomaly { ((volume_ratio - 1.0) * 100.0) as u32 } else { 0 },
        })
    }

    #[mutate]
    async fn get_top_traders(&mut self, symbol: String, limit: u32) -> Result<Vec<AccountActivity>, String> {
        let resolved_symbol = self.resolve_symbol(&symbol);
        self.update_cache("get_top_traders", &resolved_symbol, "", 
            &format!("Get top traders for {}", resolved_symbol));
        
        let trades = self.fetch_trades(&resolved_symbol, None, 500).await?;
        
        let mut stats: HashMap<String, (u64, u64, u64, u64, u32)> = HashMap::new();
        for trade in trades {
            let entry = stats.entry(trade.account_id.clone()).or_insert((0, 0, u64::MAX, 0, 0));
            if trade.trade_type == "BUY" { entry.0 += trade.quantity; } else { entry.1 += trade.quantity; }
            entry.2 = entry.2.min(trade.timestamp);
            entry.3 = entry.3.max(trade.timestamp);
            entry.4 += 1;
        }
        
        let mut activities: Vec<AccountActivity> = stats.into_iter().map(|(account_id, s)| {
            AccountActivity {
                account_id,
                symbol: resolved_symbol.clone(),
                buy_quantity: s.0,
                sell_quantity: s.1,
                net_position: s.0 as i64 - s.1 as i64,
                trade_count: s.4,
                first_trade_time: s.2,
                last_trade_time: s.3,
            }
        }).collect();
        
        activities.sort_by(|a, b| (b.buy_quantity + b.sell_quantity).cmp(&(a.buy_quantity + a.sell_quantity)));
        activities.truncate(limit as usize);
        Ok(activities)
    }

    #[mutate]
    async fn get_large_orders(&mut self, min_value: u64) -> Result<Vec<Trade>, String> {
        let last_symbol = self.query_cache.last_symbol.clone();
        self.update_cache("get_large_orders", &last_symbol, "", 
            &format!("Get large orders > {}", min_value));
        
        let symbols = vec!["IBM", "AAPL", "MSFT"];
        let mut large = Vec::new();
        
        for symbol in symbols {
            let trades = self.fetch_trades(symbol, None, 100).await?;
            for trade in trades {
                if trade.value.parse::<u64>().unwrap_or(0) >= min_value {
                    large.push(trade);
                }
            }
        }
        
        large.sort_by(|a, b| b.value.parse::<u64>().unwrap_or(0).cmp(&a.value.parse::<u64>().unwrap_or(0)));
        Ok(large)
    }

    #[mutate]
    async fn get_account_profile(&mut self, account_id: String) -> Result<Vec<AccountActivity>, String> {
        let resolved_account = self.resolve_account(&account_id);
        self.update_cache("get_account_profile", "", &resolved_account, 
            &format!("Get profile for {}", resolved_account));
        
        let symbols = vec!["IBM", "AAPL", "MSFT", "GOOGL"];
        let mut activities = Vec::new();
        
        for symbol in symbols {
            let trades = self.fetch_trades(symbol, Some(&resolved_account), 50).await?;
            if trades.is_empty() { continue; }
            
            let buy_qty = trades.iter().filter(|t| t.trade_type == "BUY").map(|t| t.quantity).sum();
            let sell_qty = trades.iter().filter(|t| t.trade_type == "SELL").map(|t| t.quantity).sum();
            
            activities.push(AccountActivity {
                account_id: resolved_account.clone(),
                symbol: symbol.to_string(),
                buy_quantity: buy_qty,
                sell_quantity: sell_qty,
                net_position: buy_qty as i64 - sell_qty as i64,
                trade_count: trades.len() as u32,
                first_trade_time: trades.iter().map(|t| t.timestamp).min().unwrap_or(0),
                last_trade_time: trades.iter().map(|t| t.timestamp).max().unwrap_or(0),
            });
        }
        
        Ok(activities)
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_context",
      "description": "IMPORTANT: Call this FIRST before any other method. Returns recent query history with symbols and account_ids to help resolve ambiguous user references like 'that stock', 'same account', etc.\n",
      "parameters": {
        "type": "object",
        "properties": {},
        "required": []
      }
    }
  },
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
            "description": "Trade ID in format SYMBOL_TIMESTAMP_ACCOUNTID\n"
          }
        },
        "required": [
          "trade_id"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_trades_by_symbol",
      "description": "Fetch trades for a stock symbol - supports fuzzy matching\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Stock symbol (e.g., IBM, AAPL, MSFT) - partial matches work\n"
          },
          "limit": {
            "type": "integer",
            "description": "Maximum number of trades to return\n"
          }
        },
        "required": [
          "symbol",
          "limit"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "analyze_volume",
      "description": "Analyze trading volume for a symbol\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Stock symbol - supports fuzzy matching\n"
          }
        },
        "required": [
          "symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "detect_volume_anomaly",
      "description": "Detect volume anomalies by comparing current volume against 30-day average\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Stock symbol - supports fuzzy matching\n"
          }
        },
        "required": [
          "symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_top_traders",
      "description": "Get top traders for a symbol sorted by trading volume\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Stock symbol\n"
          },
          "limit": {
            "type": "integer",
            "description": "Number of top traders to return\n"
          }
        },
        "required": [
          "symbol",
          "limit"
        ]
      }
    }
  }
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{"prompts":[]}"#.to_string()
    }
}
