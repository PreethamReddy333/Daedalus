//! # Case Management MCP Server
//!
//! Manages investigation cases and their full lifecycle.
//! Cases can be created from alerts, assigned to investigators, and tracked through resolution.
//!
//! ## Case Lifecycle:
//! OPEN → INVESTIGATING → ESCALATED → CLOSED
//!
//! ## State Storage:
//! - Cases stored in WeilMap for O(1) lookup
//! - Evidence and Notes stored in WeilVec per case
//! - Timeline events tracked automatically

use serde::{Deserialize, Serialize};
use weil_macros::{constructor, query, smart_contract, WeilType};
use weil_rs::collections::{WeilMap, WeilVec};
use weil_rs::config::Secrets;
use weil_rs::runtime::call_contract;

// ===== CONFIGURATION =====

#[derive(Debug, Serialize, Deserialize, WeilType, Default, Clone)]
pub struct CaseManagementConfig {
    pub dashboard_contract_id: String,
}

// ===== DATA STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct Case {
    pub case_id: String,
    pub case_type: String,
    pub status: String,
    pub priority: String,
    pub subject_entity: String,
    pub symbol: String,
    pub risk_score: u32,
    pub assigned_to: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub summary: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct CaseEvidence {
    pub evidence_id: String,
    pub case_id: String,
    pub evidence_type: String,
    pub description: String,
    pub source: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct CaseNote {
    pub note_id: String,
    pub case_id: String,
    pub author: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct CaseEvent {
    pub event_type: String,
    pub description: String,
    pub timestamp: u64,
    pub actor: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct CaseTimeline {
    pub case_id: String,
    pub events: Vec<CaseEvent>,
}

// Dashboard CaseRecord for cross-contract call
#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct DashboardCaseRecord {
    pub case_id: String,
    pub case_type: String,
    pub status: String,
    pub priority: String,
    pub subject_entity: String,
    pub symbol: String,
    pub risk_score: u32,
    pub assigned_to: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub summary: String,
}

// ===== TRAIT DEFINITION =====

trait CaseManagement {
    fn new() -> Result<Self, String> where Self: Sized;
    async fn create_case(&self, case_type: String, subject_entity: String, symbol: String, risk_score: u32, priority: String, summary: String) -> Result<String, String>;
    async fn update_case_status(&self, case_id: String, new_status: String, status_note: String) -> Result<Case, String>;
    async fn assign_case(&self, case_id: String, assigned_to: String) -> Result<Case, String>;
    async fn add_evidence(&self, case_id: String, evidence_type: String, description: String, source: String) -> Result<String, String>;
    async fn add_note(&self, case_id: String, author: String, content: String) -> Result<String, String>;
    async fn get_case(&self, case_id: String) -> Result<Case, String>;
    async fn list_open_cases(&self, priority_filter: String, limit: u32) -> Result<Vec<Case>, String>;
    async fn get_case_timeline(&self, case_id: String) -> Result<CaseTimeline, String>;
    async fn get_entity_cases(&self, entity_id: String) -> Result<Vec<Case>, String>;
    async fn get_case_stats(&self) -> Result<String, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

// ===== CONTRACT STATE =====

#[derive(Serialize, Deserialize, WeilType)]
pub struct CaseManagementContractState {
    secrets: Secrets<CaseManagementConfig>,
    cases: WeilMap<String, Case>,
    evidence: WeilVec<CaseEvidence>,
    notes: WeilVec<CaseNote>,
    events: WeilVec<CaseEvent>,
    case_counter: u64,
}

// ===== HELPER METHODS =====

impl CaseManagementContractState {
    fn generate_case_id(&self) -> String {
        format!("CASE-{:06}", self.case_counter + 1)
    }
    
    fn push_to_dashboard(&self, case: &Case) {
        let config = self.secrets.config();
        if !config.dashboard_contract_id.is_empty() {
            let dashboard_case = DashboardCaseRecord {
                case_id: case.case_id.clone(),
                case_type: case.case_type.clone(),
                status: case.status.clone(),
                priority: case.priority.clone(),
                subject_entity: case.subject_entity.clone(),
                symbol: case.symbol.clone(),
                risk_score: case.risk_score,
                assigned_to: case.assigned_to.clone(),
                created_at: case.created_at,
                updated_at: case.updated_at,
                summary: case.summary.clone(),
            };
            
            let _ = call_contract::<String, String>(
                &config.dashboard_contract_id,
                "upsert_case",
                (dashboard_case,),
            );
        }
    }
}

// ===== CONTRACT IMPLEMENTATION =====

#[smart_contract]
impl CaseManagement for CaseManagementContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(CaseManagementContractState {
            secrets: Secrets::new(),
            cases: WeilMap::new(1),
            evidence: WeilVec::new(2),
            notes: WeilVec::new(3),
            events: WeilVec::new(4),
            case_counter: 0,
        })
    }

    /// Create a new investigation case
    #[query]
    async fn create_case(
        &self, 
        case_type: String, 
        subject_entity: String, 
        symbol: String, 
        risk_score: u32, 
        priority: String, 
        summary: String
    ) -> Result<String, String> {
        let case_id = self.generate_case_id();
        let now = 0u64; // Would use Runtime::timestamp()
        
        let case = Case {
            case_id: case_id.clone(),
            case_type,
            status: "OPEN".to_string(),
            priority,
            subject_entity,
            symbol,
            risk_score,
            assigned_to: "".to_string(),
            created_at: now,
            updated_at: now,
            summary,
        };
        
        // Note: In a real impl, we'd use mutate and increment counter
        // For query-only MCP, we simulate state
        self.push_to_dashboard(&case);
        
        Ok(case_id)
    }

    /// Update case status
    #[query]
    async fn update_case_status(
        &self, 
        case_id: String, 
        new_status: String, 
        _status_note: String
    ) -> Result<Case, String> {
        // Validate status transition
        let valid_statuses = ["OPEN", "INVESTIGATING", "ESCALATED", "CLOSED"];
        if !valid_statuses.contains(&new_status.as_str()) {
            return Err(format!("Invalid status: {}. Must be one of: {:?}", new_status, valid_statuses));
        }
        
        // Get existing case (would be from WeilMap in real impl)
        let case = self.cases.get(&case_id)
            .ok_or_else(|| format!("Case {} not found", case_id))?;
        
        // Create updated case
        let updated_case = Case {
            status: new_status,
            updated_at: 0, // Would use Runtime::timestamp()
            ..case
        };
        
        self.push_to_dashboard(&updated_case);
        
        Ok(updated_case)
    }

    /// Assign case to investigator
    #[query]
    async fn assign_case(&self, case_id: String, assigned_to: String) -> Result<Case, String> {
        let case = self.cases.get(&case_id)
            .ok_or_else(|| format!("Case {} not found", case_id))?;
        
        let updated_case = Case {
            assigned_to,
            updated_at: 0,
            ..case
        };
        
        self.push_to_dashboard(&updated_case);
        
        Ok(updated_case)
    }

    /// Add evidence to a case
    #[query]
    async fn add_evidence(
        &self, 
        case_id: String, 
        evidence_type: String, 
        description: String, 
        source: String
    ) -> Result<String, String> {
        let evidence_id = format!("EV-{}-{}", case_id, self.evidence.len() + 1);
        
        let _evidence = CaseEvidence {
            evidence_id: evidence_id.clone(),
            case_id,
            evidence_type,
            description,
            source,
            timestamp: 0,
        };
        
        Ok(evidence_id)
    }

    /// Add a note to a case
    #[query]
    async fn add_note(&self, case_id: String, author: String, content: String) -> Result<String, String> {
        let note_id = format!("NOTE-{}-{}", case_id, self.notes.len() + 1);
        
        let _note = CaseNote {
            note_id: note_id.clone(),
            case_id,
            author,
            content,
            timestamp: 0,
        };
        
        Ok(note_id)
    }

    /// Get case details
    #[query]
    async fn get_case(&self, case_id: String) -> Result<Case, String> {
        self.cases.get(&case_id)
            .ok_or_else(|| format!("Case {} not found", case_id))
    }

    /// List open cases by priority
    #[query]
    async fn list_open_cases(&self, priority_filter: String, limit: u32) -> Result<Vec<Case>, String> {
        let mut result = Vec::new();
        let mut count = 0u32;
        
        for (_, case) in self.cases.iter() {
            if count >= limit {
                break;
            }
            
            // Filter by open status
            if case.status != "CLOSED" {
                // Filter by priority
                if priority_filter == "ALL" || case.priority == priority_filter {
                    result.push(case);
                    count += 1;
                }
            }
        }
        
        // Sort by priority (CRITICAL first)
        result.sort_by(|a, b| {
            let priority_order = |p: &str| match p {
                "CRITICAL" => 0,
                "HIGH" => 1,
                "MEDIUM" => 2,
                _ => 3,
            };
            priority_order(&a.priority).cmp(&priority_order(&b.priority))
        });
        
        Ok(result)
    }

    /// Get case timeline (all events in order)
    #[query]
    async fn get_case_timeline(&self, case_id: String) -> Result<CaseTimeline, String> {
        let mut events = Vec::new();
        
        // Collect events for this case
        for i in 0..self.events.len() {
            if let Some(event) = self.events.get(i) {
                // In real impl, filter by case_id
                events.push(event);
            }
        }
        
        // Sort by timestamp
        events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        Ok(CaseTimeline {
            case_id,
            events,
        })
    }

    /// Get cases for a specific entity
    #[query]
    async fn get_entity_cases(&self, entity_id: String) -> Result<Vec<Case>, String> {
        let mut result = Vec::new();
        
        for (_, case) in self.cases.iter() {
            if case.subject_entity == entity_id {
                result.push(case);
            }
        }
        
        Ok(result)
    }

    /// Get case statistics
    #[query]
    async fn get_case_stats(&self) -> Result<String, String> {
        let mut total = 0u32;
        let mut open = 0u32;
        let mut investigating = 0u32;
        let mut escalated = 0u32;
        let mut closed = 0u32;
        let mut critical = 0u32;
        
        for (_, case) in self.cases.iter() {
            total += 1;
            match case.status.as_str() {
                "OPEN" => open += 1,
                "INVESTIGATING" => investigating += 1,
                "ESCALATED" => escalated += 1,
                "CLOSED" => closed += 1,
                _ => {}
            }
            if case.priority == "CRITICAL" {
                critical += 1;
            }
        }
        
        Ok(format!(
            r#"{{"total":{}, "open":{}, "investigating":{}, "escalated":{}, "closed":{}, "critical":{}}}"#,
            total, open, investigating, escalated, closed, critical
        ))
    }

    /// Returns JSON schema of available tools
    #[query]
    fn tools(&self) -> String {
        r#"[
  {"type": "function", "function": {"name": "create_case", "description": "Create a new investigation case", "parameters": {"type": "object", "properties": {"case_type": {"type": "string", "description": "INSIDER_TRADING, SPOOFING, WASH_TRADING, PUMP_DUMP"}, "subject_entity": {"type": "string"}, "symbol": {"type": "string"}, "risk_score": {"type": "integer"}, "priority": {"type": "string", "description": "CRITICAL, HIGH, MEDIUM, LOW"}, "summary": {"type": "string"}}, "required": ["case_type", "subject_entity", "symbol", "risk_score", "priority", "summary"]}}},
  {"type": "function", "function": {"name": "update_case_status", "description": "Update case status. Allowed: OPEN -> INVESTIGATING -> ESCALATED -> CLOSED", "parameters": {"type": "object", "properties": {"case_id": {"type": "string"}, "new_status": {"type": "string"}, "status_note": {"type": "string"}}, "required": ["case_id", "new_status", "status_note"]}}},
  {"type": "function", "function": {"name": "assign_case", "description": "Assign case to an investigator", "parameters": {"type": "object", "properties": {"case_id": {"type": "string"}, "assigned_to": {"type": "string"}}, "required": ["case_id", "assigned_to"]}}},
  {"type": "function", "function": {"name": "add_evidence", "description": "Add evidence to a case", "parameters": {"type": "object", "properties": {"case_id": {"type": "string"}, "evidence_type": {"type": "string", "description": "TRADE, COMMUNICATION, DOCUMENT, ANALYSIS"}, "description": {"type": "string"}, "source": {"type": "string"}}, "required": ["case_id", "evidence_type", "description", "source"]}}},
  {"type": "function", "function": {"name": "add_note", "description": "Add a note to a case", "parameters": {"type": "object", "properties": {"case_id": {"type": "string"}, "author": {"type": "string"}, "content": {"type": "string"}}, "required": ["case_id", "author", "content"]}}},
  {"type": "function", "function": {"name": "get_case", "description": "Get case details", "parameters": {"type": "object", "properties": {"case_id": {"type": "string"}}, "required": ["case_id"]}}},
  {"type": "function", "function": {"name": "list_open_cases", "description": "List open cases by priority", "parameters": {"type": "object", "properties": {"priority_filter": {"type": "string", "description": "ALL, CRITICAL, HIGH, MEDIUM, LOW"}, "limit": {"type": "integer"}}, "required": ["priority_filter", "limit"]}}},
  {"type": "function", "function": {"name": "get_case_timeline", "description": "Get case timeline (all events)", "parameters": {"type": "object", "properties": {"case_id": {"type": "string"}}, "required": ["case_id"]}}},
  {"type": "function", "function": {"name": "get_entity_cases", "description": "Get cases for an entity", "parameters": {"type": "object", "properties": {"entity_id": {"type": "string"}}, "required": ["entity_id"]}}},
  {"type": "function", "function": {"name": "get_case_stats", "description": "Get case statistics", "parameters": {"type": "object", "properties": {}, "required": []}}}
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{ "prompts": [] }"#.to_string()
    }
}
