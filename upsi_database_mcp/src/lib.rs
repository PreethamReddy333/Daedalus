//! # UPSI Database MCP Server
//!
//! Tracks Unpublished Price Sensitive Information (UPSI) using Supabase (PostgreSQL).
//! Logs access to UPSI and monitors trading window status.
//!
//! ## External Service: Supabase (PostgreSQL + REST API)
//! - Stores UPSI records, access logs, and trading window status
//! - Accessible via REST API with Row Level Security

use serde::{Deserialize, Serialize};
use weil_macros::{constructor, query, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::http::{HttpClient, HttpMethod};

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

// ===== TRAIT DEFINITION =====

trait UPSIDatabase {
    fn new() -> Result<Self, String> where Self: Sized;
    async fn get_upsi(&self, upsi_id: String) -> Result<UPSIRecord, String>;
    async fn get_active_upsi(&self, company_symbol: String) -> Result<Vec<UPSIRecord>, String>;
    async fn get_upsi_access_log(&self, upsi_id: String, from_timestamp: u64, to_timestamp: u64) -> Result<Vec<UPSIAccessLog>, String>;
    async fn get_access_by_person(&self, accessor_entity_id: String, days_back: u32) -> Result<Vec<UPSIAccessLog>, String>;
    async fn check_upsi_access_before(&self, entity_id: String, company_symbol: String, before_timestamp: u64) -> Result<Vec<UPSIAccessLog>, String>;
    async fn get_trading_window(&self, company_symbol: String) -> Result<TradingWindowStatus, String>;
    async fn check_window_violation(&self, entity_id: String, company_symbol: String, trade_timestamp: u64) -> Result<bool, String>;
    async fn get_upsi_accessors(&self, upsi_id: String) -> Result<Vec<UPSIAccessLog>, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

// ===== CONTRACT STATE =====

#[derive(Serialize, Deserialize, WeilType)]
pub struct UPSIDatabaseContractState {
    secrets: Secrets<UPSIDatabaseConfig>,
}

impl UPSIDatabaseContractState {
    /// Helper to make HTTP requests to Supabase
    async fn supabase_request<T: for<'de> Deserialize<'de>>(&self, endpoint: &str, method: HttpMethod, body: Option<String>) -> Result<T, String> {
        let config = self.secrets.config();
        let url = format!("{}/rest/v1/{}", config.supabase_url, endpoint);
        
        // Supabase expects headers for API key
        let mut req = HttpClient::request(&url, method)
            .header("apikey", config.supabase_anon_key.clone())
            .header("Authorization", format!("Bearer {}", config.supabase_anon_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation"); // To get the response back
            
        if let Some(b) = body {
            req = req.body(b);
        }
        
        let response = req.send().map_err(|e| format!("Supabase request failed: {:?}", e))?;
        let response_text = response.text();
        
        serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse Supabase response: {} - Body: {}", e, response_text))
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
        Ok(UPSIDatabaseContractState {
            secrets: Secrets::new(),
        })
    }

    /// Get UPSI record by ID from Supabase
    #[query]
    async fn get_upsi(&self, upsi_id: String) -> Result<UPSIRecord, String> {
        // Query: upsi_records?upsi_id=eq.{id}&select=*
        // Note: URL encoding should be handled, simplified here
        let endpoint = format!("upsi_records?upsi_id=eq.{}&select=*", upsi_id);
        
        let records: Vec<UPSIRecord> = self.supabase_request(&endpoint, HttpMethod::Get, None).await?;
        
        records.into_iter().next().ok_or_else(|| format!("UPSI record {} not found", upsi_id))
    }

    /// Get all active (non-public) UPSI for a company
    #[query]
    async fn get_active_upsi(&self, company_symbol: String) -> Result<Vec<UPSIRecord>, String> {
        // Query: upsi_records?company_symbol=eq.{symbol}&is_public=eq.false&select=*
        let endpoint = format!("upsi_records?company_symbol=eq.{}&is_public=eq.false&select=*", company_symbol);
        
        self.supabase_request(&endpoint, HttpMethod::Get, None).await
    }

    /// Get UPSI access log for a specific UPSI
    #[query]
    async fn get_upsi_access_log(&self, upsi_id: String, from_timestamp: u64, to_timestamp: u64) -> Result<Vec<UPSIAccessLog>, String> {
        // Query: upsi_access_log?upsi_id=eq.{id}&access_timestamp=gte.{from}&access_timestamp=lte.{to}&select=*
        let endpoint = format!(
            "upsi_access_log?upsi_id=eq.{}&access_timestamp=gte.{}&access_timestamp=lte.{}&select=*",
            upsi_id, from_timestamp, to_timestamp
        );
        
        self.supabase_request(&endpoint, HttpMethod::Get, None).await
    }

    /// Get all UPSI accesses by a specific person
    #[query]
    async fn get_access_by_person(&self, accessor_entity_id: String, days_back: u32) -> Result<Vec<UPSIAccessLog>, String> {
        // Calculate timestamp for N days ago (approximate, ignoring exact time zone logic for now)
        let now = 1735689600; // Placeholder for current timestamp (would come from runtime in prod)
        let days_in_seconds = days_back as u64 * 86400;
        let start_time = if now > days_in_seconds { now - days_in_seconds } else { 0 };

        let endpoint = format!(
            "upsi_access_log?accessor_entity_id=eq.{}&access_timestamp=gte.{}&select=*",
            accessor_entity_id, start_time
        );
        
        self.supabase_request(&endpoint, HttpMethod::Get, None).await
    }

    /// Check if an entity had UPSI access before a date (Crucial for insider trading)
    #[query]
    async fn check_upsi_access_before(&self, entity_id: String, company_symbol: String, before_timestamp: u64) -> Result<Vec<UPSIAccessLog>, String> {
        // This is a join-like query. In Supabase/PostgREST you can use embedding resource.
        // Assuming relationship upsi_access_log.upsi_id -> upsi_records.upsi_id
        // Query: upsi_access_log?accessor_entity_id=eq.{entity}&access_timestamp=lt.{time}&select=*,upsi_records!inner(company_symbol)
        // Filter by company symbol on the joined resource
        // Since PostgREST syntax can be complex for URL construction manually, we might fetch user logs and filter in code for simplicity in this demo,
        // or execute a raw RPC call if defined in Supabase.
        // Let's try fetching logs for the user and filtering for the company via the associated UPSI record.
        // Optimization: Fetch logs for user, then fetch UPSI details.
        
        // 1. Get all logs for user before timestamp
        let endpoint_logs = format!(
            "upsi_access_log?accessor_entity_id=eq.{}&access_timestamp=lt.{}&select=*",
            entity_id, before_timestamp
        );
        let logs: Vec<UPSIAccessLog> = self.supabase_request(&endpoint_logs, HttpMethod::Get, None).await?;
        
        let mut relevant_logs = Vec::new();
        
        // 2. For each log, check if it relates to the company (Inefficient N+1, but simple for demo without complex join parsing)
        // In production, define a Postgres Function (RPC) for this query!
        for log in logs {
            let record = self.get_upsi(log.upsi_id.clone()).await;
            if let Ok(r) = record {
                if r.company_symbol == company_symbol {
                    relevant_logs.push(log);
                }
            }
        }
        
        Ok(relevant_logs)
    }

    /// Get trading window status for a company
    #[query]
    async fn get_trading_window(&self, company_symbol: String) -> Result<TradingWindowStatus, String> {
        let endpoint = format!("trading_windows?company_symbol=eq.{}&select=*", company_symbol);
        
        let windows: Vec<TradingWindowStatus> = self.supabase_request(&endpoint, HttpMethod::Get, None).await?;
        
        windows.into_iter().next().ok_or_else(|| format!("Trading window info for {} not found", company_symbol))
    }

    /// Check if entity traded during closed window
    #[query]
    async fn check_window_violation(&self, _entity_id: String, company_symbol: String, trade_timestamp: u64) -> Result<bool, String> {
        // First check window status at that time.
        // For simplicity, we check current window definition. A real system needs history of window statuses.
        // We will assume the trading_windows table reflects the "current" or "latest applicable" window policy.
        
        // Fetch current window config for company
        let window_result = self.get_trading_window(company_symbol).await;
        
        match window_result {
            Ok(window) => {
                if window.window_status == "CLOSED" {
                    // If window is closed, check if trade time falls within closure period
                    if trade_timestamp >= window.closure_start && trade_timestamp < window.expected_opening {
                         // Check if entity is a designated person (This check should ideally be here or caller)
                         // The caller asks "did they trace during closed window", so we return true if window was closed.
                         // Identity verification (is this person restricted?) logic happens in risk_scoring or here.
                         return Ok(true);
                    }
                }
                Ok(false)
            },
            Err(_) => Ok(false), // No window info found, assume open
        }
    }

    /// Get all entities who accessed a specific UPSI
    #[query]
    async fn get_upsi_accessors(&self, upsi_id: String) -> Result<Vec<UPSIAccessLog>, String> {
        let endpoint = format!("upsi_access_log?upsi_id=eq.{}&select=*", upsi_id);
        self.supabase_request(&endpoint, HttpMethod::Get, None).await
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {"type": "function", "function": {"name": "get_upsi", "description": "Get UPSI record by ID", "parameters": {"type": "object", "properties": {"upsi_id": {"type": "string"}}, "required": ["upsi_id"]}}},
  {"type": "function", "function": {"name": "get_active_upsi", "description": "Get all active (non-public) UPSI for a company", "parameters": {"type": "object", "properties": {"company_symbol": {"type": "string"}}, "required": ["company_symbol"]}}},
  {"type": "function", "function": {"name": "get_upsi_access_log", "description": "Get access log for specific UPSI", "parameters": {"type": "object", "properties": {"upsi_id": {"type": "string"}, "from_timestamp": {"type": "integer"}, "to_timestamp": {"type": "integer"}}, "required": ["upsi_id", "from_timestamp", "to_timestamp"]}}},
  {"type": "function", "function": {"name": "get_access_by_person", "description": "Get all UPSI accesses by a specific person", "parameters": {"type": "object", "properties": {"accessor_entity_id": {"type": "string"}, "days_back": {"type": "integer"}}, "required": ["accessor_entity_id", "days_back"]}}},
  {"type": "function", "function": {"name": "check_upsi_access_before", "description": "Check if entity had UPSI access before a date", "parameters": {"type": "object", "properties": {"entity_id": {"type": "string"}, "company_symbol": {"type": "string"}, "before_timestamp": {"type": "integer"}}, "required": ["entity_id", "company_symbol", "before_timestamp"]}}},
  {"type": "function", "function": {"name": "get_trading_window", "description": "Get trading window status for a company", "parameters": {"type": "object", "properties": {"company_symbol": {"type": "string"}}, "required": ["company_symbol"]}}},
  {"type": "function", "function": {"name": "check_window_violation", "description": "Check if entity traded during closed window", "parameters": {"type": "object", "properties": {"entity_id": {"type": "string"}, "company_symbol": {"type": "string"}, "trade_timestamp": {"type": "integer"}}, "required": ["entity_id", "company_symbol", "trade_timestamp"]}}},
  {"type": "function", "function": {"name": "get_upsi_accessors", "description": "Get all entities who accessed a specific UPSI", "parameters": {"type": "object", "properties": {"upsi_id": {"type": "string"}}, "required": ["upsi_id"]}}}
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{ "prompts": [] }"#.to_string()
    }
}
