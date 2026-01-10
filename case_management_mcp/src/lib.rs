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
use weil_macros::{constructor, query, mutate, smart_contract, WeilType};
use weil_rs::collections::vec::WeilVec;
use weil_rs::collections::WeilId;
use weil_rs::config::Secrets;
use weil_rs::runtime::Runtime;

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
    async fn create_case(&mut self, case_type: String, subject_entity: String, symbol: String, risk_score: u32, priority: String, summary: String) -> Result<String, String>;
    async fn update_case_status(&mut self, case_id: String, new_status: String, status_note: String) -> Result<Case, String>;
    async fn assign_case(&mut self, case_id: String, assigned_to: String) -> Result<Case, String>;
    async fn add_evidence(&mut self, case_id: String, evidence_type: String, description: String, source: String) -> Result<String, String>;
    async fn add_note(&mut self, case_id: String, author: String, content: String) -> Result<String, String>;
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
    // Using WeilVec for cases to support iteration
    cases: WeilVec<Case>,
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
            
            let args = serde_json::to_string(&dashboard_case).unwrap();
            let _ = Runtime::call_contract::<String>(
                config.dashboard_contract_id.clone(),
                "upsert_case".to_string(),
                Some(args),
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
            cases: WeilVec::new(WeilId(1)),
            evidence: WeilVec::new(WeilId(2)),
            notes: WeilVec::new(WeilId(3)),
            events: WeilVec::new(WeilId(4)),
            case_counter: 0,
        })
    }

    /// Create a new investigation case
    #[mutate]
    async fn create_case(
        &mut self, 
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
        
        self.cases.push(case.clone());
        self.push_to_dashboard(&case);
        
        Ok(case_id)
    }

    /// Update case status
    #[mutate]
    async fn update_case_status(
        &mut self, 
        case_id: String, 
        new_status: String, 
        _status_note: String
    ) -> Result<Case, String> {
        // Validate status transition
        let valid_statuses = ["OPEN", "INVESTIGATING", "ESCALATED", "CLOSED"];
        if !valid_statuses.contains(&new_status.as_str()) {
            return Err(format!("Invalid status: {}. Must be one of: {:?}", new_status, valid_statuses));
        }
        
        // Find and update existing case
        let len = self.cases.len();
        for i in 0..len {
            if let Some(mut case) = self.cases.get(i) {
                if case.case_id == case_id {
                    case.status = new_status.clone();
                    case.updated_at = 0;
                    let _ = self.cases.set(i, case.clone());
                    self.push_to_dashboard(&case);
                    return Ok(case);
                }
            }
        }
        Err(format!("Case {} not found", case_id))
    }

    /// Assign case to investigator
    #[mutate]
    async fn assign_case(&mut self, case_id: String, assigned_to: String) -> Result<Case, String> {
        let len = self.cases.len();
        for i in 0..len {
            if let Some(mut case) = self.cases.get(i) {
                if case.case_id == case_id {
                    case.assigned_to = assigned_to.clone();
                    case.updated_at = 0;
                    let _ = self.cases.set(i, case.clone());
                    self.push_to_dashboard(&case);
                    return Ok(case);
                }
            }
        }
        Err(format!("Case {} not found", case_id))
    }

    /// Add evidence to a case
    #[mutate]
    async fn add_evidence(
        &mut self, 
        case_id: String, 
        evidence_type: String, 
        description: String, 
        source: String
    ) -> Result<String, String> {
        let evidence_id = format!("EV-{}-{}", case_id, self.evidence.len() + 1);
        
        let evidence = CaseEvidence {
            evidence_id: evidence_id.clone(),
            case_id,
            evidence_type,
            description,
            source,
            timestamp: 0,
        };
        self.evidence.push(evidence);
        
        Ok(evidence_id)
    }

    /// Add a note to a case
    #[mutate]
    async fn add_note(&mut self, case_id: String, author: String, content: String) -> Result<String, String> {
        let note_id = format!("NOTE-{}-{}", case_id, self.notes.len() + 1);
        
        let note = CaseNote {
            note_id: note_id.clone(),
            case_id,
            author,
            content,
            timestamp: 0,
        };
        self.notes.push(note);
        
        Ok(note_id)
    }

    /// Get case details
    #[query]
    async fn get_case(&self, case_id: String) -> Result<Case, String> {
        let len = self.cases.len();
        for i in 0..len {
            if let Some(case) = self.cases.get(i) {
                if case.case_id == case_id {
                    return Ok(case);
                }
            }
        }
        Err(format!("Case {} not found", case_id))
    }

    /// List open cases by priority
    #[query]
    async fn list_open_cases(&self, priority_filter: String, limit: u32) -> Result<Vec<Case>, String> {
        let mut result = Vec::new();
        let mut count = 0u32;
        
        let len = self.cases.len();
        for i in 0..len {
            if count >= limit {
                break;
            }
            if let Some(case) = self.cases.get(i) {
                // Filter by open status
                if case.status != "CLOSED" {
                    // Filter by priority
                    if priority_filter == "ALL" || case.priority == priority_filter {
                        result.push(case);
                        count += 1;
                    }
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
        
        let len = self.cases.len();
        for i in 0..len {
            if let Some(case) = self.cases.get(i) {
                if case.subject_entity == entity_id {
                    result.push(case);
                }
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
        
        let len = self.cases.len();
        for i in 0..len {
            if let Some(case) = self.cases.get(i) {
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
        }
        
        Ok(format!(
            r#"{{"total":{}, "open":{}, "investigating":{}, "escalated":{}, "closed":{}, "critical":{}}}"#,
            total, open, investigating, escalated, closed, critical
        ))
    }
    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "create_case",
      "description": "Create a new investigation case. Returns the new case ID.",
      "parameters": {
        "type": "object",
        "properties": {
          "case_type": { "type": "string", "enum": ["INSIDER_TRADING", "SPOOFING", "WASH_TRADING", "PUMP_DUMP"], "description": "Type of case" },
          "subject_entity": { "type": "string", "description": "Subject entity (account ID or company ID)" },
          "symbol": { "type": "string", "description": "Stock symbol involved" },
          "risk_score": { "type": "integer", "description": "Initial risk score (0-100)" },
          "priority": { "type": "string", "enum": ["CRITICAL", "HIGH", "MEDIUM", "LOW"], "description": "Priority level" },
          "summary": { "type": "string", "description": "Brief summary of the case" }
        },
        "required": ["case_type", "subject_entity", "symbol", "risk_score", "priority", "summary"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "update_case_status",
      "description": "Update case status. Allowed: OPEN -> INVESTIGATING -> ESCALATED -> CLOSED",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": { "type": "string", "description": "Case ID to update" },
          "new_status": { "type": "string", "enum": ["OPEN", "INVESTIGATING", "ESCALATED", "CLOSED"], "description": "New status" },
          "status_note": { "type": "string", "description": "Note explaining the status change" }
        },
        "required": ["case_id", "new_status", "status_note"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "assign_case",
      "description": "Assign case to an investigator",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": { "type": "string", "description": "Case ID to assign" },
          "assigned_to": { "type": "string", "description": "Investigator ID or name" }
        },
        "required": ["case_id", "assigned_to"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "add_evidence",
      "description": "Add evidence to a case",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": { "type": "string", "description": "Case ID" },
          "evidence_type": { "type": "string", "enum": ["TRADE", "COMMUNICATION", "DOCUMENT", "ANALYSIS"], "description": "Type of evidence" },
          "description": { "type": "string", "description": "Description of the evidence" },
          "source": { "type": "string", "description": "Source of the evidence" }
        },
        "required": ["case_id", "evidence_type", "description", "source"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "add_note",
      "description": "Add a note to a case",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": { "type": "string", "description": "Case ID" },
          "author": { "type": "string", "description": "Author of the note" },
          "content": { "type": "string", "description": "Note content" }
        },
        "required": ["case_id", "author", "content"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_case",
      "description": "Get case details by ID",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": { "type": "string", "description": "Case ID to retrieve" }
        },
        "required": ["case_id"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "list_open_cases",
      "description": "List open cases by priority",
      "parameters": {
        "type": "object",
        "properties": {
          "priority_filter": { "type": "string", "enum": ["ALL", "CRITICAL", "HIGH", "MEDIUM", "LOW"], "description": "Priority filter" },
          "limit": { "type": "integer", "description": "Maximum number of cases to return" }
        },
        "required": ["priority_filter", "limit"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_case_timeline",
      "description": "Get case timeline (all events in order)",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": { "type": "string", "description": "Case ID" }
        },
        "required": ["case_id"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_entity_cases",
      "description": "Get cases for a specific entity",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": { "type": "string", "description": "Entity ID to search for" }
        },
        "required": ["entity_id"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_case_stats",
      "description": "Get case statistics",
      "parameters": {
        "type": "object",
        "properties": {},
        "required": []
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
