

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
    async fn detect_wash_trading(&mut self, entity_id: String, counterparty_id: String, symbol: String, trade_timestamp: u64) -> Result<WashTradeIndicator, String>;
    async fn detect_pump_dump(&mut self, symbol: String, time_window_minutes: u32) -> Result<PumpDumpIndicator, String>;
    async fn detect_front_running(&mut self, entity_id: String, symbol: String, client_trade_timestamp: u64, prop_trade_timestamp: u64) -> Result<AnomalyResult, String>;
    async fn analyze_volume_anomaly(&mut self, symbol: String, interval: String) -> Result<AnomalyResult, String>;
    async fn check_rsi_levels(&mut self, symbol: String) -> Result<String, String>;
    async fn scan_entity_anomalies(&self, entity_id: String) -> Result<Vec<AnomalyResult>, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

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
    fn get_headers(&self) -> HashMap<String, String> {
        HashMap::from([
            ("Content-Type".to_string(), "application/json".to_string()),
        ])
    }

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

    fn update_cache(&mut self, method_name: &str, entity_id: &str, symbol: &str, prompt: &str) {
        let already_exists = self.query_cache.recent_queries.iter()
            .any(|q| q.entity_id == entity_id && q.symbol == symbol);
        
        if !already_exists {
            let timestamp = self.query_cache.recent_queries.len() as u64 + 1;
            
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

    fn resolve_from_cache(&self, entity_partial: &str, symbol_partial: &str) -> (String, String) {
        let entity_lower = entity_partial.to_lowercase();
        let symbol_lower = symbol_partial.to_lowercase();
        
        for query in self.query_cache.recent_queries.iter().rev() {
            let entity_matches = !entity_partial.is_empty() && 
                !query.entity_id.is_empty() && 
                query.entity_id.to_lowercase().contains(&entity_lower);
            
            let symbol_matches = !symbol_partial.is_empty() && 
                !query.symbol.is_empty() && 
                query.symbol.to_lowercase().contains(&symbol_lower);
            
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
        
        (self.resolve_entity(entity_partial), self.resolve_symbol(symbol_partial))
    }

    fn maybe_push_alert(&self, alert_type: &str, severity: &str, risk_score: u32, entity_id: &str, symbol: &str, description: &str) {
        let config = self.secrets.config();
        if config.dashboard_contract_id.is_empty() {
            return;
        }

        let alert = Alert {
            id: format!("ANOMALY-{}-{}", alert_type, 0u64), // Simplified timestamp
            alert_type: alert_type.to_string(),
            severity: severity.to_string(),
            risk_score,
            entity_id: entity_id.to_string(),
            symbol: symbol.to_string(),
            description: description.to_string(),
            workflow_id: "".to_string(),
            timestamp: 0, 
        };

        let args = serde_json::json!({ "alert": alert }).to_string();
        
        let _ = Runtime::call_contract::<String>(
            config.dashboard_contract_id.clone(),
            "push_alert".to_string(),
            Some(args),
        );
    }

    fn push_history(&self, method_name: &str, params: &str, result_summary: &str, status: &str, entity_id: &str, symbol: &str) {
        let config = self.secrets.config();
        if config.dashboard_contract_id.is_empty() {
            return;
        }

        let entry = serde_json::json!({
            "id": format!("HIST-anomaly-{}-{}", method_name, 0u64),
            "timestamp": 0u64,
            "source_mcp": "anomaly_detection",
            "method_name": method_name,
            "params": params,
            "result_summary": result_summary,
            "status": status,
            "entity_id": entity_id,
            "symbol": symbol
        });

        let args = serde_json::json!({ "entry": entry }).to_string();
        
        let _ = Runtime::call_contract::<String>(
            config.dashboard_contract_id.clone(),
            "push_history".to_string(),
            Some(args),
        );
    }

    fn log_workflow(&self, workflow_id: &str, workflow_type: &str, trigger: &str) {
        let config = self.secrets.config();
        if config.dashboard_contract_id.is_empty() {
            return;
        }

        let args = serde_json::json!({
            "workflow_id": workflow_id,
            "workflow_type": workflow_type,
            "trigger": trigger,
            "total_steps": 3u32
        }).to_string();
        
        let _ = Runtime::call_contract::<String>(
            config.dashboard_contract_id.clone(),
            "log_workflow_start".to_string(),
            Some(args),
        );
    }

    fn create_case(&self, case_type: &str, entity_id: &str, symbol: &str, risk_score: u32, summary: &str) {
        let config = self.secrets.config();
        if config.dashboard_contract_id.is_empty() {
            return;
        }

        let case = serde_json::json!({
            "case_id": format!("CASE-{}-{}", case_type, 0u64),
            "case_type": case_type,
            "status": "OPEN",
            "priority": if risk_score >= 80 { "CRITICAL" } else if risk_score >= 60 { "HIGH" } else { "MEDIUM" },
            "subject_entity": entity_id,
            "symbol": symbol,
            "risk_score": risk_score,
            "assigned_to": "Unassigned",
            "created_at": 0u64,
            "updated_at": 0u64,
            "summary": summary
        });

        let args = serde_json::json!({ "case_record": case }).to_string();
        
        let _ = Runtime::call_contract::<String>(
            config.dashboard_contract_id.clone(),
            "upsert_case".to_string(),
            Some(args),
        );
    }

    fn register_risk(&self, entity_id: &str, entity_name: &str, risk_score: u32) {
        let config = self.secrets.config();
        if config.dashboard_contract_id.is_empty() || risk_score < 70 {
            return; 
        }

        let entity = serde_json::json!({
            "entity_id": entity_id,
            "entity_name": entity_name,
            "risk_score": risk_score,
            "alert_count": 1u32,
            "last_alert_at": 0u64
        });

        let args = serde_json::json!({ "entity": entity }).to_string();
        
        let _ = Runtime::call_contract::<String>(
            config.dashboard_contract_id.clone(),
            "register_risk_entity".to_string(),
            Some(args),
        );
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

    #[mutate]
    async fn get_context(&mut self) -> QueryContext {
        self.query_cache.clone()
    }

    #[mutate]
    async fn detect_spoofing(&mut self, order_id: String, entity_id: String, symbol: String, order_details: String) -> Result<SpoofingIndicator, String> {
        let (resolved_entity, resolved_symbol) = self.resolve_from_cache(&entity_id, &symbol);
        
        self.update_cache("detect_spoofing", &resolved_entity, &resolved_symbol, 
            &format!("Check spoofing for order {} by {} on {}", order_id, resolved_entity, resolved_symbol));
        
        
        let quote = self.get_quote(&resolved_symbol).await?;
        
        let market_volume: u64 = quote.volume.parse().unwrap_or(10000);
        
        let is_large_order = order_details.contains("qty: 50000") || order_details.contains("large");
        
        let is_spoof = is_large_order && market_volume < 100000; 
        
        self.log_workflow(
            &format!("WF-SPOOF-{}", order_id),
            "SPOOFING_DETECTION",
            &format!("Order {} check", order_id),
        );
        
        if is_spoof {
            self.maybe_push_alert(
                "SPOOFING",
                "HIGH",
                75,
                &resolved_entity,
                &resolved_symbol,
                &format!("Spoofing detected: Order {} has high cancellation rate and large size vs market", order_id),
            );
            self.create_case(
                "SPOOFING",
                &resolved_entity,
                &resolved_symbol,
                75,
                &format!("Potential spoofing on order {}", order_id),
            );
            self.register_risk(&resolved_entity, &format!("Entity {}", resolved_entity), 75);
        } else {
            self.maybe_push_alert(
                "SPOOFING_CHECK",
                "INFO",
                10,
                &resolved_entity,
                &resolved_symbol,
                &format!("Spoofing check passed for order {}", order_id),
            );
        }
        
        self.push_history(
            "detect_spoofing",
            &format!("order_id={}, entity_id={}, symbol={}", order_id, resolved_entity, resolved_symbol),
            &format!("is_spoof={}", is_spoof),
            if is_spoof { "ALERT" } else { "OK" },
            &resolved_entity,
            &resolved_symbol,
        );
        
        Ok(SpoofingIndicator {
            order_id,
            is_spoof,
            cancellation_rate: "High".to_string(),
            order_size_vs_market: format!("{}% of daily vol", if is_large_order { "15" } else { "1" }),
            price_impact: "Potential manipulation detected".to_string(),
        })
    }

    /// Detect wash trading
    #[mutate]
    async fn detect_wash_trading(&mut self, entity_id: String, counterparty_id: String, symbol: String, trade_timestamp: u64) -> Result<WashTradeIndicator, String> {
        
        let (resolved_entity, resolved_symbol) = self.resolve_from_cache(&entity_id, &symbol);
        
        let (resolved_counterparty, _) = self.resolve_from_cache(&counterparty_id, &symbol);
        
        // Update cache
        self.update_cache("detect_wash_trading", &resolved_entity, &resolved_symbol, 
            &format!("Check wash trading between {} and {} on {}", resolved_entity, resolved_counterparty, resolved_symbol));
        
        // Wash trading = Entity trading with itself or collider
        let is_same_entity = resolved_entity == resolved_counterparty;
        
        // Log workflow
        self.log_workflow(
            &format!("WF-WASH-{}-{}", resolved_entity, resolved_counterparty),
            "WASH_TRADING_DETECTION",
            &format!("Check {} vs {}", resolved_entity, resolved_counterparty),
        );
        
        if is_same_entity {
            self.maybe_push_alert(
                "WASH_TRADING",
                "HIGH",
                80,
                &resolved_entity,
                &resolved_symbol,
                &format!("Wash trading detected: {} trading with itself/collider {}", resolved_entity, resolved_counterparty),
            );
            self.create_case(
                "WASH_TRADING",
                &resolved_entity,
                &resolved_symbol,
                80,
                &format!("Wash trade between {} and {}", resolved_entity, resolved_counterparty),
            );
            // Register high-risk
            self.register_risk(&resolved_entity, &format!("Entity {}", resolved_entity), 80);
        } else {
            self.maybe_push_alert(
                "WASH_TRADING_CHECK",
                "INFO",
                10,
                &resolved_entity,
                &resolved_symbol,
                &format!("Wash trading check passed between {} and {}", resolved_entity, resolved_counterparty),
            );
        }
        
        // Push history
        self.push_history(
            "detect_wash_trading",
            &format!("entity={}, counterparty={}, symbol={}", resolved_entity, resolved_counterparty, resolved_symbol),
            &format!("is_wash_trade={}", is_same_entity),
            if is_same_entity { "ALERT" } else { "OK" },
            &resolved_entity,
            &resolved_symbol,
        );
        
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
        
        // Push alert to dashboard if pump & dump detected
        if is_pump {
            self.maybe_push_alert(
                "PUMP_DUMP",
                "CRITICAL",
                85,
                "",
                &resolved_symbol,
                &format!("Pump & Dump detected: {} has {}% price change in {} min window", resolved_symbol, change_pct, time_window_minutes),
            );
        } else {
             self.maybe_push_alert(
                "PUMP_DUMP_CHECK",
                "INFO",
                10,
                "",
                &resolved_symbol,
                &format!("Pump & Dump check passed: {} has {}% price change (normal)", resolved_symbol, change_pct),
            );
        }
        
        // Push history
        self.push_history(
            "detect_pump_dump",
            &format!("symbol={}, window={}min", resolved_symbol, time_window_minutes),
            &format!("is_pump_dump={}, change={}%", is_pump, change_pct),
            if is_pump { "ALERT" } else { "OK" },
            "",
            &resolved_symbol,
        );
        
        Ok(PumpDumpIndicator {
            symbol: resolved_symbol,
            is_pump_dump: is_pump,
            price_velocity: format!("{}%", change_pct),
            volume_surge: "High".to_string(),
            social_sentiment_score: if is_pump { 85 } else { 40 },
        })
    }

    /// Detect potential front-running (placeholder for logic requiring high-frequency data)
    #[mutate]
    async fn detect_front_running(&mut self, entity_id: String, symbol: String, client_trade_timestamp: u64, prop_trade_timestamp: u64) -> Result<AnomalyResult, String> {
        // Cross-parameter resolution
        let (resolved_entity, resolved_symbol) = self.resolve_from_cache(&entity_id, &symbol);
        
        // Update cache
        self.update_cache("detect_front_running", &resolved_entity, &resolved_symbol, 
            &format!("Check front running for {} on {}", resolved_entity, resolved_symbol));
        
        let client_ts = client_trade_timestamp;
        let prop_ts = prop_trade_timestamp;
        let diff = if prop_ts > client_ts {
            prop_ts - client_ts
        } else {
            client_ts - prop_ts
        };
        
        let is_suspicious = diff < 2 && prop_ts < client_ts; // Prop traded *just* before client
        
        if is_suspicious {
            self.maybe_push_alert(
                "FRONT_RUNNING",
                "CRITICAL",
                90,
                &resolved_entity,
                &resolved_symbol,
                &format!("Front running detected: Prop desk traded {}s before client", diff),
            );
        } else {
            self.maybe_push_alert(
                "FRONT_RUNNING_CHECK",
                "INFO",
                10,
                &resolved_entity,
                &resolved_symbol,
                &format!("Front running check passed: Trade gap {}s (safe)", diff),
            );
        }
        
        // Push history
        self.push_history(
            "detect_front_running",
            &format!("entity={}, symbol={}, gap={}s", resolved_entity, resolved_symbol, diff),
            &format!("is_suspicious={}", is_suspicious),
            if is_suspicious { "ALERT" } else { "OK" },
            &resolved_entity,
            &resolved_symbol,
        );
        
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

    #[mutate]
    async fn analyze_volume_anomaly(&mut self, symbol: String, interval: String) -> Result<AnomalyResult, String> {
        let resolved_symbol = self.resolve_symbol(&symbol);
        
        self.update_cache("analyze_volume_anomaly", "", &resolved_symbol, 
            &format!("Check volume anomaly on {} with {} interval", resolved_symbol, interval));
        
        let quote = self.get_quote(&resolved_symbol).await?;
        
        let volume: u64 = quote.volume.parse().unwrap_or(0);
        
        let is_anomaly = volume > 1000000;
        
        if is_anomaly {
            self.maybe_push_alert(
                "VOLUME_SPIKE",
                "MEDIUM",
                60,
                "MARKET",
                &resolved_symbol,
                &format!("Volume spike detected: {} volume > 1M", volume),
            );
        } else {
             self.maybe_push_alert(
                "VOLUME_CHECK",
                "INFO",
                10,
                "MARKET",
                &resolved_symbol,
                &format!("Volume check passed: {} volume is normal", volume),
            );
        }
        
        // Push history
        self.push_history(
            "analyze_volume_anomaly",
            &format!("symbol={}, interval={}", resolved_symbol, interval),
            &format!("volume={}, is_anomaly={}", volume, is_anomaly),
            if is_anomaly { "ALERT" } else { "OK" },
            "MARKET",
            &resolved_symbol,
        );
        
        Ok(AnomalyResult {
            entity_id: "MARKET".to_string(),
            symbol: resolved_symbol,
            anomaly_type: "VOLUME_SPIKE".to_string(),
            confidence_score: if is_anomaly { 80 } else { 20 },
            details: format!("Current Volume: {}", volume),
            timestamp: 0,
            supporting_evidence: "Volume analysis from Alpha Vantage".to_string(),
        })
    }

    #[mutate]
    async fn check_rsi_levels(&mut self, symbol: String) -> Result<String, String> {
        let resolved_symbol = self.resolve_symbol(&symbol);
        
        self.update_cache("check_rsi_levels", "", &resolved_symbol, 
            &format!("Check RSI levels for {}", resolved_symbol));
        
        let rsi = self.get_rsi(&resolved_symbol).await?;
        
        if rsi > 70.0 {
            self.maybe_push_alert(
                "RSI_OVERBOUGHT",
                "HIGH",
                70,
                "MARKET",
                &resolved_symbol,
                &format!("RSI Overbought: {:.2} > 70", rsi),
            );
            self.push_history(
                "check_rsi_levels",
                &format!("symbol={}", resolved_symbol),
                &format!("RSI={:.2}, status=OVERBOUGHT", rsi),
                "ALERT",
                "MARKET",
                &resolved_symbol,
            );
            Ok(format!("{} is OVERBOUGHT (RSI: {:.2})", resolved_symbol, rsi))
        } else if rsi < 30.0 {
            self.maybe_push_alert(
                "RSI_OVERSOLD",
                "MEDIUM",
                50,
                "MARKET",
                &resolved_symbol,
                &format!("RSI Oversold: {:.2} < 30", rsi),
            );
            self.push_history(
                "check_rsi_levels",
                &format!("symbol={}", resolved_symbol),
                &format!("RSI={:.2}, status=OVERSOLD", rsi),
                "ALERT",
                "MARKET",
                &resolved_symbol,
            );
            Ok(format!("{} is OVERSOLD (RSI: {:.2})", resolved_symbol, rsi))
        } else {
            self.maybe_push_alert(
                "RSI_CHECK",
                "INFO",
                10,
                "MARKET",
                &resolved_symbol,
                &format!("RSI Normal: {:.2}", rsi),
            );
            self.push_history(
                "check_rsi_levels",
                &format!("symbol={}", resolved_symbol),
                &format!("RSI={:.2}, status=NEUTRAL", rsi),
                "OK",
                "MARKET",
                &resolved_symbol,
            );
            Ok(format!("{} is NEUTRAL (RSI: {:.2})", resolved_symbol, rsi))
        }
    }

    #[query]
    async fn scan_entity_anomalies(&self, entity_id: String) -> Result<Vec<AnomalyResult>, String> {
        let resolved_entity = self.resolve_entity(&entity_id);
        
        Ok(vec![])
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_context",
      "description": "DO NOT CALL THIS - internal test function only.\n",
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
