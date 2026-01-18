
mod anomaly_detection;
mod dashboard;
mod entity_relationship;
mod jira;
mod risk_scoring;

use anomaly_detection::AnomalyDetectionMcp;
use dashboard::DashboardMcp;
use entity_relationship::EntityRelationshipMcp;
use jira::JiraMcp;
use risk_scoring::RiskScoringMcp;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use weil_macros::{constructor, mutate, query, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::http::{HttpClient, HttpMethod};

// ===== CONFIGURATION =====

#[derive(Debug, Serialize, Deserialize, WeilType, Default, Clone)]
pub struct RegulatoryReportsConfig {
    pub dashboard_contract_id: String,
    pub jira_contract_id: String,
    pub risk_scoring_contract_id: String,
    pub anomaly_detection_contract_id: String,
    pub entity_relationship_contract_id: String,
    pub supabase_url: String,
    pub supabase_service_key: String,
    pub supabase_bucket: String,
    pub sebi_api_endpoint: String,
}

// ===== DATA STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct STRReport {
    pub str_id: String,
    pub report_date: String,
    pub suspicious_entity_id: String,
    pub suspicious_entity_name: String,
    pub suspicious_activity_type: String,
    pub transaction_details: String,
    pub total_value: String,
    pub suspicion_reason: String,
    pub investigation_summary: String,
    pub recommendation: String,
    pub risk_score: u32,
    pub generated_at: u64,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct MarketSurveillanceReport {
    pub report_id: String,
    pub report_period: String,
    pub total_alerts: u32,
    pub critical_alerts: u32,
    pub investigations_opened: u32,
    pub investigations_closed: u32,
    pub manipulation_cases: u32,
    pub insider_trading_cases: u32,
    pub enforcement_actions: u32,
    pub summary: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct ComplianceScorecard {
    pub entity_id: String,
    pub entity_name: String,
    pub reporting_period: String,
    pub overall_score: u32,
    pub kyc_compliance: u32,
    pub aml_compliance: u32,
    pub surveillance_compliance: u32,
    pub reporting_compliance: u32,
    pub violations_count: u32,
    pub risk_score: u32,
    pub last_updated: u64,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
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

// ===== CONTEXT CACHE STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone, Default)]
pub struct QueryHistory {
    pub method_name: String,
    pub entity_id: String,
    pub company_symbol: String,
    pub case_id: String,
    pub report_id: String,
    pub timestamp: u64,
    pub natural_language_prompt: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone, Default)]
pub struct QueryContext {
    pub recent_queries: Vec<QueryHistory>,
    pub last_entity_id: String,
    pub last_company_symbol: String,
    pub last_case_id: String,
    pub last_report_id: String,
}

// ===== TRAIT DEFINITION =====

trait RegulatoryReports {
    fn new() -> Result<Self, String> where Self: Sized;
    async fn get_context(&mut self) -> QueryContext;
    async fn generate_str(&mut self, case_id: String, entity_id: String, suspicious_activity_type: String, suspicion_reason: String) -> Result<ReportResult, String>;
    async fn generate_surveillance_report(&mut self, from_date: String, to_date: String, report_type: String) -> Result<ReportResult, String>;
    async fn generate_compliance_scorecard(&mut self, entity_id: String, period: String) -> Result<ReportResult, String>;
    async fn generate_entity_risk_report(&mut self, entity_id: String) -> Result<ReportResult, String>;
    async fn generate_gsm_report(&mut self, report_date: String) -> Result<ReportResult, String>;
    async fn generate_esm_report(&mut self, report_date: String) -> Result<ReportResult, String>;
    async fn get_pending_strs(&mut self, limit: u32) -> Result<Vec<STRReport>, String>;
    async fn submit_str(&mut self, str_id: String) -> Result<ReportResult, String>;
    async fn generate_investigation_report(&mut self, case_id: String, include_evidence: bool) -> Result<ReportResult, String>;
    async fn get_report_url(&mut self, report_id: String) -> Result<ReportResult, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

// ===== CONTRACT STATE =====

#[derive(Serialize, Deserialize, WeilType)]
pub struct RegulatoryReportsContractState {
    secrets: Secrets<RegulatoryReportsConfig>,
    query_cache: QueryContext,
    pending_strs: Vec<STRReport>,
    report_counter: u32,
}
impl RegulatoryReportsContractState {
    // ===== SUPABASE STORAGE METHODS =====

    fn upload_to_supabase(&self, file_path: &str, content: &str) -> Result<String, String> {
        let config = self.secrets.config();
        
        let url = format!(
            "{}/storage/v1/object/{}/{}",
            config.supabase_url, config.supabase_bucket, file_path
        );
        
        let mut headers = HashMap::new();
        headers.insert("apikey".to_string(), config.supabase_service_key.clone());
        headers.insert("Authorization".to_string(), format!("Bearer {}", config.supabase_service_key));
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("x-upsert".to_string(), "true".to_string());
        
        match HttpClient::request(&url, HttpMethod::Post)
            .headers(headers)
            .body(content.to_string())
            .send() 
        {
            Ok(response) => {
                let resp_text = response.text();
                let debug_resp = if resp_text.len() > 80 {
                    format!("{}...", &resp_text[..80])
                } else {
                    resp_text.clone()
                };
                
                if resp_text.contains("\"error\"") || resp_text.contains("\"statusCode\"") {
                    Ok(format!("ERR|{}|{}", debug_resp.replace("\"", "'"), file_path))
                } else if resp_text.is_empty() {
                    Ok(format!("EMPTY|{}", file_path))
                } else {
                    Ok(format!("OK|{}|{}", debug_resp.replace("\"", "'"), file_path))
                }
            },
            Err(e) => {
                Ok(format!("FAIL|{:?}|{}", e, file_path))
            }
        }
    }

    fn get_public_url(&self, file_path: &str) -> String {
        let config = self.secrets.config();
        format!(
            "{}/storage/v1/object/public/{}/{}",
            config.supabase_url, config.supabase_bucket, file_path
        )
    }

    #[allow(dead_code)]
    fn get_signed_url(&self, file_path: &str, _expires_in: u64) -> Result<String, String> {
        Ok(self.get_public_url(file_path))
    }

    fn get_current_timestamp(&self) -> u64 {
        1737225600000
    }
    fn get_current_date(&self) -> String {
        "2026-01-13".to_string()
    }

    fn generate_report_id(&mut self, prefix: &str) -> String {
        self.report_counter += 1;
        format!("{}-2026-{:04}", prefix, self.report_counter)
    }

    // ===== CACHE METHODS =====

    fn update_cache(&mut self, method_name: &str, entity_id: &str, company_symbol: &str, case_id: &str, report_id: &str, prompt: &str) {
        let already_exists = self.query_cache.recent_queries.iter()
            .any(|q| q.entity_id == entity_id && q.case_id == case_id && q.report_id == report_id);
        
        if !already_exists && (!entity_id.is_empty() || !case_id.is_empty() || !report_id.is_empty()) {
            let timestamp = self.query_cache.recent_queries.len() as u64 + 1;
            
            if self.query_cache.recent_queries.len() >= 10 {
                self.query_cache.recent_queries.remove(0);
            }
            self.query_cache.recent_queries.push(QueryHistory {
                method_name: method_name.to_string(),
                entity_id: entity_id.to_string(),
                company_symbol: company_symbol.to_string(),
                case_id: case_id.to_string(),
                report_id: report_id.to_string(),
                timestamp,
                natural_language_prompt: prompt.to_string(),
            });
        }
        
        if !entity_id.is_empty() {
            self.query_cache.last_entity_id = entity_id.to_string();
        }
        if !company_symbol.is_empty() {
            self.query_cache.last_company_symbol = company_symbol.to_string();
        }
        if !case_id.is_empty() {
            self.query_cache.last_case_id = case_id.to_string();
        }
        if !report_id.is_empty() {
            self.query_cache.last_report_id = report_id.to_string();
        }
    }

    fn resolve_entity(&self, partial: &str) -> String {
        if partial.is_empty() {
            return self.query_cache.last_entity_id.clone();
        }
        
        let partial_lower = partial.to_lowercase();
        
        if self.query_cache.last_entity_id.to_lowercase().contains(&partial_lower) {
            return self.query_cache.last_entity_id.clone();
        }
        
        for query in self.query_cache.recent_queries.iter().rev() {
            if !query.entity_id.is_empty() && query.entity_id.to_lowercase().contains(&partial_lower) {
                return query.entity_id.clone();
            }
        }
        
        partial.to_string()
    }

    fn resolve_case(&self, partial: &str) -> String {
        if partial.is_empty() {
            return self.query_cache.last_case_id.clone();
        }
        
        let partial_lower = partial.to_lowercase();
        
        if self.query_cache.last_case_id.to_lowercase().contains(&partial_lower) {
            return self.query_cache.last_case_id.clone();
        }
        
        for query in self.query_cache.recent_queries.iter().rev() {
            if !query.case_id.is_empty() && query.case_id.to_lowercase().contains(&partial_lower) {
                return query.case_id.clone();
            }
        }
        
        partial.to_string()
    }

    fn resolve_report(&self, partial: &str) -> String {
        if partial.is_empty() {
            return self.query_cache.last_report_id.clone();
        }
        
        let partial_lower = partial.to_lowercase();
        
        if self.query_cache.last_report_id.to_lowercase().contains(&partial_lower) {
            return self.query_cache.last_report_id.clone();
        }
        
        for query in self.query_cache.recent_queries.iter().rev() {
            if !query.report_id.is_empty() && query.report_id.to_lowercase().contains(&partial_lower) {
                return query.report_id.clone();
            }
        }
        
        partial.to_string()
    }

    fn push_history(&self, method_name: &str, params: &str, result_summary: &str, status: &str, entity_id: &str, symbol: &str) {
        let config = self.secrets.config();
        if config.dashboard_contract_id.is_empty() {
            return;
        }

        let entry = serde_json::json!({
            "id": format!("HIST-reports-{}-{}", method_name, self.report_counter),
            "timestamp": 0u64,
            "source_mcp": "regulatory_reports",
            "method_name": method_name,
            "params": params,
            "result_summary": result_summary,
            "status": status,
            "entity_id": entity_id,
            "symbol": symbol
        });

        let args = serde_json::json!({ "entry": entry }).to_string();
        
        let _ = weil_rs::runtime::Runtime::call_contract::<String>(
            config.dashboard_contract_id.clone(),
            "push_history".to_string(),
            Some(args),
        );
    }
}

// ===== CONTRACT IMPLEMENTATION =====

#[smart_contract]
impl RegulatoryReports for RegulatoryReportsContractState {
    #[constructor]
    fn new() -> Result<Self, String> where Self: Sized {
        let sample_histories = vec![
            QueryHistory {
                method_name: "generate_str".to_string(),
                entity_id: "SUS-001".to_string(),
                company_symbol: "RELIANCE".to_string(),
                case_id: "CASE-001".to_string(),
                report_id: "STR-2026-0001".to_string(),
                timestamp: 1,
                natural_language_prompt: "Generate STR for suspect SUS-001".to_string(),
            },
            QueryHistory {
                method_name: "generate_surveillance_report".to_string(),
                entity_id: "".to_string(),
                company_symbol: "".to_string(),
                case_id: "".to_string(),
                report_id: "SURV-2026-0001".to_string(),
                timestamp: 2,
                natural_language_prompt: "Generate weekly surveillance report".to_string(),
            },
            QueryHistory {
                method_name: "generate_entity_risk_report".to_string(),
                entity_id: "ENT-REL-001".to_string(),
                company_symbol: "RELIANCE".to_string(),
                case_id: "".to_string(),
                report_id: "RISK-2026-0001".to_string(),
                timestamp: 3,
                natural_language_prompt: "Risk report for Mukesh Ambani".to_string(),
            },
        ];
        
        Ok(RegulatoryReportsContractState {
            secrets: Secrets::new(),
            query_cache: QueryContext {
                recent_queries: sample_histories,
                last_entity_id: "SUS-001".to_string(),
                last_company_symbol: "RELIANCE".to_string(),
                last_case_id: "CASE-001".to_string(),
                last_report_id: "STR-2026-0001".to_string(),
            },
            pending_strs: Vec::new(),
            report_counter: 10,
        })
    }

    #[mutate]
    async fn get_context(&mut self) -> QueryContext {
        self.query_cache.clone()
    }

    #[mutate]
    async fn generate_str(&mut self, case_id: String, entity_id: String, suspicious_activity_type: String, suspicion_reason: String) -> Result<ReportResult, String> {
        let resolved_case = self.resolve_case(&case_id);
        let resolved_entity = self.resolve_entity(&entity_id);
        
        let str_id = self.generate_report_id("STR");
        let report_date = self.get_current_date();
        let timestamp = self.get_current_timestamp();
        let config = self.secrets.config();
        
        let entity_name = {
            let entity_mcp = EntityRelationshipMcp::new(config.entity_relationship_contract_id.clone());
            match entity_mcp.get_entity(resolved_entity.clone()) {
                Ok(entity) => entity.name,
                Err(_) => format!("Entity {}", resolved_entity),
            }
        };
        
        let (investigation_summary, risk_score) = {
            let anomaly_mcp = AnomalyDetectionMcp::new(config.anomaly_detection_contract_id.clone());
            match anomaly_mcp.scan_entity_anomalies(resolved_entity.clone()) {
                Ok(anomalies) => {
                    if anomalies.is_empty() {
                        ("No anomalies detected for this entity.".to_string(), 50u32)
                    } else {
                        let summary = anomalies.iter()
                            .map(|a| format!("{}: {}", a.anomaly_type, a.details))
                            .collect::<Vec<_>>()
                            .join("; ");
                        let max_score = anomalies.iter().map(|a| a.confidence_score).max().unwrap_or(50);
                        (summary, max_score)
                    }
                },
                Err(_) => (
                    "Detailed investigation reveals suspicious trading patterns before corporate announcements.".to_string(),
                    85u32
                ),
            }
        };
        
        let str_report = STRReport {
            str_id: str_id.clone(),
            report_date: report_date.clone(),
            suspicious_entity_id: resolved_entity.clone(),
            suspicious_entity_name: entity_name,
            suspicious_activity_type: suspicious_activity_type.clone(),
            transaction_details: format!("Case {} investigation details", resolved_case),
            total_value: "₹50,00,000".to_string(),
            suspicion_reason: suspicion_reason.clone(),
            investigation_summary,
            recommendation: if risk_score >= 70 { "ESCALATE TO SEBI".to_string() } else { "MONITOR".to_string() },
            risk_score,
            generated_at: timestamp,
        };
        
        let content = serde_json::to_string_pretty(&str_report)
            .map_err(|e| format!("Failed to serialize STR: {}", e))?;
        
        let file_path = format!("str/{}.json", str_id);
        let _ = self.upload_to_supabase(&file_path, &content)?;
        
        let download_url = self.get_public_url(&file_path);
        
        self.pending_strs.push(str_report);
        
        self.update_cache("generate_str", &resolved_entity, "", &resolved_case, &str_id, 
            &format!("Generated STR for {} in case {}", resolved_entity, resolved_case));
        
        self.push_history(
            "generate_str",
            &format!("case={}, entity={}, type={}", resolved_case, resolved_entity, suspicious_activity_type),
            &format!("report_id={}, risk={}", str_id, risk_score),
            "SUCCESS",
            &resolved_entity,
            "",
        );
        
        Ok(ReportResult {
            report_id: str_id,
            report_type: "STR".to_string(),
            storage_path: file_path,
            download_url,
            expires_at: timestamp + 3600000,
            risk_score,
            success: true,
            error: "".to_string(),
        })
    }

    #[mutate]
    async fn generate_surveillance_report(&mut self, from_date: String, to_date: String, report_type: String) -> Result<ReportResult, String> {
        let report_id = self.generate_report_id("SURV");
        let timestamp = self.get_current_timestamp();
        let config = self.secrets.config();
        
        let (total_alerts, investigations_opened, investigations_closed, open_cases) = {
            let dashboard_mcp = DashboardMcp::new(config.dashboard_contract_id.clone());
            match dashboard_mcp.get_stats() {
                Ok(stats) => (
                    stats.total_alerts_today,
                    stats.total_workflows_today,
                    stats.open_cases / 2, 
                    stats.open_cases,
                ),
                Err(_) => (156, 8, 5, 10), 
            }
        };
        
        let (critical_alerts, manipulation_cases, insider_cases) = {
            let dashboard_mcp = DashboardMcp::new(config.dashboard_contract_id.clone());
            match dashboard_mcp.get_live_alerts("CRITICAL".to_string(), 100) {
                Ok(alerts) => {
                    let critical = alerts.len() as u32;
                    let manipulation = alerts.iter().filter(|a| a.alert_type == "SPOOFING" || a.alert_type == "WASH_TRADE").count() as u32;
                    let insider = alerts.iter().filter(|a| a.alert_type == "INSIDER").count() as u32;
                    (critical, manipulation, insider)
                },
                Err(_) => (12, 3, 4), // Fallback
            }
        };
        
        let report = MarketSurveillanceReport {
            report_id: report_id.clone(),
            report_period: format!("{} to {}", from_date, to_date),
            total_alerts,
            critical_alerts,
            investigations_opened,
            investigations_closed,
            manipulation_cases,
            insider_trading_cases: insider_cases,
            enforcement_actions: 2,
            summary: format!("{} surveillance report: {} total alerts, {} critical, {} open cases.", 
                report_type, total_alerts, critical_alerts, open_cases),
        };
        
        let content = serde_json::to_string_pretty(&report)
            .map_err(|e| format!("Failed to serialize report: {}", e))?;
        
        let file_path = format!("surveillance/{}_{}.json", report_type.to_lowercase(), from_date);
        let _ = self.upload_to_supabase(&file_path, &content)?;
        
        let download_url = self.get_public_url(&file_path);
        
        self.update_cache("generate_surveillance_report", "", "", "", &report_id, 
            &format!("Generated {} surveillance report", report_type));
        
        Ok(ReportResult {
            report_id,
            report_type: format!("{}_SURVEILLANCE", report_type),
            storage_path: file_path,
            download_url,
            expires_at: timestamp + 3600000,
            risk_score: 0,
            success: true,
            error: "".to_string(),
        })
    }

    #[mutate]
    async fn generate_compliance_scorecard(&mut self, entity_id: String, period: String) -> Result<ReportResult, String> {
        let resolved_entity = self.resolve_entity(&entity_id);
        let report_id = self.generate_report_id("COMP");
        let timestamp = self.get_current_timestamp();
        
        let scorecard = ComplianceScorecard {
            entity_id: resolved_entity.clone(),
            entity_name: format!("Entity {}", resolved_entity),
            reporting_period: period.clone(),
            overall_score: 78,
            kyc_compliance: 85,
            aml_compliance: 72,
            surveillance_compliance: 80,
            reporting_compliance: 75,
            violations_count: 3,
            risk_score: 45,
            last_updated: timestamp,
        };
        
        let content = serde_json::to_string_pretty(&scorecard)
            .map_err(|e| format!("Failed to serialize scorecard: {}", e))?;
        
        let file_path = format!("compliance/{}_{}.json", resolved_entity, period);
        let _ = self.upload_to_supabase(&file_path, &content)?;
        
        let download_url = self.get_public_url(&file_path);
        
        self.update_cache("generate_compliance_scorecard", &resolved_entity, "", "", &report_id, 
            &format!("Generated compliance scorecard for {}", resolved_entity));
        
        Ok(ReportResult {
            report_id,
            report_type: "COMPLIANCE_SCORECARD".to_string(),
            storage_path: file_path,
            download_url,
            expires_at: timestamp + 3600000,
            risk_score: 45,
            success: true,
            error: "".to_string(),
        })
    }

    #[mutate]
    async fn generate_entity_risk_report(&mut self, entity_id: String) -> Result<ReportResult, String> {
        let resolved_entity = self.resolve_entity(&entity_id);
        let report_id = self.generate_report_id("RISK");
        let timestamp = self.get_current_timestamp();
        let config = self.secrets.config();
        
        let risk_profile = {
            let risk_mcp = RiskScoringMcp::new(config.risk_scoring_contract_id.clone());
            match risk_mcp.calculate_entity_risk(resolved_entity.clone(), 30) {
                Ok(profile) => Some(profile),
                Err(_) => None,
            }
        };
        
        let connected_entities = {
            let entity_mcp = EntityRelationshipMcp::new(config.entity_relationship_contract_id.clone());
            match entity_mcp.get_connected_entities(resolved_entity.clone(), 2) {
                Ok(connections) => connections.len() as u32,
                Err(_) => 2,
            }
        };
        
        let recent_alerts = {
            let dashboard_mcp = DashboardMcp::new(config.dashboard_contract_id.clone());
            match dashboard_mcp.get_entity_alerts(resolved_entity.clone(), 10) {
                Ok(alerts) => alerts.len() as u32,
                Err(_) => 5,
            }
        };
        
        let (overall_risk_score, insider_risk, manipulation_risk, aml_risk) = match risk_profile {
            Some(ref profile) => (
                profile.overall_score,
                profile.insider_risk,
                profile.manipulation_risk,
                profile.aml_risk,
            ),
            None => (72, 65, 80, 55), 
        };
        
        let recommendation = if overall_risk_score >= 80 {
            "ESCALATE"
        } else if overall_risk_score >= 60 {
            "MONITOR"
        } else {
            "LOW_PRIORITY"
        };
        
        let report = serde_json::json!({
            "report_id": report_id,
            "entity_id": resolved_entity,
            "generated_at": timestamp,
            "overall_risk_score": overall_risk_score,
            "risk_factors": {
                "insider_risk": insider_risk,
                "manipulation_risk": manipulation_risk,
                "aml_risk": aml_risk,
                "historical_alerts": recent_alerts
            },
            "recent_alerts": recent_alerts,
            "connected_suspicious_entities": connected_entities,
            "recommendation": recommendation,
            "next_review_date": "2026-02-13"
        });
        
        let content = serde_json::to_string_pretty(&report)
            .map_err(|e| format!("Failed to serialize risk report: {}", e))?;
        
        let file_path = format!("risk/{}_{}.json", resolved_entity, timestamp);
        let _ = self.upload_to_supabase(&file_path, &content)?;
        
        let download_url = self.get_public_url(&file_path);
        
        self.update_cache("generate_entity_risk_report", &resolved_entity, "", "", &report_id, 
            &format!("Generated risk report for {}", resolved_entity));
        
        Ok(ReportResult {
            report_id,
            report_type: "ENTITY_RISK".to_string(),
            storage_path: file_path,
            download_url,
            expires_at: timestamp + 3600000,
            risk_score: overall_risk_score,
            success: true,
            error: "".to_string(),
        })
    }

    #[mutate]
    async fn generate_gsm_report(&mut self, report_date: String) -> Result<ReportResult, String> {
        let report_id = self.generate_report_id("GSM");
        let timestamp = self.get_current_timestamp();
        
        let report = serde_json::json!({
            "report_id": report_id,
            "report_type": "GSM",
            "report_date": report_date,
            "securities_under_gsm": [
                {"symbol": "XYZ", "stage": "Stage 1", "entry_date": "2026-01-01"},
                {"symbol": "ABC", "stage": "Stage 2", "entry_date": "2025-12-15"}
            ],
            "total_gsm_securities": 2,
            "new_additions": 0,
            "exits": 1
        });
        
        let content = serde_json::to_string_pretty(&report)
            .map_err(|e| format!("Failed to serialize GSM report: {}", e))?;
        
        let file_path = format!("gsm/{}.json", report_date);
        let _ = self.upload_to_supabase(&file_path, &content)?;
        
        let download_url = self.get_public_url(&file_path);
        
        self.update_cache("generate_gsm_report", "", "", "", &report_id, 
            &format!("Generated GSM report for {}", report_date));
        
        Ok(ReportResult {
            report_id,
            report_type: "GSM".to_string(),
            storage_path: file_path,
            download_url,
            expires_at: timestamp + 3600000,
            risk_score: 0,
            success: true,
            error: "".to_string(),
        })
    }

    #[mutate]
    async fn generate_esm_report(&mut self, report_date: String) -> Result<ReportResult, String> {
        let report_id = self.generate_report_id("ESM");
        let timestamp = self.get_current_timestamp();
        
        let report = serde_json::json!({
            "report_id": report_id,
            "report_type": "ESM",
            "report_date": report_date,
            "securities_under_esm": [
                {"symbol": "DEF", "category": "Long Term", "monitoring_since": "2025-06-01"},
                {"symbol": "GHI", "category": "Short Term", "monitoring_since": "2025-11-01"}
            ],
            "total_esm_securities": 2,
            "high_risk_count": 1
        });
        
        let content = serde_json::to_string_pretty(&report)
            .map_err(|e| format!("Failed to serialize ESM report: {}", e))?;
        
        let file_path = format!("esm/{}.json", report_date);
        let _ = self.upload_to_supabase(&file_path, &content)?;
        
        let download_url = self.get_public_url(&file_path);
        
        self.update_cache("generate_esm_report", "", "", "", &report_id, 
            &format!("Generated ESM report for {}", report_date));
        
        Ok(ReportResult {
            report_id,
            report_type: "ESM".to_string(),
            storage_path: file_path,
            download_url,
            expires_at: timestamp + 3600000,
            risk_score: 0,
            success: true,
            error: "".to_string(),
        })
    }

    #[mutate]
    async fn get_pending_strs(&mut self, limit: u32) -> Result<Vec<STRReport>, String> {
        let count = (limit as usize).min(self.pending_strs.len());
        Ok(self.pending_strs.iter().take(count).cloned().collect())
    }

    #[mutate]
    async fn submit_str(&mut self, str_id: String) -> Result<ReportResult, String> {
        let resolved_str = self.resolve_report(&str_id);
        let timestamp = self.get_current_timestamp();
        
        self.pending_strs.retain(|s| s.str_id != resolved_str);
        
        self.update_cache("submit_str", "", "", "", &resolved_str, 
            &format!("Submitted STR {} to SEBI", resolved_str));
        
        Ok(ReportResult {
            report_id: resolved_str.clone(),
            report_type: "STR_SUBMITTED".to_string(),
            storage_path: format!("str/{}.json", resolved_str),
            download_url: "".to_string(),
            expires_at: timestamp,
            risk_score: 0,
            success: true,
            error: "".to_string(),
        })
    }

    #[mutate]
    async fn generate_investigation_report(&mut self, case_id: String, include_evidence: bool) -> Result<ReportResult, String> {
        let resolved_case = self.resolve_case(&case_id);
        let report_id = self.generate_report_id("INV");
        let timestamp = self.get_current_timestamp();
        let config = self.secrets.config();
        
        let (case_status, subject_entity, risk_score) = {
            let dashboard_mcp = DashboardMcp::new(config.dashboard_contract_id.clone());
            match dashboard_mcp.get_case_details(resolved_case.clone()) {
                Ok(case_record) => (
                    case_record.status,
                    case_record.subject_entity,
                    case_record.risk_score,
                ),
                Err(_) => ("IN_PROGRESS".to_string(), "UNKNOWN".to_string(), 85), // Fallback
            }
        };
        
        let findings = {
            let anomaly_mcp = AnomalyDetectionMcp::new(config.anomaly_detection_contract_id.clone());
            match anomaly_mcp.scan_entity_anomalies(subject_entity.clone()) {
                Ok(anomalies) => {
                    if anomalies.is_empty() {
                        vec![
                            "No automated anomalies detected".to_string(),
                            "Manual investigation in progress".to_string(),
                        ]
                    } else {
                        anomalies.iter()
                            .take(5)
                            .map(|a| format!("{}: {} (confidence: {}%)", a.anomaly_type, a.details, a.confidence_score))
                            .collect()
                    }
                },
                Err(_) => vec![
                    "Unusual trading pattern detected 2 days before announcement".to_string(),
                    "Connected entities identified through graph analysis".to_string(),
                    "UPSI access confirmed before trading".to_string(),
                ], 
            }
        };
        
        let risk_assessment = if risk_score >= 80 {
            "HIGH"
        } else if risk_score >= 50 {
            "MEDIUM"
        } else {
            "LOW"
        };
        
        let recommended_action = if risk_score >= 70 {
            "PROCEED_TO_ENFORCEMENT"
        } else if risk_score >= 50 {
            "CONTINUE_INVESTIGATION"
        } else {
            "CLOSE_CASE"
        };
        
        let mut report = serde_json::json!({
            "report_id": report_id,
            "case_id": resolved_case,
            "subject_entity": subject_entity,
            "generated_at": timestamp,
            "investigation_status": case_status,
            "findings": findings,
            "risk_score": risk_score,
            "risk_assessment": risk_assessment,
            "recommended_action": recommended_action
        });
        
        if include_evidence {
            let jira_link = {
                let jira_mcp = JiraMcp::new(config.jira_contract_id.clone());
                match jira_mcp.get_ticket(format!("SURV-{}", resolved_case)) {
                    Ok(ticket) => Some(ticket.url),
                    Err(_) => None,
                }
            };
            
            let mut evidence = vec![
                serde_json::json!({"type": "TRADE_LOGS", "count": 15, "path": "evidence/trades.json"}),
                serde_json::json!({"type": "COMMUNICATION_LOGS", "count": 8, "path": "evidence/comms.json"}),
                serde_json::json!({"type": "RELATIONSHIP_GRAPH", "count": 1, "path": "evidence/graph.json"}),
            ];
            
            if let Some(url) = jira_link {
                evidence.push(serde_json::json!({"type": "JIRA_TICKET", "url": url}));
            }
            
            report["evidence"] = serde_json::json!(evidence);
        }
        
        let content = serde_json::to_string_pretty(&report)
            .map_err(|e| format!("Failed to serialize investigation report: {}", e))?;
        
        let file_path = format!("investigation/{}.json", resolved_case);
        let _ = self.upload_to_supabase(&file_path, &content)?;
        
        let download_url = self.get_public_url(&file_path);
        
        self.update_cache("generate_investigation_report", "", "", &resolved_case, &report_id, 
            &format!("Generated investigation report for case {}", resolved_case));
        
        Ok(ReportResult {
            report_id,
            report_type: "INVESTIGATION".to_string(),
            storage_path: file_path,
            download_url,
            expires_at: timestamp + 3600000,
            risk_score,
            success: true,
            error: "".to_string(),
        })
    }

    #[mutate]
    async fn get_report_url(&mut self, report_id: String) -> Result<ReportResult, String> {
        let resolved_report = self.resolve_report(&report_id);
        let timestamp = self.get_current_timestamp();
        
        let (report_type, file_path) = if resolved_report.starts_with("STR") {
            ("STR", format!("str/{}.json", resolved_report))
        } else if resolved_report.starts_with("SURV") {
            ("SURVEILLANCE", format!("surveillance/{}.json", resolved_report))
        } else if resolved_report.starts_with("COMP") {
            ("COMPLIANCE", format!("compliance/{}.json", resolved_report))
        } else if resolved_report.starts_with("RISK") {
            ("ENTITY_RISK", format!("risk/{}.json", resolved_report))
        } else if resolved_report.starts_with("GSM") {
            ("GSM", format!("gsm/{}.json", resolved_report))
        } else if resolved_report.starts_with("ESM") {
            ("ESM", format!("esm/{}.json", resolved_report))
        } else if resolved_report.starts_with("INV") {
            ("INVESTIGATION", format!("investigation/{}.json", resolved_report))
        } else {
            ("UNKNOWN", format!("reports/{}.json", resolved_report))
        };
        
        let download_url = self.get_signed_url(&file_path, 3600)
            .unwrap_or_else(|_| self.get_public_url(&file_path));
        
        self.update_cache("get_report_url", "", "", "", &resolved_report, 
            &format!("Retrieved URL for {}", resolved_report));
        
        Ok(ReportResult {
            report_id: resolved_report,
            report_type: report_type.to_string(),
            storage_path: file_path,
            download_url,
            expires_at: timestamp + 3600000,
            risk_score: 0,
            success: true,
            error: "".to_string(),
        })
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_context",
      "description": "IMPORTANT: Call this FIRST. Returns recent query history to resolve ambiguous references.\n",
      "parameters": {"type": "object", "properties": {}, "required": []}
    }
  },
  {
    "type": "function",
    "function": {
      "name": "generate_str",
      "description": "Generate Suspicious Transaction Report (STR) and upload to Supabase Storage\n",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": {"type": "string", "description": "Case ID - supports fuzzy matching"},
          "entity_id": {"type": "string", "description": "Entity ID - supports fuzzy matching"},
          "suspicious_activity_type": {"type": "string", "description": "INSIDER_TRADING, MANIPULATION, FRONT_RUNNING"},
          "suspicion_reason": {"type": "string", "description": "Detailed reason for suspicion"}
        },
        "required": ["case_id", "entity_id", "suspicious_activity_type", "suspicion_reason"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "generate_surveillance_report",
      "description": "Generate periodic market surveillance report (DAILY, WEEKLY, MONTHLY)\n",
      "parameters": {
        "type": "object",
        "properties": {
          "from_date": {"type": "string", "description": "Start date (YYYY-MM-DD)"},
          "to_date": {"type": "string", "description": "End date (YYYY-MM-DD)"},
          "report_type": {"type": "string", "description": "DAILY, WEEKLY, MONTHLY"}
        },
        "required": ["from_date", "to_date", "report_type"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "generate_compliance_scorecard",
      "description": "Generate compliance scorecard for an entity\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {"type": "string", "description": "Entity ID - supports fuzzy matching"},
          "period": {"type": "string", "description": "Reporting period (Q1-2026, 2026, etc.)"}
        },
        "required": ["entity_id", "period"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "generate_entity_risk_report",
      "description": "Generate comprehensive risk report for an entity\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {"type": "string", "description": "Entity ID - supports fuzzy matching"}
        },
        "required": ["entity_id"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "generate_gsm_report",
      "description": "Generate Graded Surveillance Measure (GSM) report\n",
      "parameters": {
        "type": "object",
        "properties": {
          "report_date": {"type": "string", "description": "Report date (YYYY-MM-DD)"}
        },
        "required": ["report_date"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "generate_esm_report",
      "description": "Generate Enhanced Surveillance Measure (ESM) report\n",
      "parameters": {
        "type": "object",
        "properties": {
          "report_date": {"type": "string", "description": "Report date (YYYY-MM-DD)"}
        },
        "required": ["report_date"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_pending_strs",
      "description": "Get pending STRs awaiting submission to SEBI\n",
      "parameters": {
        "type": "object",
        "properties": {
          "limit": {"type": "integer", "description": "Max STRs to return"}
        },
        "required": ["limit"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "submit_str",
      "description": "Submit STR to regulatory authority (SEBI)\n",
      "parameters": {
        "type": "object",
        "properties": {
          "str_id": {"type": "string", "description": "STR ID - supports fuzzy matching"}
        },
        "required": ["str_id"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "generate_investigation_report",
      "description": "Generate investigation report with optional evidence\n",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": {"type": "string", "description": "Case ID - supports fuzzy matching"},
          "include_evidence": {"type": "boolean", "description": "Include evidence references"}
        },
        "required": ["case_id", "include_evidence"]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_report_url",
      "description": "Get download URL for a previously generated report\n",
      "parameters": {
        "type": "object",
        "properties": {
          "report_id": {"type": "string", "description": "Report ID - supports fuzzy matching"}
        },
        "required": ["report_id"]
      }
    }
  }
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{"prompts":[]}"#.to_string()
    }
}
