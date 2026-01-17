
use serde::{Deserialize, Serialize};
use anyhow::Result;
use weil_rs::runtime::Runtime;


#[derive(Debug, Serialize, Deserialize, Clone)]
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


#[derive(Debug, Serialize, Deserialize)]
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


#[derive(Debug, Serialize, Deserialize)]
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


#[derive(Debug, Serialize, Deserialize)]
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
pub struct QueryHistory {
    pub method_name: String,
    pub entity_id: String,
    pub company_symbol: String,
    pub case_id: String,
    pub report_id: String,
    pub timestamp: u64,
    pub natural_language_prompt: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct QueryContext {
    pub recent_queries: Vec<QueryHistory>,
    pub last_entity_id: String,
    pub last_company_symbol: String,
    pub last_case_id: String,
    pub last_report_id: String,
}


pub struct RegulatoryReportsProxy {
    contract_id: String,
}

impl RegulatoryReportsProxy {
    pub fn new(contract_id: String) -> Self {
        RegulatoryReportsProxy {
            contract_id,
        }
    }
}

impl RegulatoryReportsProxy {
    pub fn get_context(&self) -> Result<QueryContext> {
        let serialized_args = None;

        let resp = Runtime::call_contract::<QueryContext>(
            self.contract_id.to_string(),
            "get_context".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn generate_str(&self, case_id: String, entity_id: String, suspicious_activity_type: String, suspicion_reason: String) -> Result<ReportResult> {

        #[derive(Debug, Serialize)]
        struct generate_strArgs {
            case_id: String,
            entity_id: String,
            suspicious_activity_type: String,
            suspicion_reason: String,
        }

        let serialized_args = Some(serde_json::to_string(&generate_strArgs { case_id, entity_id, suspicious_activity_type, suspicion_reason }).unwrap());

        let resp = Runtime::call_contract::<ReportResult>(
            self.contract_id.to_string(),
            "generate_str".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn generate_surveillance_report(&self, from_date: String, to_date: String, report_type: String) -> Result<ReportResult> {

        #[derive(Debug, Serialize)]
        struct generate_surveillance_reportArgs {
            from_date: String,
            to_date: String,
            report_type: String,
        }

        let serialized_args = Some(serde_json::to_string(&generate_surveillance_reportArgs { from_date, to_date, report_type }).unwrap());

        let resp = Runtime::call_contract::<ReportResult>(
            self.contract_id.to_string(),
            "generate_surveillance_report".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn generate_compliance_scorecard(&self, entity_id: String, period: String) -> Result<ReportResult> {

        #[derive(Debug, Serialize)]
        struct generate_compliance_scorecardArgs {
            entity_id: String,
            period: String,
        }

        let serialized_args = Some(serde_json::to_string(&generate_compliance_scorecardArgs { entity_id, period }).unwrap());

        let resp = Runtime::call_contract::<ReportResult>(
            self.contract_id.to_string(),
            "generate_compliance_scorecard".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn generate_entity_risk_report(&self, entity_id: String) -> Result<ReportResult> {

        #[derive(Debug, Serialize)]
        struct generate_entity_risk_reportArgs {
            entity_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&generate_entity_risk_reportArgs { entity_id }).unwrap());

        let resp = Runtime::call_contract::<ReportResult>(
            self.contract_id.to_string(),
            "generate_entity_risk_report".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn generate_gsm_report(&self, report_date: String) -> Result<ReportResult> {

        #[derive(Debug, Serialize)]
        struct generate_gsm_reportArgs {
            report_date: String,
        }

        let serialized_args = Some(serde_json::to_string(&generate_gsm_reportArgs { report_date }).unwrap());

        let resp = Runtime::call_contract::<ReportResult>(
            self.contract_id.to_string(),
            "generate_gsm_report".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn generate_esm_report(&self, report_date: String) -> Result<ReportResult> {

        #[derive(Debug, Serialize)]
        struct generate_esm_reportArgs {
            report_date: String,
        }

        let serialized_args = Some(serde_json::to_string(&generate_esm_reportArgs { report_date }).unwrap());

        let resp = Runtime::call_contract::<ReportResult>(
            self.contract_id.to_string(),
            "generate_esm_report".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_pending_strs(&self, limit: u32) -> Result<Vec<STRReport>> {

        #[derive(Debug, Serialize)]
        struct get_pending_strsArgs {
            limit: u32,
        }

        let serialized_args = Some(serde_json::to_string(&get_pending_strsArgs { limit }).unwrap());

        let resp = Runtime::call_contract::<Vec<STRReport>>(
            self.contract_id.to_string(),
            "get_pending_strs".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn submit_str(&self, str_id: String) -> Result<ReportResult> {

        #[derive(Debug, Serialize)]
        struct submit_strArgs {
            str_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&submit_strArgs { str_id }).unwrap());

        let resp = Runtime::call_contract::<ReportResult>(
            self.contract_id.to_string(),
            "submit_str".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn generate_investigation_report(&self, case_id: String, include_evidence: bool) -> Result<ReportResult> {

        #[derive(Debug, Serialize)]
        struct generate_investigation_reportArgs {
            case_id: String,
            include_evidence: bool,
        }

        let serialized_args = Some(serde_json::to_string(&generate_investigation_reportArgs { case_id, include_evidence }).unwrap());

        let resp = Runtime::call_contract::<ReportResult>(
            self.contract_id.to_string(),
            "generate_investigation_report".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_report_url(&self, report_id: String) -> Result<ReportResult> {

        #[derive(Debug, Serialize)]
        struct get_report_urlArgs {
            report_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&get_report_urlArgs { report_id }).unwrap());

        let resp = Runtime::call_contract::<ReportResult>(
            self.contract_id.to_string(),
            "get_report_url".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

}
