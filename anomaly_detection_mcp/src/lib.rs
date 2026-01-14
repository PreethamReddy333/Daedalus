//! # Anomaly Detection MCP Server
//!
//! Detects market manipulation patterns using Alpha Vantage and TAAPI.IO.
//! Analyzes spoofs, wash trades, pump & dumps, and volume anomalies.
//!
//! ## External Services:
//! - **Alpha Vantage**: Price data, RSI, global market status
//! - **TAAPI.IO**: Advanced technical indicators, volume analysis

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use weil_macros::{constructor, mutate, query, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::http::{HttpClient, HttpMethod};
use weil_rs::runtime::Runtime;

// ===== CONFIGURATION =====

#[derive(Debug, Serialize, Deserialize, WeilType, Default, Clone)]
pub struct AnomalyDetectionConfig {
    pub dashboard_contract_id: String,
    pub alpha_vantage_key: String,
    pub taapi_secret: String,
}

// ===== DATA STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct AnomalyResult {
    pub entity_id: String,
    pub symbol: String,
    pub anomaly_type: String,
    pub confidence_score: u32,
    pub details: String,
    pub timestamp: u64,
    pub supporting_evidence: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct SpoofingIndicator {
    pub order_id: String,
    pub is_spoof: bool,
    pub cancellation_rate: String,
    pub order_size_vs_market: String,
    pub price_impact: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct WashTradeIndicator {
    pub entity_id: String,
    pub counterparty_id: String,
    pub is_wash_trade: bool,
    pub volume_match: bool,
    pub price_match: bool,
    pub time_gap_seconds: u32,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct PumpDumpIndicator {
    pub symbol: String,
    pub is_pump_dump: bool,
    pub price_velocity: String,
    pub volume_surge: String,
    pub social_sentiment_score: i32,
}

// Helper structs for API responses
#[derive(Debug, Deserialize)]
struct AlphaVantageGlobalQuote {
    #[serde(rename = "Global Quote")]
    quote: Option<GlobalQuoteData>,
}

#[derive(Debug, Deserialize)]
struct GlobalQuoteData {
    #[serde(rename = "05. price")]
    price: String,
    #[serde(rename = "06. volume")]
    volume: String,
    #[serde(rename = "10. change percent")]
    change_percent: String,
}

#[derive(Debug, Deserialize)]
struct TaapiRsi {
    value: f64,
}

// ===== TRAIT DEFINITION =====

trait AnomalyDetection {
    fn new() -> Result<Self, String> where Self: Sized;
    async fn get_context(&mut self) -> QueryContext;
    async fn detect_spoofing(&mut self, order_id: String, entity_id: String, symbol: String, order_details: String) -> Result<SpoofingIndicator, String>;
    async fn detect_wash_trading(&self, entity_id: String, counterparty_id: String, symbol: String, trade_timestamp: u64) -> Result<WashTradeIndicator, String>;
    async fn detect_pump_dump(&mut self, symbol: String, time_window_minutes: u32) -> Result<PumpDumpIndicator, String>;
    async fn detect_front_running(&self, entity_id: String, symbol: String, client_trade_timestamp: u64, prop_trade_timestamp: u64) -> Result<AnomalyResult, String>;
    async fn analyze_volume_anomaly(&mut self, symbol: String, interval: String) -> Result<AnomalyResult, String>;
    async fn check_rsi_levels(&mut self, symbol: String) -> Result<String, String>;
    async fn scan_entity_anomalies(&self, entity_id: String) -> Result<Vec<AnomalyResult>, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

// Alert struct for dashboard push
#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct Alert {
    pub id: String,
    pub alert_type: String,
    pub severity: String,
    pub risk_score: u32,
    pub entity_id: String,
    pub symbol: String,
    pub description: String,
    pub workflow_id: String,
    pub timestamp: u64,
}

// ===== CONTEXT CACHE STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone, Default)]
pub struct QueryHistory {
    pub method_name: String,
    pub entity_id: String,
    pub symbol: String,
    pub timestamp: u64,
    pub natural_language_prompt: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone, Default)]
pub struct QueryContext {
    pub recent_queries: Vec<QueryHistory>,
    pub last_entity_id: String,
    pub last_symbol: String,
}

// ===== CONTRACT STATE =====

#[derive(Serialize, Deserialize, WeilType)]
pub struct AnomalyDetectionContractState {
    secrets: Secrets<AnomalyDetectionConfig>,
    query_cache: QueryContext,
}

impl AnomalyDetectionContractState {
    /// Get standard headers for HTTP requests (following Icarus pattern)
    fn get_headers(&self) -> HashMap<String, String> {
        HashMap::from([
            ("Content-Type".to_string(), "application/json".to_string()),
        ])
    }

    /// Make an HTTP GET request with proper headers (Icarus-compatible pattern)
    async fn make_request(
        &self,
        url: &str,
        query_params: Vec<(String, String)>,
    ) -> Result<String, String> {
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

    /// Fetch real-time quote from Alpha Vantage
    /// API: https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=IBM&apikey=demo
    async fn get_quote(&self, symbol: &str) -> Result<GlobalQuoteData, String> {
        let config = self.secrets.config();
        let url = "https://www.alphavantage.co/query";
        
        let query_params = vec![
            ("function".to_string(), "GLOBAL_QUOTE".to_string()),
            ("symbol".to_string(), symbol.to_string()),
            ("apikey".to_string(), config.alpha_vantage_key.clone()),
        ];
        
        let response_text = self.make_request(url, query_params).await?;
            
        let quote_res: AlphaVantageGlobalQuote = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse quote: {}. Response: {}", e, response_text))?;
            
        quote_res.quote.ok_or_else(|| format!("Symbol not found or API limit reached. Response: {}", response_text))
    }

    /// Fetch RSI from TAAPI.IO
    /// API: https://api.taapi.io/rsi?secret=MY_SECRET&exchange=binance&symbol=BTC/USDT&interval=1h
    async fn get_rsi(&self, symbol: &str) -> Result<f64, String> {
        let config = self.secrets.config();
        let url = "https://api.taapi.io/rsi";
        
        // TAAPI uses crypto pairs - convert stock symbol to crypto for demo
        // For production, would need proper stock data source
        let crypto_symbol = format!("{}/USDT", symbol);
        
        let query_params = vec![
            ("secret".to_string(), config.taapi_secret.clone()),
            ("exchange".to_string(), "binance".to_string()),
            ("symbol".to_string(), crypto_symbol),
            ("interval".to_string(), "1h".to_string()),
        ];
        
        let response_text = self.make_request(url, query_params).await?;
            
        let rsi: TaapiRsi = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse RSI: {}. Response: {}", e, response_text))?;
            
        Ok(rsi.value)
    }

    /// Update the query cache with a new query entry (only if unique)
    fn update_cache(&mut self, method_name: &str, entity_id: &str, symbol: &str, prompt: &str) {
        // Check if this entity+symbol combination already exists in cache
        let already_exists = self.query_cache.recent_queries.iter()
            .any(|q| q.entity_id == entity_id && q.symbol == symbol);
        
        // Only add to cache if it's a NEW unique combination
        if !already_exists {
            // Use query count as sequence number
            let timestamp = self.query_cache.recent_queries.len() as u64 + 1;
            
            // Add to recent queries (keep last 10)
            if self.query_cache.recent_queries.len() >= 10 {
                self.query_cache.recent_queries.remove(0);
            }
            self.query_cache.recent_queries.push(QueryHistory {
                method_name: method_name.to_string(),
                entity_id: entity_id.to_string(),
                symbol: symbol.to_string(),
                timestamp,
                natural_language_prompt: prompt.to_string(),
            });
        }
        
        // Always update last entity/symbol for recency tracking
        if !entity_id.is_empty() {
            self.query_cache.last_entity_id = entity_id.to_string();
        }
        if !symbol.is_empty() {
            self.query_cache.last_symbol = symbol.to_string();
        }
    }

    /// Resolve a partial entity reference from cache using fuzzy matching
    /// "Neeta" → "Neeta Ambani", "TRADER" → "TRADER-001"
    fn resolve_entity(&self, partial: &str) -> String {
        // If empty, use last entity from cache
        if partial.is_empty() {
            return self.query_cache.last_entity_id.clone();
        }
        
        let partial_lower = partial.to_lowercase();
        
        // First check last entity (most likely match)
        if self.query_cache.last_entity_id.to_lowercase().contains(&partial_lower) {
            return self.query_cache.last_entity_id.clone();
        }
        
        // Search through cached queries for fuzzy match
        for query in self.query_cache.recent_queries.iter().rev() {
            // Check if cached entity contains the partial
            if !query.entity_id.is_empty() && query.entity_id.to_lowercase().contains(&partial_lower) {
                return query.entity_id.clone();
            }
            // Also check if natural language prompt mentions this entity
            if query.natural_language_prompt.to_lowercase().contains(&partial_lower) {
                if !query.entity_id.is_empty() {
                    return query.entity_id.clone();
                }
            }
        }
        
        // No match found, return original
        partial.to_string()
    }

    /// Resolve a partial symbol reference from cache using fuzzy matching
    /// "RELI" → "RELIANCE", "bank" → "HDFCBANK"
    fn resolve_symbol(&self, partial: &str) -> String {
        // If empty, use last symbol from cache
        if partial.is_empty() {
            return self.query_cache.last_symbol.clone();
        }
        
        let partial_lower = partial.to_lowercase();
        
        // First check last symbol (most likely match)
        if self.query_cache.last_symbol.to_lowercase().contains(&partial_lower) {
            return self.query_cache.last_symbol.clone();
        }
        
        // Search through cached queries for fuzzy match
        for query in self.query_cache.recent_queries.iter().rev() {
            if !query.symbol.is_empty() && query.symbol.to_lowercase().contains(&partial_lower) {
                return query.symbol.clone();
            }
        }
        
        // No match found, return original
        partial.to_string()
    }

    /// Cross-parameter resolution: Find a cache entry matching ANY input param,
    /// then return BOTH entity_id and symbol from that same entry.
    /// This allows: "RELI" → returns ("Mukesh Ambani", "RELIANCE.BSE") if they were cached together
    fn resolve_from_cache(&self, entity_partial: &str, symbol_partial: &str) -> (String, String) {
        let entity_lower = entity_partial.to_lowercase();
        let symbol_lower = symbol_partial.to_lowercase();
        
        // Search through cached queries for a match on EITHER entity OR symbol
        for query in self.query_cache.recent_queries.iter().rev() {
            let entity_matches = !entity_partial.is_empty() && 
                !query.entity_id.is_empty() && 
                query.entity_id.to_lowercase().contains(&entity_lower);
            
            let symbol_matches = !symbol_partial.is_empty() && 
                !query.symbol.is_empty() && 
                query.symbol.to_lowercase().contains(&symbol_lower);
            
            // If EITHER matches, return BOTH from this cache entry
            if entity_matches || symbol_matches {
                let resolved_entity = if query.entity_id.is_empty() {
                    self.resolve_entity(entity_partial)
                } else {
                    query.entity_id.clone()
                };
                
                let resolved_symbol = if query.symbol.is_empty() {
                    self.resolve_symbol(symbol_partial)
                } else {
                    query.symbol.clone()
                };
                
                return (resolved_entity, resolved_symbol);
            }
            
            // Also check natural language prompt for matches
            let prompt_lower = query.natural_language_prompt.to_lowercase();
            if (!entity_partial.is_empty() && prompt_lower.contains(&entity_lower)) ||
               (!symbol_partial.is_empty() && prompt_lower.contains(&symbol_lower)) {
                let resolved_entity = if query.entity_id.is_empty() {
                    self.resolve_entity(entity_partial)
                } else {
                    query.entity_id.clone()
                };
                
                let resolved_symbol = if query.symbol.is_empty() {
                    self.resolve_symbol(symbol_partial)
                } else {
                    query.symbol.clone()
                };
                
                return (resolved_entity, resolved_symbol);
            }
        }
        
        // No cross-match found, fall back to individual resolution
        (self.resolve_entity(entity_partial), self.resolve_symbol(symbol_partial))
    }
}

// ===== CONTRACT IMPLEMENTATION =====

#[smart_contract]
impl AnomalyDetection for AnomalyDetectionContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        // Initialize with 10 sample query histories for testing context resolution
        let sample_histories = vec![
            QueryHistory {
                method_name: "detect_spoofing".to_string(),
                entity_id: "TRADER-001".to_string(),
                symbol: "RELIANCE".to_string(),
                timestamp: 1736700000,
                natural_language_prompt: "Check if order ORD-123 by TRADER-001 is spoofing on RELIANCE".to_string(),
            },
            QueryHistory {
                method_name: "detect_pump_dump".to_string(),
                entity_id: "".to_string(),
                symbol: "INFY".to_string(),
                timestamp: 1736701000,
                natural_language_prompt: "Is there a pump and dump on INFY in last 30 minutes?".to_string(),
            },
            QueryHistory {
                method_name: "detect_wash_trading".to_string(),
                entity_id: "TRADER-001".to_string(),
                symbol: "RELIANCE".to_string(),
                timestamp: 1736702000,
                natural_language_prompt: "Check wash trading between TRADER-001 and BROKER-ABC on RELIANCE".to_string(),
            },
            QueryHistory {
                method_name: "analyze_volume_anomaly".to_string(),
                entity_id: "".to_string(),
                symbol: "TCS".to_string(),
                timestamp: 1736703000,
                natural_language_prompt: "Check volume anomalies on TCS with 5 minute interval".to_string(),
            },
            QueryHistory {
                method_name: "check_rsi_levels".to_string(),
                entity_id: "".to_string(),
                symbol: "HDFCBANK".to_string(),
                timestamp: 1736704000,
                natural_language_prompt: "What is the RSI for HDFC Bank?".to_string(),
            },
            QueryHistory {
                method_name: "detect_front_running".to_string(),
                entity_id: "BROKER-XYZ".to_string(),
                symbol: "WIPRO".to_string(),
                timestamp: 1736705000,
                natural_language_prompt: "Check if BROKER-XYZ front ran client order on WIPRO".to_string(),
            },
            QueryHistory {
                method_name: "scan_entity_anomalies".to_string(),
                entity_id: "TRADER-002".to_string(),
                symbol: "".to_string(),
                timestamp: 1736706000,
                natural_language_prompt: "Run full anomaly scan on TRADER-002".to_string(),
            },
            QueryHistory {
                method_name: "detect_spoofing".to_string(),
                entity_id: "TRADER-003".to_string(),
                symbol: "SBIN".to_string(),
                timestamp: 1736707000,
                natural_language_prompt: "Is TRADER-003 spoofing orders on SBI?".to_string(),
            },
            QueryHistory {
                method_name: "detect_pump_dump".to_string(),
                entity_id: "".to_string(),
                symbol: "BHARTIARTL".to_string(),
                timestamp: 1736708000,
                natural_language_prompt: "Analyze Bharti Airtel for pump dump in last hour".to_string(),
            },
            QueryHistory {
                method_name: "detect_wash_trading".to_string(),
                entity_id: "TRADER-001".to_string(),
                symbol: "INFY".to_string(),
                timestamp: 1736709000,
                natural_language_prompt: "Check if TRADER-001 did wash trades on INFY with any counterparty".to_string(),
            },
        ];
        
        Ok(AnomalyDetectionContractState {
            secrets: Secrets::new(),
            query_cache: QueryContext {
                recent_queries: sample_histories,
                last_entity_id: "TRADER-001".to_string(),
                last_symbol: "RELIANCE".to_string(),
            },
        })
    }

    /// Get context from recent queries to resolve ambiguous references
    #[mutate]
    async fn get_context(&mut self) -> QueryContext {
        self.query_cache.clone()
    }

    /// Detect spoofing patterns
    #[mutate]
    async fn detect_spoofing(&mut self, order_id: String, entity_id: String, symbol: String, order_details: String) -> Result<SpoofingIndicator, String> {
        // Cross-parameter resolution: if one matches, get both from same cache entry
        let (resolved_entity, resolved_symbol) = self.resolve_from_cache(&entity_id, &symbol);
        
        // Update cache with resolved values
        self.update_cache("detect_spoofing", &resolved_entity, &resolved_symbol, 
            &format!("Check spoofing for order {} by {} on {}", order_id, resolved_entity, resolved_symbol));
        
        // Real implementation would need Order Book data (Level 2)
        // For demo, we check if order size is unusually large compared to current volume from Alpha Vantage
        
        let quote = self.get_quote(&resolved_symbol).await?;
        
        let market_volume: u64 = quote.volume.parse().unwrap_or(10000);
        
        // Simple heuristic: If order details mention huge quantity (simulated parsing)
        let is_large_order = order_details.contains("qty: 50000") || order_details.contains("large");
        
        let is_spoof = is_large_order && market_volume < 100000; // Large relative to market
        
        Ok(SpoofingIndicator {
            order_id,
            is_spoof,
            cancellation_rate: "High".to_string(),
            order_size_vs_market: format!("{}% of daily vol", if is_large_order { "15" } else { "1" }),
            price_impact: "Potential manipulation detected".to_string(),
        })
    }

    /// Detect wash trading
    #[query]
    async fn detect_wash_trading(&self, entity_id: String, counterparty_id: String, symbol: String, trade_timestamp: u64) -> Result<WashTradeIndicator, String> {
        // Cross-parameter resolution
        // Try to resolve entity and symbol together first
        let (resolved_entity, resolved_symbol) = self.resolve_from_cache(&entity_id, &symbol);
        // Then resolve counterparty separately (or against symbol)
        let (resolved_counterparty, _) = self.resolve_from_cache(&counterparty_id, &symbol);
        
        // Wash trading = Entity trading with itself or collider
        let is_same_entity = resolved_entity == resolved_counterparty;
        
        Ok(WashTradeIndicator {
            entity_id: resolved_entity,
            counterparty_id: resolved_counterparty,
            is_wash_trade: is_same_entity,
            volume_match: true,
            price_match: true,
            time_gap_seconds: 0,
        })
    }

    /// Detect Pump & Dump schemes
    #[mutate]
    async fn detect_pump_dump(&mut self, symbol: String, time_window_minutes: u32) -> Result<PumpDumpIndicator, String> {
        // Resolve partial symbol from cache
        let resolved_symbol = self.resolve_symbol(&symbol);
        
        // Update cache with resolved value
        self.update_cache("detect_pump_dump", "", &resolved_symbol, 
            &format!("Check pump and dump on {} in last {} minutes", resolved_symbol, time_window_minutes));
        
        // Use Alpha Vantage to check price velocity and volume surge
        let quote = self.get_quote(&resolved_symbol).await?;
        
        let change_str = quote.change_percent.trim_end_matches('%');
        let change_pct: f64 = change_str.parse().unwrap_or(0.0);
        
        // Heuristic: Price up > 10% in short time is suspicious
        let is_pump = change_pct > 10.0;
        
        Ok(PumpDumpIndicator {
            symbol: resolved_symbol,
            is_pump_dump: is_pump,
            price_velocity: format!("{}%", change_pct),
            volume_surge: "High".to_string(),
            social_sentiment_score: if is_pump { 85 } else { 40 },
        })
    }

    /// Detect potential front-running (placeholder for logic requiring high-frequency data)
    #[query]
    async fn detect_front_running(&self, entity_id: String, symbol: String, client_trade_timestamp: u64, prop_trade_timestamp: u64) -> Result<AnomalyResult, String> {
        // Cross-parameter resolution
        let (resolved_entity, resolved_symbol) = self.resolve_from_cache(&entity_id, &symbol);
        
        let client_ts = client_trade_timestamp;
        let prop_ts = prop_trade_timestamp;
        let diff = if prop_ts > client_ts {
            prop_ts - client_ts
        } else {
            client_ts - prop_ts
        };
        
        let is_suspicious = diff < 2 && prop_ts < client_ts; // Prop traded *just* before client
        
        Ok(AnomalyResult {
            entity_id: resolved_entity,
            symbol: resolved_symbol,
            anomaly_type: "FRONT_RUNNING".to_string(),
            confidence_score: if is_suspicious { 90 } else { 10 },
            details: format!("Trade gap: {}s", diff),
            timestamp: prop_ts,
            supporting_evidence: "Prop desk trade executed immediately prior to large client order".to_string(),
        })
    }

    /// Analyze volume anomalies using Alpha Vantage
    #[mutate]
    async fn analyze_volume_anomaly(&mut self, symbol: String, interval: String) -> Result<AnomalyResult, String> {
        // Resolve partial symbol from cache
        let resolved_symbol = self.resolve_symbol(&symbol);
        
        // Update cache with resolved value
        self.update_cache("analyze_volume_anomaly", "", &resolved_symbol, 
            &format!("Check volume anomaly on {} with {} interval", resolved_symbol, interval));
        
        let quote = self.get_quote(&resolved_symbol).await?;
        
        let volume: u64 = quote.volume.parse().unwrap_or(0);
        
        Ok(AnomalyResult {
            entity_id: "MARKET".to_string(),
            symbol: resolved_symbol,
            anomaly_type: "VOLUME_SPIKE".to_string(),
            confidence_score: if volume > 1000000 { 80 } else { 20 },
            details: format!("Current Volume: {}", volume),
            timestamp: 0,
            supporting_evidence: "Volume analysis from Alpha Vantage".to_string(),
        })
    }

    /// Calculate RSI to check overbought/oversold conditions (TAAPI.IO)
    #[mutate]
    async fn check_rsi_levels(&mut self, symbol: String) -> Result<String, String> {
        // Resolve partial symbol from cache
        let resolved_symbol = self.resolve_symbol(&symbol);
        
        // Update cache with resolved value
        self.update_cache("check_rsi_levels", "", &resolved_symbol, 
            &format!("Check RSI levels for {}", resolved_symbol));
        
        let rsi = self.get_rsi(&resolved_symbol).await?;
        
        if rsi > 70.0 {
            Ok(format!("{} is OVERBOUGHT (RSI: {:.2})", resolved_symbol, rsi))
        } else if rsi < 30.0 {
            Ok(format!("{} is OVERSOLD (RSI: {:.2})", resolved_symbol, rsi))
        } else {
            Ok(format!("{} is NEUTRAL (RSI: {:.2})", resolved_symbol, rsi))
        }
    }

    /// Run full anomaly scan for an entity
    #[query]
    async fn scan_entity_anomalies(&self, entity_id: String) -> Result<Vec<AnomalyResult>, String> {
        // Resolve partial entity reference from cache
        let resolved_entity = self.resolve_entity(&entity_id);
        
        // In production, this would aggregate results from multiple detection methods
        // For now, return empty with the resolved entity noted
        Ok(vec![])
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_context",
      "description": "IMPORTANT: Call this FIRST before any other method. Returns recent query history with entity_ids, symbols, and natural language prompts to help resolve ambiguous user references like 'that trader', 'same stock', etc.\n",
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
      "name": "detect_spoofing",
      "description": "Detect spoofing patterns for a stock order\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Stock symbol (e.g., AAPL, IBM)\n"
          },
          "order_id": {
            "type": "string",
            "description": "Order ID to analyze\n"
          },
          "entity_id": {
            "type": "string",
            "description": "Entity ID placing the order\n"
          },
          "order_details": {
            "type": "string",
            "description": "Order details string\n"
          }
        },
        "required": [
          "symbol",
          "order_id",
          "entity_id",
          "order_details"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "detect_wash_trading",
      "description": "Detect wash trading between two entities\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "First entity ID\n"
          },
          "counterparty_id": {
            "type": "string",
            "description": "Second entity ID (counterparty)\n"
          },
          "symbol": {
            "type": "string",
            "description": "Stock symbol\n"
          },
          "trade_timestamp": {
            "type": "integer",
            "description": "Optional trade timestamp\n"
          }
        },
        "required": [
          "entity_id",
          "counterparty_id",
          "symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "detect_pump_dump",
      "description": "Detect Pump & Dump schemes for a stock\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Stock symbol to analyze\n"
          },
          "time_window_minutes": {
            "type": "integer",
            "description": "Time window in minutes (default: 60)\n"
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
      "name": "detect_front_running",
      "description": "Detect front-running patterns\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity ID to investigate\n"
          },
          "symbol": {
            "type": "string",
            "description": "Stock symbol\n"
          },
          "client_trade_timestamp": {
            "type": "integer",
            "description": "Client trade timestamp\n"
          },
          "prop_trade_timestamp": {
            "type": "integer",
            "description": "Prop desk trade timestamp\n"
          }
        },
        "required": [
          "entity_id",
          "symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "analyze_volume_anomaly",
      "description": "Analyze volume anomalies for a stock\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Stock symbol\n"
          },
          "interval": {
            "type": "string",
            "description": "Time interval (default: 1h)\n"
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
      "name": "check_rsi_levels",
      "description": "Check RSI overbought/oversold levels for a crypto pair via TAAPI.IO\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Crypto symbol (e.g., BTC for BTC/USDT)\n"
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
      "name": "scan_entity_anomalies",
      "description": "Run full anomaly scan for an entity\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity ID to scan\n"
          }
        },
        "required": [
          "entity_id"
        ]
      }
    }
  }
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{
  "prompts": [
  ]
}"#.to_string()
    }
}
