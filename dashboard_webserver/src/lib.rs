
mod trade_data;
mod entity_relationship;
mod upsi_database;
mod anomaly_detection;
mod regulatory_reports;
mod slack_notifier;

use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, smart_contract, WeilType};
use weil_rs::collections::{WeilId, WeilIdGenerator};
use weil_rs::collections::vec::WeilVec;
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;

pub use trade_data::{Trade, TradeAnalysis, TradeDataProxy};
pub use entity_relationship::{Entity, Relationship, InsiderStatus, EntityRelationshipProxy};
pub use upsi_database::{UPSIRecord, TradingWindowStatus, UPSIDatabaseProxy};
pub use regulatory_reports::{ReportResult, RegulatoryReportsProxy};

// ===== CONFIG =====

#[derive(Debug, Serialize, Deserialize, WeilType, Default, Clone)]
pub struct DashboardConfig {
    pub name: String,
    pub trade_data_contract_id: String,
    pub entity_relationship_contract_id: String,
    pub regulatory_reports_contract_id: String,
    pub upsi_database_contract_id: String,
}

// ===== DATA STRUCTURES (From Surveillance Dashboard) =====

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

// ===== TRAIT DEFINITION (Unified) =====

trait DashboardWebserver {
    fn new() -> Result<Self, String> where Self: Sized;
    fn ping(&self) -> String;

    // --- Business Logic Methods ---
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
    fn get_tools(&self) -> String;
    fn get_prompts(&self) -> String;

    // --- Proxy Methods (Cross-Contract) - all mutate because targets may be mutate ---
    async fn get_trades_proxy(&mut self, symbol: String, limit: Option<u32>) -> Result<Vec<Trade>, String>;
    async fn search_entities_proxy(&mut self, search_query: String) -> Result<Vec<Entity>, String>;
    async fn get_relationships_proxy(&mut self, entity_id: String) -> Result<Vec<Relationship>, String>;
    async fn check_insider_proxy(&mut self, entity_id: String, company_symbol: String) -> Result<InsiderStatus, String>;
    async fn get_active_upsi_proxy(&mut self, company_symbol: String) -> Result<Vec<UPSIRecord>, String>;
    async fn get_trading_window_proxy(&mut self, company_symbol: String) -> Result<TradingWindowStatus, String>;
    async fn analyze_volume_proxy(&mut self, symbol: String) -> Result<TradeAnalysis, String>;
    async fn generate_report_proxy(&mut self, report_type: String, params: String) -> Result<ReportResult, String>;

    // --- Webserver Methods ---
    fn start_file_upload(&mut self, path: String, total_chunks: u32) -> Result<(), String>;
    fn add_path_content(&mut self, path: String, chunk: Vec<u8>, index: u32) -> Result<(), String>;
    fn finish_upload(&mut self, path: String, size_bytes: u32) -> Result<(), String>;
    fn total_chunks(&self, path: String) -> Result<u32, String>;
    fn http_content(&self, path: String, index: u32, method: String) -> (u16, std::collections::HashMap<String, String>, Vec<u8>);
    fn size_bytes(&self, path: String) -> Result<u32, String>;
    fn get_chunk_size(&self) -> u32;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct DashboardWebserverContractState {
    secrets: Secrets<DashboardConfig>,
    
    alerts: WeilVec<Alert>,
    workflows: WeilVec<WorkflowExecution>,
    cases: WeilVec<CaseRecord>,
    risk_entities: WeilVec<RiskEntity>,
    alert_count_today: u32,
    workflow_count_today: u32,

    server: WebServer,
    weil_id_generator: WeilIdGenerator,
}

#[smart_contract]
impl DashboardWebserver for DashboardWebserverContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(DashboardWebserverContractState {
            secrets: Secrets::new(),
            // Logic State (Allocating IDs 1-4)
            alerts: WeilVec::new(WeilId(1)),
            workflows: WeilVec::new(WeilId(2)),
            cases: WeilVec::new(WeilId(3)),
            risk_entities: WeilVec::new(WeilId(4)),
            alert_count_today: 0,
            workflow_count_today: 0,
            
            // Webserver State
            server: WebServer::new(WeilId(5), None),
            // Generator starts at 100 for file uploads
            weil_id_generator: WeilIdGenerator::new(WeilId(6)),
        })
    }

    #[mutate]
    fn ping(&self) -> String {
        "pong".to_string()
    }

    // ===== LOGIC IMPLEMENTATION =====

    #[mutate]
    async fn push_alert(&mut self, alert: Alert) -> Result<String, String> {
        let alert_id = alert.id.clone();
        self.alerts.push(alert);
        self.alert_count_today += 1;
        Ok(alert_id)
    }

    #[mutate]
    async fn log_workflow_start(&mut self, workflow_id: String, workflow_type: String, trigger: String, total_steps: u32) -> Result<String, String> {
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
    async fn update_workflow_progress(&mut self, workflow_id: String, steps_completed: u32, status: String, result_summary: String) -> Result<String, String> {
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

    #[mutate]
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

    #[mutate]
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

    #[mutate]
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

    #[mutate]
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

    #[mutate]
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
    fn get_tools(&self) -> String {
        r#"[
          { "type": "function", "function": { "name": "push_alert", "parameters": { "type": "object", "properties": { "id": {"type": "string"} } } } },
          { "type": "function", "function": { "name": "upsert_case", "parameters": { "type": "object", "properties": { "case_id": {"type": "string"} } } } }
        ]"#.to_string()
    }

    #[query]
    fn get_prompts(&self) -> String {
        r#"{ "prompts": [] }"#.to_string()
    }

    // ===== PROXY IMPLEMENTATION (Using Generated Cross-Contract Bindings) =====

    #[mutate]
    async fn get_trades_proxy(&mut self, symbol: String, limit: Option<u32>) -> Result<Vec<Trade>, String> {
        let contract_id = self.secrets.config().trade_data_contract_id.clone();
        if contract_id.is_empty() { return Err("Trade Data Contract ID not configured".to_string()); }

        let proxy = TradeDataProxy::new(contract_id);
        proxy.get_trades_by_symbol(symbol, limit.unwrap_or(20))
            .map_err(|e| e.to_string())
    }

    #[mutate]
    async fn search_entities_proxy(&mut self, search_query: String) -> Result<Vec<Entity>, String> {
        let contract_id = self.secrets.config().entity_relationship_contract_id.clone();
        if contract_id.is_empty() { return Err("Entity Contract ID not configured".to_string()); }

        let proxy = EntityRelationshipProxy::new(contract_id);
        proxy.search_entities(search_query, 10)
            .map_err(|e| e.to_string())
    }

    #[mutate]
    async fn get_relationships_proxy(&mut self, entity_id: String) -> Result<Vec<Relationship>, String> {
        let contract_id = self.secrets.config().entity_relationship_contract_id.clone();
        if contract_id.is_empty() { return Err("Entity Contract ID not configured".to_string()); }

        let proxy = EntityRelationshipProxy::new(contract_id);
        proxy.get_relationships(entity_id)
            .map_err(|e| e.to_string())
    }

    #[mutate]
    async fn check_insider_proxy(&mut self, entity_id: String, company_symbol: String) -> Result<InsiderStatus, String> {
        let contract_id = self.secrets.config().entity_relationship_contract_id.clone();
        if contract_id.is_empty() { return Err("Entity Contract ID not configured".to_string()); }

        let proxy = EntityRelationshipProxy::new(contract_id);
        proxy.check_insider_status(entity_id, company_symbol)
            .map_err(|e| e.to_string())
    }

    #[mutate]
    async fn get_active_upsi_proxy(&mut self, company_symbol: String) -> Result<Vec<UPSIRecord>, String> {
        let contract_id = self.secrets.config().upsi_database_contract_id.clone();
        if contract_id.is_empty() { return Err("UPSI Contract ID not configured".to_string()); }

        let proxy = UPSIDatabaseProxy::new(contract_id);
        proxy.get_active_upsi(company_symbol)
            .map_err(|e| e.to_string())
    }

    #[mutate]
    async fn get_trading_window_proxy(&mut self, company_symbol: String) -> Result<TradingWindowStatus, String> {
        let contract_id = self.secrets.config().upsi_database_contract_id.clone();
        if contract_id.is_empty() { return Err("UPSI Contract ID not configured".to_string()); }

        let proxy = UPSIDatabaseProxy::new(contract_id);
        proxy.get_trading_window(company_symbol)
            .map_err(|e| e.to_string())
    }

    #[mutate]
    async fn analyze_volume_proxy(&mut self, symbol: String) -> Result<TradeAnalysis, String> {
        let contract_id = self.secrets.config().trade_data_contract_id.clone();
        if contract_id.is_empty() { return Err("Trade Data Contract ID not configured".to_string()); }

        let proxy = TradeDataProxy::new(contract_id);
        proxy.analyze_volume(symbol)
            .map_err(|e| e.to_string())
    }

    #[mutate]
    async fn generate_report_proxy(&mut self, report_type: String, params: String) -> Result<regulatory_reports::ReportResult, String> {
        let contract_id = self.secrets.config().regulatory_reports_contract_id.clone();
        if contract_id.is_empty() { return Err("Regulatory Reports Contract ID not configured".to_string()); }

        let proxy = regulatory_reports::RegulatoryReportsProxy::new(contract_id);
        
        if report_type == "surveillance" {
            let parsed: serde_json::Value = serde_json::from_str(&params)
                .map_err(|e| format!("Invalid params: {}", e))?;
            let from_date = parsed["from_date"].as_str().unwrap_or("").to_string();
            let to_date = parsed["to_date"].as_str().unwrap_or("").to_string();
            let rtype = parsed["report_type"].as_str().unwrap_or("daily").to_string();
            return proxy.generate_surveillance_report(from_date, to_date, rtype)
                .map_err(|e| e.to_string());
        } else if report_type == "str" {
            let parsed: serde_json::Value = serde_json::from_str(&params)
                .map_err(|e| format!("Invalid params: {}", e))?;
            let case_id = parsed["case_id"].as_str().unwrap_or("").to_string();
            let entity_id = parsed["entity_id"].as_str().unwrap_or("").to_string();
            let activity_type = parsed["activity_type"].as_str().unwrap_or("").to_string();
            let reason = parsed["reason"].as_str().unwrap_or("").to_string();
            return proxy.generate_str(case_id, entity_id, activity_type, reason)
                .map_err(|e| e.to_string());
        }
        
        Err("Unknown report type".to_string())
    }

    // ===== WEBSERVER IMPLEMENTATION =====

    #[mutate]
    fn start_file_upload(&mut self, path: String, total_chunks: u32) -> Result<(), String> {
        self.server.start_file_upload(self.weil_id_generator.next_id(), path, total_chunks)
    }

    #[query]
    fn total_chunks(&self, path: String) -> Result<u32, String> {
        self.server.total_chunks(path)
    }

    #[mutate]
    fn add_path_content(&mut self, path: String, chunk: Vec<u8>, index: u32) -> Result<(), String> {
        self.server.add_path_content(path, chunk, index)
    }

    #[mutate]
    fn finish_upload(&mut self, path: String, size_bytes: u32) -> Result<(), String> {
        self.server.finish_upload(path, size_bytes)
    }

    #[query]
    fn http_content(&self, path: String, index: u32, method: String) -> (u16, std::collections::HashMap<String, String>, Vec<u8>) {
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
