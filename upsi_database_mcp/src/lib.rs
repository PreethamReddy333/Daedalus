//! # UPSI Database MCP Server
//!
//! Tracks Unpublished Price Sensitive Information (UPSI) using Supabase (PostgreSQL).
//! Logs access to UPSI and monitors trading window status.
//!
//! ## External Service: Supabase (PostgreSQL + REST API)
//! - Stores UPSI records, access logs, and trading window status
//! - Accessible via REST API with Row Level Security
//!
//! ## Context Cache Feature:
//! - Implements fuzzy resolution for entity_id, company_symbol, and upsi_id
//! - Cross-parameter resolution: if one param matches cache, related params are auto-filled
//! - Helps Icarus resolve ambiguous prompts like "that company", "same person", etc.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use weil_macros::{constructor, mutate, query, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::http::{HttpClient, HttpMethod};
use weil_rs::runtime::Runtime;

// ===== CONFIGURATION =====

#[derive(Debug, Serialize, Deserialize, WeilType, Default, Clone)]
pub struct UPSIDatabaseConfig {
    pub dashboard_contract_id: String,
    pub supabase_url: String,
    pub supabase_anon_key: String,
}

// ===== DATA STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct UPSIRecord {
    pub upsi_id: String,
    pub company_symbol: String,
    pub upsi_type: String,
    pub description: String,
    pub nature: String,
    pub created_date: u64,
    pub public_date: u64,
    pub is_public: bool,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct UPSIAccessLog {
    pub access_id: String,
    pub upsi_id: String,
    pub accessor_entity_id: String,
    pub accessor_name: String,
    pub accessor_designation: String,
    pub access_timestamp: u64,
    pub access_reason: String,
    pub access_mode: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct TradingWindowStatus {
    pub company_symbol: String,
    pub window_status: String,
    pub closure_reason: String,
    pub closure_start: u64,
    pub expected_opening: u64,
}

// ===== CONTEXT CACHE STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone, Default)]
pub struct QueryHistory {
    pub method_name: String,
    pub entity_id: String,
    pub company_symbol: String,
    pub upsi_id: String,
    pub timestamp: u64,
    pub natural_language_prompt: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone, Default)]
pub struct QueryContext {
    pub recent_queries: Vec<QueryHistory>,
    pub last_entity_id: String,
    pub last_company_symbol: String,
    pub last_upsi_id: String,
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

// ===== TRAIT DEFINITION =====

trait UPSIDatabase {
    fn new() -> Result<Self, String> where Self: Sized;
    async fn get_context(&mut self) -> QueryContext;
    async fn get_upsi(&mut self, upsi_id: String) -> Result<UPSIRecord, String>;
    async fn get_active_upsi(&mut self, company_symbol: String) -> Result<Vec<UPSIRecord>, String>;
    async fn get_upsi_access_log(&mut self, upsi_id: String, from_timestamp: u64, to_timestamp: u64) -> Result<Vec<UPSIAccessLog>, String>;
    async fn get_access_by_person(&mut self, accessor_entity_id: String, days_back: u32) -> Result<Vec<UPSIAccessLog>, String>;
    async fn check_upsi_access_before(&mut self, entity_id: String, company_symbol: String, before_timestamp: u64) -> Result<Vec<UPSIAccessLog>, String>;
    async fn get_trading_window(&mut self, company_symbol: String) -> Result<TradingWindowStatus, String>;
    async fn check_window_violation(&mut self, entity_id: String, company_symbol: String, trade_timestamp: u64) -> Result<bool, String>;
    async fn get_upsi_accessors(&mut self, upsi_id: String) -> Result<Vec<UPSIAccessLog>, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

// ===== CONTRACT STATE =====

#[derive(Serialize, Deserialize, WeilType)]
pub struct UPSIDatabaseContractState {
    secrets: Secrets<UPSIDatabaseConfig>,
    query_cache: QueryContext,
}

// ===== HELPER METHODS =====

impl UPSIDatabaseContractState {
    /// Helper to make HTTP requests to Supabase
    async fn supabase_request<T: for<'de> Deserialize<'de>>(&self, endpoint: &str, method: HttpMethod, body: Option<String>) -> Result<T, String> {
        let config = self.secrets.config();
        let url = format!("{}/rest/v1/{}", config.supabase_url, endpoint);
        
        let headers = HashMap::from([
            ("apikey".to_string(), config.supabase_anon_key.clone()),
            ("Authorization".to_string(), format!("Bearer {}", config.supabase_anon_key)),
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Prefer".to_string(), "return=representation".to_string()),
        ]);
        
        let mut req = HttpClient::request(&url, method)
            .headers(headers);
            
        if let Some(b) = body {
            req = req.body(b);
        }
        
        let response = req.send().map_err(|e| format!("Supabase request failed: {:?}", e))?;
        let response_text = response.text();
        
        serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse Supabase response: {} - Body: {}", e, response_text))
    }

    /// Update the query cache with a new query entry (only if unique)
    fn update_cache(&mut self, method_name: &str, entity_id: &str, company_symbol: &str, upsi_id: &str, prompt: &str) {
        // Check if this entity+company+upsi combination already exists in cache
        let already_exists = self.query_cache.recent_queries.iter()
            .any(|q| q.entity_id == entity_id && q.company_symbol == company_symbol && q.upsi_id == upsi_id);
        
        // Only add to cache if it's a NEW unique combination
        if !already_exists && (!entity_id.is_empty() || !company_symbol.is_empty() || !upsi_id.is_empty()) {
            let timestamp = self.query_cache.recent_queries.len() as u64 + 1;
            
            // Keep last 10 entries
            if self.query_cache.recent_queries.len() >= 10 {
                self.query_cache.recent_queries.remove(0);
            }
            self.query_cache.recent_queries.push(QueryHistory {
                method_name: method_name.to_string(),
                entity_id: entity_id.to_string(),
                company_symbol: company_symbol.to_string(),
                upsi_id: upsi_id.to_string(),
                timestamp,
                natural_language_prompt: prompt.to_string(),
            });
        }
        
        // Always update last values for recency tracking
        if !entity_id.is_empty() {
            self.query_cache.last_entity_id = entity_id.to_string();
        }
        if !company_symbol.is_empty() {
            self.query_cache.last_company_symbol = company_symbol.to_string();
        }
        if !upsi_id.is_empty() {
            self.query_cache.last_upsi_id = upsi_id.to_string();
        }
    }

    /// Resolve a partial entity reference from cache using fuzzy matching
    /// "Mukesh" → "ENT-REL-001", "SUS" → "SUS-001"
    fn resolve_entity(&self, partial: &str) -> String {
        if partial.is_empty() {
            return self.query_cache.last_entity_id.clone();
        }
        
        let partial_lower = partial.to_lowercase();
        
        // First check last entity
        if self.query_cache.last_entity_id.to_lowercase().contains(&partial_lower) {
            return self.query_cache.last_entity_id.clone();
        }
        
        // Search through cached queries for fuzzy match
        for query in self.query_cache.recent_queries.iter().rev() {
            if !query.entity_id.is_empty() && query.entity_id.to_lowercase().contains(&partial_lower) {
                return query.entity_id.clone();
            }
            // Also check natural language prompt
            if query.natural_language_prompt.to_lowercase().contains(&partial_lower) {
                if !query.entity_id.is_empty() {
                    return query.entity_id.clone();
                }
            }
        }
        
        partial.to_string()
    }

    /// Resolve a partial company symbol from cache using fuzzy matching
    /// "RELI" → "RELIANCE", "TCS" → "TCS"
    fn resolve_company(&self, partial: &str) -> String {
        if partial.is_empty() {
            return self.query_cache.last_company_symbol.clone();
        }
        
        let partial_lower = partial.to_lowercase();
        
        // First check last company
        if self.query_cache.last_company_symbol.to_lowercase().contains(&partial_lower) {
            return self.query_cache.last_company_symbol.clone();
        }
        
        // Search through cached queries
        for query in self.query_cache.recent_queries.iter().rev() {
            if !query.company_symbol.is_empty() && query.company_symbol.to_lowercase().contains(&partial_lower) {
                return query.company_symbol.clone();
            }
        }
        
        partial.to_string()
    }

    /// Resolve a partial UPSI ID from cache
    /// "001" → "UPSI-001", "merger" → "UPSI-002" (if prompt mentioned merger)
    fn resolve_upsi_id(&self, partial: &str) -> String {
        if partial.is_empty() {
            return self.query_cache.last_upsi_id.clone();
        }
        
        let partial_lower = partial.to_lowercase();
        
        // First check last upsi_id
        if self.query_cache.last_upsi_id.to_lowercase().contains(&partial_lower) {
            return self.query_cache.last_upsi_id.clone();
        }
        
        // Search through cached queries
        for query in self.query_cache.recent_queries.iter().rev() {
            if !query.upsi_id.is_empty() && query.upsi_id.to_lowercase().contains(&partial_lower) {
                return query.upsi_id.clone();
            }
            // Check prompt for context
            if query.natural_language_prompt.to_lowercase().contains(&partial_lower) {
                if !query.upsi_id.is_empty() {
                    return query.upsi_id.clone();
                }
            }
        }
        
        partial.to_string()
    }

    /// Cross-parameter resolution: If ONE param matches cache, return ALL related params from that entry
    /// "RELIANCE" → returns (entity_id from cache, "RELIANCE", upsi_id from cache)
    fn resolve_from_cache(&self, entity_partial: &str, company_partial: &str, upsi_partial: &str) -> (String, String, String) {
        let entity_lower = entity_partial.to_lowercase();
        let company_lower = company_partial.to_lowercase();
        let upsi_lower = upsi_partial.to_lowercase();
        
        // Search through cached queries for a match on ANY param
        for query in self.query_cache.recent_queries.iter().rev() {
            let entity_matches = !entity_partial.is_empty() && 
                !query.entity_id.is_empty() && 
                query.entity_id.to_lowercase().contains(&entity_lower);
            
            let company_matches = !company_partial.is_empty() && 
                !query.company_symbol.is_empty() && 
                query.company_symbol.to_lowercase().contains(&company_lower);
            
            let upsi_matches = !upsi_partial.is_empty() && 
                !query.upsi_id.is_empty() && 
                query.upsi_id.to_lowercase().contains(&upsi_lower);
            
            // If ANY matches, return ALL from this cache entry
            if entity_matches || company_matches || upsi_matches {
                let resolved_entity = if query.entity_id.is_empty() {
                    self.resolve_entity(entity_partial)
                } else {
                    query.entity_id.clone()
                };
                
                let resolved_company = if query.company_symbol.is_empty() {
                    self.resolve_company(company_partial)
                } else {
                    query.company_symbol.clone()
                };
                
                let resolved_upsi = if query.upsi_id.is_empty() {
                    self.resolve_upsi_id(upsi_partial)
                } else {
                    query.upsi_id.clone()
                };
                
                return (resolved_entity, resolved_company, resolved_upsi);
            }
            
            // Also check natural language prompt
            let prompt_lower = query.natural_language_prompt.to_lowercase();
            if (!entity_partial.is_empty() && prompt_lower.contains(&entity_lower)) ||
               (!company_partial.is_empty() && prompt_lower.contains(&company_lower)) ||
               (!upsi_partial.is_empty() && prompt_lower.contains(&upsi_lower)) {
                return (
                    if query.entity_id.is_empty() { self.resolve_entity(entity_partial) } else { query.entity_id.clone() },
                    if query.company_symbol.is_empty() { self.resolve_company(company_partial) } else { query.company_symbol.clone() },
                    if query.upsi_id.is_empty() { self.resolve_upsi_id(upsi_partial) } else { query.upsi_id.clone() },
                );
            }
        }
        
        // No cross-match found, fall back to individual resolution
        (self.resolve_entity(entity_partial), self.resolve_company(company_partial), self.resolve_upsi_id(upsi_partial))
    }

    /// Push alert to surveillance dashboard via cross-contract call
    fn maybe_push_alert(&self, alert_type: &str, severity: &str, risk_score: u32, entity_id: &str, symbol: &str, description: &str) {
        let config = self.secrets.config();
        if config.dashboard_contract_id.is_empty() {
            return;
        }

        let alert = Alert {
            id: format!("UPSI-{}", 0u64),
            alert_type: alert_type.to_string(),
            severity: severity.to_string(),
            risk_score,
            entity_id: entity_id.to_string(),
            symbol: symbol.to_string(),
            description: description.to_string(),
            workflow_id: "".to_string(),
            timestamp: 0,
        };

        let args = serde_json::to_string(&alert).unwrap_or_default();
        let _ = Runtime::call_contract::<String>(
            config.dashboard_contract_id.clone(),
            "push_alert".to_string(),
            Some(args),
        );
    }
}

// ===== CONTRACT IMPLEMENTATION =====

#[smart_contract]
impl UPSIDatabase for UPSIDatabaseContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        // Initialize with sample query histories for testing context resolution
        let sample_histories = vec![
            QueryHistory {
                method_name: "get_active_upsi".to_string(),
                entity_id: "ENT-REL-001".to_string(),
                company_symbol: "RELIANCE".to_string(),
                upsi_id: "UPSI-001".to_string(),
                timestamp: 1,
                natural_language_prompt: "Check UPSI for Mukesh Ambani on RELIANCE".to_string(),
            },
            QueryHistory {
                method_name: "get_trading_window".to_string(),
                entity_id: "".to_string(),
                company_symbol: "INFY".to_string(),
                upsi_id: "UPSI-003".to_string(),
                timestamp: 2,
                natural_language_prompt: "Is INFY trading window open?".to_string(),
            },
            QueryHistory {
                method_name: "check_upsi_access_before".to_string(),
                entity_id: "SUS-001".to_string(),
                company_symbol: "RELIANCE".to_string(),
                upsi_id: "UPSI-002".to_string(),
                timestamp: 3,
                natural_language_prompt: "Did suspect SUS-001 access RELIANCE UPSI before trading?".to_string(),
            },
            QueryHistory {
                method_name: "get_access_by_person".to_string(),
                entity_id: "ENT-REL-006".to_string(),
                company_symbol: "RELIANCE".to_string(),
                upsi_id: "".to_string(),
                timestamp: 4,
                natural_language_prompt: "What UPSI did Reliance CFO access?".to_string(),
            },
            QueryHistory {
                method_name: "get_trading_window".to_string(),
                entity_id: "".to_string(),
                company_symbol: "TCS".to_string(),
                upsi_id: "".to_string(),
                timestamp: 5,
                natural_language_prompt: "Check TCS trading window status".to_string(),
            },
        ];
        
        Ok(UPSIDatabaseContractState {
            secrets: Secrets::new(),
            query_cache: QueryContext {
                recent_queries: sample_histories,
                last_entity_id: "ENT-REL-001".to_string(),
                last_company_symbol: "RELIANCE".to_string(),
                last_upsi_id: "UPSI-001".to_string(),
            },
        })
    }

    /// Get context from recent queries to resolve ambiguous references
    #[mutate]
    async fn get_context(&mut self) -> QueryContext {
        self.query_cache.clone()
    }

    /// Get UPSI record by ID from Supabase
    #[mutate]
    async fn get_upsi(&mut self, upsi_id: String) -> Result<UPSIRecord, String> {
        // Resolve partial UPSI ID
        let resolved_upsi = self.resolve_upsi_id(&upsi_id);
        
        // Update cache
        self.update_cache("get_upsi", "", "", &resolved_upsi, 
            &format!("Get UPSI record {}", resolved_upsi));
        
        let endpoint = format!("upsi_records?upsi_id=eq.{}&select=*", resolved_upsi);
        
        let records: Vec<UPSIRecord> = self.supabase_request(&endpoint, HttpMethod::Get, None).await?;
        
        records.into_iter().next().ok_or_else(|| format!("UPSI record {} not found", resolved_upsi))
    }

    /// Get all active (non-public) UPSI for a company
    #[mutate]
    async fn get_active_upsi(&mut self, company_symbol: String) -> Result<Vec<UPSIRecord>, String> {
        // Resolve partial company symbol
        let resolved_company = self.resolve_company(&company_symbol);
        
        // Update cache
        self.update_cache("get_active_upsi", "", &resolved_company, "", 
            &format!("Get active UPSI for {}", resolved_company));
        
        let endpoint = format!("upsi_records?company_symbol=eq.{}&is_public=eq.false&select=*", resolved_company);
        
        self.supabase_request(&endpoint, HttpMethod::Get, None).await
    }

    /// Get UPSI access log for a specific UPSI
    #[mutate]
    async fn get_upsi_access_log(&mut self, upsi_id: String, from_timestamp: u64, to_timestamp: u64) -> Result<Vec<UPSIAccessLog>, String> {
        // Resolve partial UPSI ID
        let resolved_upsi = self.resolve_upsi_id(&upsi_id);
        
        // Update cache
        self.update_cache("get_upsi_access_log", "", "", &resolved_upsi, 
            &format!("Get access log for UPSI {}", resolved_upsi));
        
        let endpoint = format!(
            "upsi_access_log?upsi_id=eq.{}&access_timestamp=gte.{}&access_timestamp=lte.{}&select=*",
            resolved_upsi, from_timestamp, to_timestamp
        );
        
        self.supabase_request(&endpoint, HttpMethod::Get, None).await
    }

    /// Get all UPSI accesses by a specific person
    #[mutate]
    async fn get_access_by_person(&mut self, accessor_entity_id: String, days_back: u32) -> Result<Vec<UPSIAccessLog>, String> {
        // Resolve partial entity ID
        let resolved_entity = self.resolve_entity(&accessor_entity_id);
        
        // Update cache
        self.update_cache("get_access_by_person", &resolved_entity, "", "", 
            &format!("Get UPSI accesses by {}", resolved_entity));
        
        let now = 1735689600u64;
        let days_in_seconds = days_back as u64 * 86400;
        let start_time = if now > days_in_seconds { now - days_in_seconds } else { 0 };

        let endpoint = format!(
            "upsi_access_log?accessor_entity_id=eq.{}&access_timestamp=gte.{}&select=*",
            resolved_entity, start_time
        );
        
        self.supabase_request(&endpoint, HttpMethod::Get, None).await
    }

    /// Check if an entity had UPSI access before a date
    #[mutate]
    async fn check_upsi_access_before(&mut self, entity_id: String, company_symbol: String, before_timestamp: u64) -> Result<Vec<UPSIAccessLog>, String> {
        // Cross-parameter resolution
        let (resolved_entity, resolved_company, _) = self.resolve_from_cache(&entity_id, &company_symbol, "");
        
        // Update cache
        self.update_cache("check_upsi_access_before", &resolved_entity, &resolved_company, "", 
            &format!("Check if {} accessed {} UPSI before trading", resolved_entity, resolved_company));
        
        let endpoint_logs = format!(
            "upsi_access_log?accessor_entity_id=eq.{}&access_timestamp=lt.{}&select=*",
            resolved_entity, before_timestamp
        );
        let logs: Vec<UPSIAccessLog> = self.supabase_request(&endpoint_logs, HttpMethod::Get, None).await?;
        
        let mut relevant_logs = Vec::new();
        
        for log in logs {
            let record = self.get_upsi(log.upsi_id.clone()).await;
            if let Ok(r) = record {
                if r.company_symbol == resolved_company {
                    relevant_logs.push(log);
                }
            }
        }
        
        Ok(relevant_logs)
    }

    /// Get trading window status for a company
    #[mutate]
    async fn get_trading_window(&mut self, company_symbol: String) -> Result<TradingWindowStatus, String> {
        // Resolve partial company symbol
        let resolved_company = self.resolve_company(&company_symbol);
        
        // Update cache
        self.update_cache("get_trading_window", "", &resolved_company, "", 
            &format!("Get trading window for {}", resolved_company));
        
        let endpoint = format!("trading_windows?company_symbol=eq.{}&select=*", resolved_company);
        
        let windows: Vec<TradingWindowStatus> = self.supabase_request(&endpoint, HttpMethod::Get, None).await?;
        
        windows.into_iter().next().ok_or_else(|| format!("Trading window info for {} not found", resolved_company))
    }

    /// Check if entity traded during closed window
    #[mutate]
    async fn check_window_violation(&mut self, entity_id: String, company_symbol: String, trade_timestamp: u64) -> Result<bool, String> {
        // Cross-parameter resolution
        let (resolved_entity, resolved_company, _) = self.resolve_from_cache(&entity_id, &company_symbol, "");
        
        // Update cache (though entity_id is not actually used in the query)
        self.update_cache("check_window_violation", &resolved_entity, &resolved_company, "", 
            &format!("Check if {} violated {} trading window", resolved_entity, resolved_company));
        
        let window_result = self.get_trading_window(resolved_company.clone()).await;
        
        match window_result {
            Ok(window) => {
                if window.window_status == "CLOSED" {
                    if trade_timestamp >= window.closure_start && trade_timestamp < window.expected_opening {
                        // Push alert for trading window violation
                        self.maybe_push_alert(
                            "WINDOW_VIOLATION",
                            "CRITICAL",
                            90,
                            &resolved_entity,
                            &resolved_company,
                            &format!("Trading window violation: {} traded {} during closed window", resolved_entity, resolved_company),
                        );
                        return Ok(true);
                    }
                }
                Ok(false)
            },
            Err(_) => Ok(false),
        }
    }

    /// Get all entities who accessed a specific UPSI
    #[mutate]
    async fn get_upsi_accessors(&mut self, upsi_id: String) -> Result<Vec<UPSIAccessLog>, String> {
        // Resolve partial UPSI ID
        let resolved_upsi = self.resolve_upsi_id(&upsi_id);
        
        // Update cache
        self.update_cache("get_upsi_accessors", "", "", &resolved_upsi, 
            &format!("Get all accessors of UPSI {}", resolved_upsi));
        
        let endpoint = format!("upsi_access_log?upsi_id=eq.{}&select=*", resolved_upsi);
        self.supabase_request(&endpoint, HttpMethod::Get, None).await
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_context",
      "description": "IMPORTANT: Call this FIRST before any other method. Returns recent query history with entity_ids, company_symbols, and upsi_ids to help resolve ambiguous user references like 'that company', 'same person', 'that UPSI', etc.\n",
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
      "name": "get_upsi",
      "description": "Get UPSI record by ID\n",
      "parameters": {
        "type": "object",
        "properties": {
          "upsi_id": {
            "type": "string",
            "description": "UPSI record ID (e.g., UPSI-001)\n"
          }
        },
        "required": [
          "upsi_id"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_active_upsi",
      "description": "Get all active (non-public) UPSI for a company\n",
      "parameters": {
        "type": "object",
        "properties": {
          "company_symbol": {
            "type": "string",
            "description": "Company stock symbol (e.g., RELIANCE, INFY, TCS)\n"
          }
        },
        "required": [
          "company_symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_upsi_access_log",
      "description": "Get access log for specific UPSI with optional time range\n",
      "parameters": {
        "type": "object",
        "properties": {
          "upsi_id": {
            "type": "string",
            "description": "UPSI record ID\n"
          },
          "from_timestamp": {
            "type": "integer",
            "description": "Start timestamp (optional)\n"
          },
          "to_timestamp": {
            "type": "integer",
            "description": "End timestamp (optional)\n"
          }
        },
        "required": [
          "upsi_id"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_access_by_person",
      "description": "Get all UPSI accesses by a specific person\n",
      "parameters": {
        "type": "object",
        "properties": {
          "accessor_entity_id": {
            "type": "string",
            "description": "Entity ID of the accessor (e.g., ENT-REL-001)\n"
          },
          "days_back": {
            "type": "integer",
            "description": "Number of days to look back (default: 30)\n"
          }
        },
        "required": [
          "accessor_entity_id"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "check_upsi_access_before",
      "description": "Check if entity had UPSI access before a date (for insider trading detection)\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity ID to check\n"
          },
          "company_symbol": {
            "type": "string",
            "description": "Company symbol\n"
          },
          "before_timestamp": {
            "type": "integer",
            "description": "Check access before this timestamp\n"
          }
        },
        "required": [
          "entity_id",
          "company_symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_trading_window",
      "description": "Get trading window status for a company\n",
      "parameters": {
        "type": "object",
        "properties": {
          "company_symbol": {
            "type": "string",
            "description": "Company symbol\n"
          }
        },
        "required": [
          "company_symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "check_window_violation",
      "description": "Check if entity traded during closed window\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity ID\n"
          },
          "company_symbol": {
            "type": "string",
            "description": "Company symbol\n"
          },
          "trade_timestamp": {
            "type": "integer",
            "description": "Timestamp of the trade\n"
          }
        },
        "required": [
          "entity_id",
          "company_symbol",
          "trade_timestamp"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_upsi_accessors",
      "description": "Get all entities who accessed a specific UPSI\n",
      "parameters": {
        "type": "object",
        "properties": {
          "upsi_id": {
            "type": "string",
            "description": "UPSI record ID\n"
          }
        },
        "required": [
          "upsi_id"
        ]
      }
    }
  }
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{
  "prompts": []
}"#.to_string()
    }
}
