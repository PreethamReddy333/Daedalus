
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, smart_contract, WeilType};
use weil_rs::collections::vec::WeilVec;
use weil_rs::collections::WeilId;
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;

// ===== CONFIG =====

#[derive(Debug, Serialize, Deserialize, WeilType, Default, Clone)]
pub struct DashboardConfig {
    pub name: String,
}

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
    async fn get_live_alerts(&self, severity_filter: Option<String>, limit: Option<u32>) -> Result<Vec<Alert>, String>;
    async fn get_workflow_history(&self, workflow_type: Option<String>, limit: Option<u32>) -> Result<Vec<WorkflowExecution>, String>;
    async fn get_cases_by_status(&self, status: Option<String>, limit: Option<u32>) -> Result<Vec<CaseRecord>, String>;
    async fn get_stats(&self) -> Result<SurveillanceStats, String>;
    async fn get_high_risk_entities(&self, min_risk_score: Option<u32>, limit: Option<u32>) -> Result<Vec<RiskEntity>, String>;
    async fn get_case_details(&self, case_id: String) -> Result<CaseRecord, String>;
    async fn get_entity_alerts(&self, entity_id: String, limit: Option<u32>) -> Result<Vec<Alert>, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
  
    fn start_file_upload(&mut self, path: String, total_chunks: u32) -> Result<(), String>;
    fn add_path_content(&mut self, path: String, chunk: Vec<u8>, index: u32) -> Result<(), String>;
    fn finish_upload(&mut self, path: String, size_bytes: u32) -> Result<(), String>;
    fn total_chunks(&self, path: String) -> Result<u32, String>;
    fn http_content(&self, path: String, index: u32, method: String) -> (u16, std::collections::HashMap<String, String>, Vec<u8>);
    fn size_bytes(&self, path: String) -> Result<u32, String>;
    fn get_chunk_size(&self) -> u32;
}

// ===== CONTRACT STATE =====

#[derive(Serialize, Deserialize, WeilType)]
pub struct SurveillanceDashboardContractState {
    secrets: Secrets<DashboardConfig>,
    alerts: WeilVec<Alert>,
    workflows: WeilVec<WorkflowExecution>,
    cases: WeilVec<CaseRecord>,
    risk_entities: WeilVec<RiskEntity>,
    alert_count_today: u32,
    workflow_count_today: u32,
    server: WebServer,
}

// ===== CONTRACT IMPLEMENTATION =====

#[smart_contract]
impl SurveillanceDashboard for SurveillanceDashboardContractState {
    
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(SurveillanceDashboardContractState {
            secrets: Secrets::new(),
            alerts: WeilVec::new(WeilId(1)),
            workflows: WeilVec::new(WeilId(2)),
            cases: WeilVec::new(WeilId(3)),
            risk_entities: WeilVec::new(WeilId(4)),
            alert_count_today: 0,
            workflow_count_today: 0,
            server: WebServer::new(WeilId(5), None),
        })
    }

    // ===== MUTATE FUNCTIONS (Called by other MCPs) =====

    #[mutate]
    async fn push_alert(&mut self, alert: Alert) -> Result<String, String> {
        let alert_id = alert.id.clone();
        self.alerts.push(alert);
        self.alert_count_today += 1;
        Ok(alert_id)
    }

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

    #[mutate]
    async fn upsert_case(&mut self, case_record: CaseRecord) -> Result<String, String> {
        let case_id = case_record.case_id.clone();
        let len = self.cases.len();
        for i in 0..len {
            if let Some(existing) = self.cases.get(i) {
                if existing.case_id == case_id {
                    let _ = self.cases.set(i, case_record);
                    return Ok(case_id);
                }
            }
        }
        self.cases.push(case_record);
        Ok(case_id)
    }

    #[mutate]
    async fn register_risk_entity(&mut self, entity: RiskEntity) -> Result<String, String> {
        let entity_id = entity.entity_id.clone();
        let len = self.risk_entities.len();
        for i in 0..len {
            if let Some(existing) = self.risk_entities.get(i) {
                if existing.entity_id == entity_id {
                    let _ = self.risk_entities.set(i, entity);
                    return Ok(entity_id);
                }
            }
        }
        self.risk_entities.push(entity);
        Ok(entity_id)
    }

    // ===== QUERY FUNCTIONS (Called by Frontend UI) =====

    #[query]
    async fn get_live_alerts(&self, severity_filter: Option<String>, limit: Option<u32>) -> Result<Vec<Alert>, String> {
        let filter = severity_filter.unwrap_or_else(|| "ALL".to_string());
        let lim = limit.unwrap_or(20);
        let mut result = Vec::new();
        let len = self.alerts.len();
        let mut count = 0u32;
        
        for i in (0..len).rev() {
            if count >= lim { break; }
            if let Some(alert) = self.alerts.get(i) {
                if filter == "ALL" || alert.severity == filter {
                    result.push(alert);
                    count += 1;
                }
            }
        }
        Ok(result)
    }

    #[query]
    async fn get_workflow_history(&self, workflow_type: Option<String>, limit: Option<u32>) -> Result<Vec<WorkflowExecution>, String> {
        let wf_type = workflow_type.unwrap_or_else(|| "ALL".to_string());
        let lim = limit.unwrap_or(20);
        let mut result = Vec::new();
        let len = self.workflows.len();
        let mut count = 0u32;
        
        for i in (0..len).rev() {
            if count >= lim { break; }
            if let Some(wf) = self.workflows.get(i) {
                if wf_type == "ALL" || wf.workflow_type == wf_type {
                    result.push(wf);
                    count += 1;
                }
            }
        }
        Ok(result)
    }

    #[query]
    async fn get_cases_by_status(&self, status: Option<String>, limit: Option<u32>) -> Result<Vec<CaseRecord>, String> {
        let st = status.unwrap_or_else(|| "ALL".to_string());
        let lim = limit.unwrap_or(20);
        let mut result = Vec::new();
        let len = self.cases.len();
        let mut count = 0u32;
        
        for i in 0..len {
            if count >= lim { break; }
            if let Some(case) = self.cases.get(i) {
                if st == "ALL" || case.status == st {
                    result.push(case);
                    count += 1;
                }
            }
        }
        Ok(result)
    }

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

    #[query]
    async fn get_high_risk_entities(&self, min_risk_score: Option<u32>, limit: Option<u32>) -> Result<Vec<RiskEntity>, String> {
        let min_score = min_risk_score.unwrap_or(70);
        let lim = limit.unwrap_or(20);
        let mut result = Vec::new();
        let len = self.risk_entities.len();
        let mut count = 0u32;
        
        for i in 0..len {
            if count >= lim { break; }
            if let Some(entity) = self.risk_entities.get(i) {
                if entity.risk_score >= min_score {
                    result.push(entity);
                    count += 1;
                }
            }
        }
        Ok(result)
    }

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

    #[query]
    async fn get_entity_alerts(&self, entity_id: String, limit: Option<u32>) -> Result<Vec<Alert>, String> {
        let lim = limit.unwrap_or(20);
        let mut result = Vec::new();
        let len = self.alerts.len();
        let mut count = 0u32;
        
        for i in (0..len).rev() {
            if count >= lim { break; }
            if let Some(alert) = self.alerts.get(i) {
                if alert.entity_id == entity_id {
                    result.push(alert);
                    count += 1;
                }
            }
        }
        Ok(result)
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "push_alert",
      "description": "Push a new surveillance alert to the dashboard",
      "parameters": {
        "type": "object",
        "properties": {
          "id": { "type": "string", "description": "Unique alert ID" },
          "alert_type": { "type": "string", "enum": ["INSIDER", "SPOOFING", "WASH_TRADE", "PUMP_DUMP", "FRONT_RUN"], "description": "Type of alert" },
          "severity": { "type": "string", "enum": ["CRITICAL", "HIGH", "MEDIUM", "LOW"], "description": "Severity level" },
          "risk_score": { "type": "integer", "description": "Risk score (0-100)" },
          "entity_id": { "type": "string", "description": "Entity ID involved" },
          "symbol": { "type": "string", "description": "Stock symbol" },
          "description": { "type": "string", "description": "Alert description" },
          "workflow_id": { "type": "string", "description": "Associated workflow ID" },
          "timestamp": { "type": "integer", "description": "Unix timestamp" }
        },
        "required": ["id", "alert_type", "severity", "risk_score", "entity_id", "symbol", "description", "workflow_id", "timestamp"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "log_workflow_start",
      "description": "Log the start of a workflow execution",
      "parameters": {
        "type": "object",
        "properties": {
          "workflow_id": { "type": "string", "description": "Unique workflow ID" },
          "workflow_type": { "type": "string", "enum": ["INSIDER_DETECTION", "MANIPULATION_CHECK", "KYC_ONBOARD"], "description": "Type of workflow" },
          "trigger": { "type": "string", "description": "What triggered this workflow" },
          "total_steps": { "type": "integer", "description": "Total number of steps in workflow" }
        },
        "required": ["workflow_id", "workflow_type", "trigger", "total_steps"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "update_workflow_progress",
      "description": "Update workflow execution progress",
      "parameters": {
        "type": "object",
        "properties": {
          "workflow_id": { "type": "string", "description": "Workflow ID to update" },
          "steps_completed": { "type": "integer", "description": "Number of steps completed" },
          "status": { "type": "string", "enum": ["RUNNING", "COMPLETED", "FAILED"], "description": "Current status" },
          "result_summary": { "type": "string", "description": "Summary of results" }
        },
        "required": ["workflow_id", "steps_completed", "status", "result_summary"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "upsert_case",
      "description": "Create or update a case record in the dashboard",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": { "type": "string", "description": "Unique case ID" },
          "case_type": { "type": "string", "enum": ["INSIDER_TRADING", "SPOOFING", "WASH_TRADING"], "description": "Type of case" },
          "status": { "type": "string", "enum": ["OPEN", "INVESTIGATING", "ESCALATED", "CLOSED"], "description": "Case status" },
          "priority": { "type": "string", "enum": ["CRITICAL", "HIGH", "MEDIUM", "LOW"], "description": "Priority level" },
          "subject_entity": { "type": "string", "description": "Subject entity ID" },
          "symbol": { "type": "string", "description": "Stock symbol" },
          "risk_score": { "type": "integer", "description": "Risk score (0-100)" },
          "assigned_to": { "type": "string", "description": "Assigned investigator" },
          "created_at": { "type": "integer", "description": "Creation timestamp" },
          "updated_at": { "type": "integer", "description": "Last update timestamp" },
          "summary": { "type": "string", "description": "Case summary" }
        },
        "required": ["case_id", "case_type", "status", "priority", "subject_entity", "symbol", "risk_score", "assigned_to", "created_at", "updated_at", "summary"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "register_risk_entity",
      "description": "Register or update a high-risk entity",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": { "type": "string", "description": "Entity ID" },
          "entity_name": { "type": "string", "description": "Entity name" },
          "risk_score": { "type": "integer", "description": "Risk score (0-100)" },
          "alert_count": { "type": "integer", "description": "Number of alerts" },
          "last_alert_at": { "type": "integer", "description": "Last alert timestamp" }
        },
        "required": ["entity_id", "entity_name", "risk_score", "alert_count", "last_alert_at"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_live_alerts",
      "description": "Get latest surveillance alerts. Defaults: severity_filter=ALL, limit=20",
      "parameters": {
        "type": "object",
        "properties": {
          "severity_filter": { "type": "string", "enum": ["ALL", "CRITICAL", "HIGH", "MEDIUM", "LOW"], "description": "Optional severity filter (default: ALL)" },
          "limit": { "type": "integer", "description": "Optional max alerts (default: 20)" }
        },
        "required": []
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_workflow_history",
      "description": "Get history of automated workflows. Defaults: workflow_type=ALL, limit=20",
      "parameters": {
        "type": "object",
        "properties": {
          "workflow_type": { "type": "string", "description": "Optional workflow type filter (default: ALL)" },
          "limit": { "type": "integer", "description": "Optional max records (default: 20)" }
        },
        "required": []
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_cases_by_status",
      "description": "Get investigation cases. Defaults: status=ALL, limit=20",
      "parameters": {
        "type": "object",
        "properties": {
          "status": { "type": "string", "enum": ["ALL", "OPEN", "INVESTIGATING", "CLOSED"], "description": "Optional status filter (default: ALL)" },
          "limit": { "type": "integer", "description": "Optional max cases (default: 20)" }
        },
        "required": []
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
      "description": "Get entities with high risk scores. Defaults: min_risk_score=70, limit=20",
      "parameters": {
        "type": "object",
        "properties": {
          "min_risk_score": { "type": "integer", "description": "Optional minimum score (default: 70)" },
          "limit": { "type": "integer", "description": "Optional max entities (default: 20)" }
        },
        "required": []
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
      "description": "Get all alerts for a specific entity. Default limit: 20",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": { "type": "string", "description": "Entity ID to search for" },
          "limit": { "type": "integer", "description": "Optional max alerts (default: 20)" }
        },
        "required": ["entity_id"]
      }
    }
  }
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{ "prompts": [] }"#.to_string()
    }

    // ===== WEBSERVER METHODS for static asset hosting =====

    #[mutate]
    fn start_file_upload(&mut self, path: String, total_chunks: u32) -> Result<(), String> {
        self.server.start_file_upload(WeilId(5), path, total_chunks)
    }

    #[query]
    fn total_chunks(&self, path: String) -> Result<u32, String> {
        self.server.total_chunks(path)
    }

    #[mutate]
    fn add_path_content(
        &mut self,
        path: String,
        chunk: Vec<u8>,
        index: u32,
    ) -> Result<(), String> {
        self.server.add_path_content(path, chunk, index)
    }

    #[mutate]
    fn finish_upload(&mut self, path: String, size_bytes: u32) -> Result<(), String> {
        self.server.finish_upload(path, size_bytes)
    }

    #[query]
    fn http_content(
        &self,
        path: String,
        index: u32,
        method: String,
    ) -> (u16, std::collections::HashMap<String, String>, Vec<u8>) {
        self.server.http_content(path, index, method)
    }

    #[query]
    fn size_bytes(&self, path: String) -> Result<u32, String> {
        self.server.size_bytes(path)
    }

    #[query]
    fn get_chunk_size(&self) -> u32 {
        self.server.get_chunk_size()
    }
}
