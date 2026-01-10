//! # Surveillance Dashboard Applet
//!
//! Central state store for the Capital Market Surveillance Platform.
//! This applet receives data from all MCPs via cross-contract calls
//! and provides query interface for the frontend UI.
//!
//! ## Architecture
//! - Other MCPs call mutate functions via `call_contract`
//! - Frontend UI calls query functions directly
//! - State is stored in WeilVec (WeilMap is not iterable, so we use WeilVec with manual indexing)

use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, smart_contract, WeilType};
use weil_rs::collections::vec::WeilVec;
use weil_rs::collections::WeilId;

// ===== DATA STRUCTURES =====

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

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct WorkflowExecution {
    pub id: String,
    pub workflow_type: String,
    pub trigger: String,
    pub steps_completed: u32,
    pub total_steps: u32,
    pub status: String,
    pub started_at: u64,
    pub completed_at: u64,
    pub result_summary: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct CaseRecord {
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
pub struct SurveillanceStats {
    pub total_alerts_today: u32,
    pub total_workflows_today: u32,
    pub open_cases: u32,
    pub high_risk_entities: u32,
    pub compliance_score: u32,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct RiskEntity {
    pub entity_id: String,
    pub entity_name: String,
    pub risk_score: u32,
    pub alert_count: u32,
    pub last_alert_at: u64,
}

// ===== TRAIT DEFINITION =====

trait SurveillanceDashboard {
    fn new() -> Result<Self, String> where Self: Sized;
    async fn push_alert(&mut self, alert: Alert) -> Result<String, String>;
    async fn log_workflow_start(&mut self, workflow_id: String, workflow_type: String, trigger: String, total_steps: u32) -> Result<String, String>;
    async fn update_workflow_progress(&mut self, workflow_id: String, steps_completed: u32, status: String, result_summary: String) -> Result<String, String>;
    async fn upsert_case(&mut self, case_record: CaseRecord) -> Result<String, String>;
    async fn register_risk_entity(&mut self, entity: RiskEntity) -> Result<String, String>;
    async fn get_live_alerts(&self, severity_filter: String, limit: u32) -> Result<Vec<Alert>, String>;
    async fn get_workflow_history(&self, workflow_type: String, limit: u32) -> Result<Vec<WorkflowExecution>, String>;
    async fn get_cases_by_status(&self, status: String, limit: u32) -> Result<Vec<CaseRecord>, String>;
    async fn get_stats(&self) -> Result<SurveillanceStats, String>;
    async fn get_high_risk_entities(&self, min_risk_score: u32, limit: u32) -> Result<Vec<RiskEntity>, String>;
    async fn get_case_details(&self, case_id: String) -> Result<CaseRecord, String>;
    async fn get_entity_alerts(&self, entity_id: String, limit: u32) -> Result<Vec<Alert>, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

// ===== CONTRACT STATE =====
// Note: Using WeilVec for all collections since WeilMap is not iterable

#[derive(Serialize, Deserialize, WeilType)]
pub struct SurveillanceDashboardContractState {
    /// All alerts stored in order of arrival
    alerts: WeilVec<Alert>,
    /// All workflow executions
    workflows: WeilVec<WorkflowExecution>,
    /// Cases stored in WeilVec (upsert by scanning)
    cases: WeilVec<CaseRecord>,
    /// High-risk entities stored in WeilVec
    risk_entities: WeilVec<RiskEntity>,
    /// Running counters for stats
    alert_count_today: u32,
    workflow_count_today: u32,
}

// ===== CONTRACT IMPLEMENTATION =====

#[smart_contract]
impl SurveillanceDashboard for SurveillanceDashboardContractState {
    
    /// Initialize empty dashboard state
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(SurveillanceDashboardContractState {
            alerts: WeilVec::new(WeilId(1)),
            workflows: WeilVec::new(WeilId(2)),
            cases: WeilVec::new(WeilId(3)),
            risk_entities: WeilVec::new(WeilId(4)),
            alert_count_today: 0,
            workflow_count_today: 0,
        })
    }

    // ===== MUTATE FUNCTIONS (Called by other MCPs) =====

    /// Push a new alert
    #[mutate]
    async fn push_alert(&mut self, alert: Alert) -> Result<String, String> {
        let alert_id = alert.id.clone();
        self.alerts.push(alert);
        self.alert_count_today += 1;
        Ok(alert_id)
    }

    /// Log workflow start
    #[mutate]
    async fn log_workflow_start(
        &mut self, 
        workflow_id: String, 
        workflow_type: String, 
        trigger: String, 
        total_steps: u32
    ) -> Result<String, String> {
        let execution = WorkflowExecution {
            id: workflow_id.clone(),
            workflow_type,
            trigger,
            steps_completed: 0,
            total_steps,
            status: "RUNNING".to_string(),
            started_at: 0,
            completed_at: 0,
            result_summary: "".to_string(),
        };
        self.workflows.push(execution);
        self.workflow_count_today += 1;
        Ok(workflow_id)
    }

    /// Update workflow progress
    #[mutate]
    async fn update_workflow_progress(
        &mut self, 
        workflow_id: String, 
        steps_completed: u32, 
        status: String, 
        result_summary: String
    ) -> Result<String, String> {
        let len = self.workflows.len();
        for i in 0..len {
            if let Some(mut wf) = self.workflows.get(i) {
                if wf.id == workflow_id {
                    wf.steps_completed = steps_completed;
                    wf.status = status.clone();
                    wf.result_summary = result_summary.clone();
                    let _ = self.workflows.set(i, wf);
                    return Ok(workflow_id);
                }
            }
        }
        Err(format!("Workflow {} not found", workflow_id))
    }

    /// Upsert a case record
    #[mutate]
    async fn upsert_case(&mut self, case_record: CaseRecord) -> Result<String, String> {
        let case_id = case_record.case_id.clone();
        // Check if case exists and update
        let len = self.cases.len();
        for i in 0..len {
            if let Some(existing) = self.cases.get(i) {
                if existing.case_id == case_id {
                    let _ = self.cases.set(i, case_record);
                    return Ok(case_id);
                }
            }
        }
        // Not found, insert new
        self.cases.push(case_record);
        Ok(case_id)
    }

    /// Register or update a high-risk entity
    #[mutate]
    async fn register_risk_entity(&mut self, entity: RiskEntity) -> Result<String, String> {
        let entity_id = entity.entity_id.clone();
        // Check if entity exists and update
        let len = self.risk_entities.len();
        for i in 0..len {
            if let Some(existing) = self.risk_entities.get(i) {
                if existing.entity_id == entity_id {
                    let _ = self.risk_entities.set(i, entity);
                    return Ok(entity_id);
                }
            }
        }
        // Not found, insert new
        self.risk_entities.push(entity);
        Ok(entity_id)
    }

    // ===== QUERY FUNCTIONS (Called by Frontend UI) =====

    /// Get live alerts with optional severity filter
    #[query]
    async fn get_live_alerts(&self, severity_filter: String, limit: u32) -> Result<Vec<Alert>, String> {
        let mut result = Vec::new();
        let len = self.alerts.len();
        let mut count = 0u32;
        
        for i in (0..len).rev() {
            if count >= limit { break; }
            if let Some(alert) = self.alerts.get(i) {
                if severity_filter == "ALL" || alert.severity == severity_filter {
                    result.push(alert);
                    count += 1;
                }
            }
        }
        Ok(result)
    }

    /// Get workflow execution history
    #[query]
    async fn get_workflow_history(&self, workflow_type: String, limit: u32) -> Result<Vec<WorkflowExecution>, String> {
        let mut result = Vec::new();
        let len = self.workflows.len();
        let mut count = 0u32;
        
        for i in (0..len).rev() {
            if count >= limit { break; }
            if let Some(wf) = self.workflows.get(i) {
                if workflow_type == "ALL" || wf.workflow_type == workflow_type {
                    result.push(wf);
                    count += 1;
                }
            }
        }
        Ok(result)
    }

    /// Get cases by status
    #[query]
    async fn get_cases_by_status(&self, status: String, limit: u32) -> Result<Vec<CaseRecord>, String> {
        let mut result = Vec::new();
        let len = self.cases.len();
        let mut count = 0u32;
        
        for i in 0..len {
            if count >= limit { break; }
            if let Some(case) = self.cases.get(i) {
                if status == "ALL" || case.status == status {
                    result.push(case);
                    count += 1;
                }
            }
        }
        Ok(result)
    }

    /// Get surveillance statistics
    #[query]
    async fn get_stats(&self) -> Result<SurveillanceStats, String> {
        let mut open_cases = 0u32;
        let cases_len = self.cases.len();
        for i in 0..cases_len {
            if let Some(case) = self.cases.get(i) {
                if case.status == "OPEN" || case.status == "INVESTIGATING" {
                    open_cases += 1;
                }
            }
        }
        
        let mut high_risk = 0u32;
        let entities_len = self.risk_entities.len();
        for i in 0..entities_len {
            if let Some(entity) = self.risk_entities.get(i) {
                if entity.risk_score > 70 {
                    high_risk += 1;
                }
            }
        }
        
        let compliance = if self.alert_count_today > 100 { 0 } else { 100 - self.alert_count_today };
        
        Ok(SurveillanceStats {
            total_alerts_today: self.alert_count_today,
            total_workflows_today: self.workflow_count_today,
            open_cases,
            high_risk_entities: high_risk,
            compliance_score: compliance,
        })
    }

    /// Get high-risk entities above a threshold
    #[query]
    async fn get_high_risk_entities(&self, min_risk_score: u32, limit: u32) -> Result<Vec<RiskEntity>, String> {
        let mut result = Vec::new();
        let len = self.risk_entities.len();
        let mut count = 0u32;
        
        for i in 0..len {
            if count >= limit { break; }
            if let Some(entity) = self.risk_entities.get(i) {
                if entity.risk_score >= min_risk_score {
                    result.push(entity);
                    count += 1;
                }
            }
        }
        Ok(result)
    }

    /// Get specific case details by ID
    #[query]
    async fn get_case_details(&self, case_id: String) -> Result<CaseRecord, String> {
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

    /// Get alerts for a specific entity
    #[query]
    async fn get_entity_alerts(&self, entity_id: String, limit: u32) -> Result<Vec<Alert>, String> {
        let mut result = Vec::new();
        let len = self.alerts.len();
        let mut count = 0u32;
        
        for i in (0..len).rev() {
            if count >= limit { break; }
            if let Some(alert) = self.alerts.get(i) {
                if alert.entity_id == entity_id {
                    result.push(alert);
                }
            }
        }
        Ok(result)
    }

    /// Returns JSON schema of available tools
    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_live_alerts",
      "description": "Get latest surveillance alerts. Use filter='ALL' for everything.",
      "parameters": {
        "type": "object",
        "properties": {
          "severity_filter": { "type": "string", "enum": ["ALL", "CRITICAL", "HIGH", "MEDIUM", "LOW"], "description": "Severity level to filter by" },
          "limit": { "type": "integer", "description": "Maximum number of alerts to return" }
        },
        "required": ["severity_filter", "limit"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_workflow_history",
      "description": "Get history of automated workflows",
      "parameters": {
        "type": "object",
        "properties": {
          "workflow_type": { "type": "string", "description": "Type of workflow to filter by (or ALL)" },
          "limit": { "type": "integer", "description": "Maximum number of records" }
        },
        "required": ["workflow_type", "limit"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_cases_by_status",
      "description": "Get investigation cases by status",
      "parameters": {
        "type": "object",
        "properties": {
          "status": { "type": "string", "enum": ["ALL", "OPEN", "INVESTIGATING", "CLOSED"], "description": "Case status to filter by" },
          "limit": { "type": "integer", "description": "Maximum number of cases" }
        },
        "required": ["status", "limit"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_stats",
      "description": "Get daily surveillance statistics",
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
      "name": "get_high_risk_entities",
      "description": "Get entities with high risk scores",
      "parameters": {
        "type": "object",
        "properties": {
          "min_risk_score": { "type": "integer", "description": "Minimum risk score threshold" },
          "limit": { "type": "integer", "description": "Maximum number of entities" }
        },
        "required": ["min_risk_score", "limit"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_case_details",
      "description": "Get full details of a specific case",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": { "type": "string", "description": "Unique case ID" }
        },
        "required": ["case_id"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_entity_alerts",
      "description": "Get all alerts for a specific entity",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": { "type": "string", "description": "Entity ID to search for" },
          "limit": { "type": "integer", "description": "Maximum number of alerts" }
        },
        "required": ["entity_id", "limit"]
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
