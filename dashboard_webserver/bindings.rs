
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


#[derive(Debug, Serialize, Deserialize, WeilType, Default)]
pub struct DashboardConfig {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SurveillanceStats {
    pub total_alerts_today: u32,
    pub total_workflows_today: u32,
    pub open_cases: u32,
    pub high_risk_entities: u32,
    pub compliance_score: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiskEntity {
    pub entity_id: String,
    pub entity_name: String,
    pub risk_score: u32,
    pub alert_count: u32,
    pub last_alert_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub entity_id: String,
    pub entity_type: String,
    pub name: String,
    pub pan_number: String,
    pub registration_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub source_entity_id: String,
    pub target_entity_id: String,
    pub relationship_type: String,
    pub relationship_detail: String,
    pub strength: u32,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsiderStatus {
    pub entity_id: String,
    pub company_symbol: String,
    pub is_insider: bool,
    pub insider_type: String,
    pub designation: String,
    pub window_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportResult {
    pub report_id: String,
    pub report_type: String,
    pub storage_path: String,
    pub download_url: String,
    pub expires_at: u64,
    pub risk_score: u32,
    pub success: bool,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TradingWindowStatus {
    pub company_symbol: String,
    pub window_status: String,
    pub closure_reason: String,
    pub closure_start: u64,
    pub expected_opening: u64,
}

#[derive(Debug, Serialize, Deserialize)]
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

trait DashboardWebserver {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn ping(&self) -> String;
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
    async fn get_trades_proxy(&mut self, symbol: String, limit: Option<u32>) -> Result<Vec<Trade>, String>;
    async fn search_entities_proxy(&mut self, search_query: String) -> Result<Vec<Entity>, String>;
    async fn get_relationships_proxy(&mut self, entity_id: String) -> Result<Vec<Relationship>, String>;
    async fn check_insider_proxy(&mut self, entity_id: String, company_symbol: String) -> Result<InsiderStatus, String>;
    async fn get_active_upsi_proxy(&mut self, company_symbol: String) -> Result<Vec<UPSIRecord>, String>;
    async fn get_trading_window_proxy(&mut self, company_symbol: String) -> Result<TradingWindowStatus, String>;
    async fn analyze_volume_proxy(&mut self, symbol: String) -> Result<TradeAnalysis, String>;
    async fn generate_report_proxy(&mut self, report_type: String, params: String) -> Result<ReportResult, String>;
    async fn get_tools(&self) -> String;
    async fn get_prompts(&self) -> String;

    // webserver specific functions
    fn start_file_upload(&mut self, path: String, total_chunks: u32) -> Result<(), String>;
    fn add_path_content(
        &mut self,
        path: String,
        chunk: Vec<u8>,
        index: u32,
    ) -> Result<(), String>;
    fn finish_upload(&mut self, path: String, size_bytes: u32) -> Result<(), String>;
    fn total_chunks(&self, path: String) -> Result<u32, String>;
    fn http_content(
        &self,
        path: String,
        index: u32,
        method: String,
    ) -> (u16, std::collections::HashMap<String, String>, Vec<u8>);
    fn size_bytes(&self, path: String) -> Result<u32, String>;
    fn get_chunk_size(&self) -> u32;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct DashboardWebserverContractState {
    // define your contract state here!
    secrets: Secrets<DashboardConfig>,
    server: WebServer,
}

#[smart_contract]
impl DashboardWebserver for DashboardWebserverContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!();
    }


    #[query]
    async fn ping(&self) -> String {
        unimplemented!();
    }

    #[mutate]
    async fn push_alert(&mut self, alert: Alert) -> Result<String, String> {
        unimplemented!();
    }

    #[mutate]
    async fn log_workflow_start(&mut self, workflow_id: String, workflow_type: String, trigger: String, total_steps: u32) -> Result<String, String> {
        unimplemented!();
    }

    #[mutate]
    async fn update_workflow_progress(&mut self, workflow_id: String, steps_completed: u32, status: String, result_summary: String) -> Result<String, String> {
        unimplemented!();
    }

    #[mutate]
    async fn upsert_case(&mut self, case_record: CaseRecord) -> Result<String, String> {
        unimplemented!();
    }

    #[mutate]
    async fn register_risk_entity(&mut self, entity: RiskEntity) -> Result<String, String> {
        unimplemented!();
    }

    #[query]
    async fn get_live_alerts(&self, severity_filter: Option<String>, limit: Option<u32>) -> Result<Vec<Alert>, String> {
        unimplemented!();
    }

    #[query]
    async fn get_workflow_history(&self, workflow_type: Option<String>, limit: Option<u32>) -> Result<Vec<WorkflowExecution>, String> {
        unimplemented!();
    }

    #[query]
    async fn get_cases_by_status(&self, status: Option<String>, limit: Option<u32>) -> Result<Vec<CaseRecord>, String> {
        unimplemented!();
    }

    #[query]
    async fn get_stats(&self) -> Result<SurveillanceStats, String> {
        unimplemented!();
    }

    #[query]
    async fn get_high_risk_entities(&self, min_risk_score: Option<u32>, limit: Option<u32>) -> Result<Vec<RiskEntity>, String> {
        unimplemented!();
    }

    #[query]
    async fn get_case_details(&self, case_id: String) -> Result<CaseRecord, String> {
        unimplemented!();
    }

    #[query]
    async fn get_entity_alerts(&self, entity_id: String, limit: Option<u32>) -> Result<Vec<Alert>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_trades_proxy(&mut self, symbol: String, limit: Option<u32>) -> Result<Vec<Trade>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn search_entities_proxy(&mut self, search_query: String) -> Result<Vec<Entity>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_relationships_proxy(&mut self, entity_id: String) -> Result<Vec<Relationship>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn check_insider_proxy(&mut self, entity_id: String, company_symbol: String) -> Result<InsiderStatus, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_active_upsi_proxy(&mut self, company_symbol: String) -> Result<Vec<UPSIRecord>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_trading_window_proxy(&mut self, company_symbol: String) -> Result<TradingWindowStatus, String> {
        unimplemented!();
    }

    #[mutate]
    async fn analyze_volume_proxy(&mut self, symbol: String) -> Result<TradeAnalysis, String> {
        unimplemented!();
    }

    #[mutate]
    async fn generate_report_proxy(&mut self, report_type: String, params: String) -> Result<ReportResult, String> {
        unimplemented!();
    }

    #[query]
    async fn get_tools(&self) -> String {
        unimplemented!();
    }

    #[query]
    async fn get_prompts(&self) -> String {
        unimplemented!();
    }

    #[mutate]
    fn start_file_upload(&mut self, path: String, total_chunks: u32) -> Result<(), String> {
        self.server.start_file_upload(path, total_chunks)
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

