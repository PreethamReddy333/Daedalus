//! Cross-contract bindings for Surveillance Dashboard
//!
//! Provides proxy methods to call the deployed Surveillance Dashboard contract.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use weil_rs::runtime::Runtime;

/// Proxy struct for Surveillance Dashboard cross-contract calls
pub struct DashboardMcp {
    contract_id: String,
}

impl DashboardMcp {
    pub fn new(contract_id: String) -> Self {
        DashboardMcp { contract_id }
    }
}

// ===== Data Types for Dashboard =====

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SurveillanceStats {
    pub total_alerts_today: u32,
    pub total_workflows_today: u32,
    pub open_cases: u32,
    pub high_risk_entities: u32,
    pub compliance_score: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskEntity {
    pub entity_id: String,
    pub entity_name: String,
    pub risk_score: u32,
    pub alert_count: u32,
    pub last_alert_at: u64,
}

impl DashboardMcp {
    /// Get surveillance statistics
    pub fn get_stats(&self) -> Result<SurveillanceStats> {
        let serialized_args = None;

        let resp = Runtime::call_contract::<SurveillanceStats>(
            self.contract_id.clone(),
            "get_stats".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Get cases by status
    pub fn get_cases_by_status(&self, status: String, limit: u32) -> Result<Vec<CaseRecord>> {
        #[derive(Debug, Serialize)]
        struct GetCasesByStatusArgs {
            status: String,
            limit: u32,
        }

        let serialized_args = Some(serde_json::to_string(&GetCasesByStatusArgs {
            status,
            limit,
        })?);

        let resp = Runtime::call_contract::<Vec<CaseRecord>>(
            self.contract_id.clone(),
            "get_cases_by_status".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Get live alerts with severity filter
    pub fn get_live_alerts(&self, severity_filter: String, limit: u32) -> Result<Vec<Alert>> {
        #[derive(Debug, Serialize)]
        struct GetLiveAlertsArgs {
            severity_filter: String,
            limit: u32,
        }

        let serialized_args = Some(serde_json::to_string(&GetLiveAlertsArgs {
            severity_filter,
            limit,
        })?);

        let resp = Runtime::call_contract::<Vec<Alert>>(
            self.contract_id.clone(),
            "get_live_alerts".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Get workflow execution history
    pub fn get_workflow_history(&self, workflow_type: String, limit: u32) -> Result<Vec<WorkflowExecution>> {
        #[derive(Debug, Serialize)]
        struct GetWorkflowHistoryArgs {
            workflow_type: String,
            limit: u32,
        }

        let serialized_args = Some(serde_json::to_string(&GetWorkflowHistoryArgs {
            workflow_type,
            limit,
        })?);

        let resp = Runtime::call_contract::<Vec<WorkflowExecution>>(
            self.contract_id.clone(),
            "get_workflow_history".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Get high-risk entities
    pub fn get_high_risk_entities(&self, min_risk_score: u32, limit: u32) -> Result<Vec<RiskEntity>> {
        #[derive(Debug, Serialize)]
        struct GetHighRiskEntitiesArgs {
            min_risk_score: u32,
            limit: u32,
        }

        let serialized_args = Some(serde_json::to_string(&GetHighRiskEntitiesArgs {
            min_risk_score,
            limit,
        })?);

        let resp = Runtime::call_contract::<Vec<RiskEntity>>(
            self.contract_id.clone(),
            "get_high_risk_entities".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Get specific case details
    pub fn get_case_details(&self, case_id: String) -> Result<CaseRecord> {
        #[derive(Debug, Serialize)]
        struct GetCaseDetailsArgs {
            case_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&GetCaseDetailsArgs { case_id })?);

        let resp = Runtime::call_contract::<CaseRecord>(
            self.contract_id.clone(),
            "get_case_details".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Push a new alert to the dashboard
    pub fn push_alert(&self, alert: Alert) -> Result<String> {
        #[derive(Debug, Serialize)]
        struct PushAlertArgs {
            alert: Alert,
        }

        let serialized_args = Some(serde_json::to_string(&PushAlertArgs { alert })?);

        let resp = Runtime::call_contract::<String>(
            self.contract_id.clone(),
            "push_alert".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Upsert a case record
    pub fn upsert_case(&self, case_record: CaseRecord) -> Result<String> {
        #[derive(Debug, Serialize)]
        struct UpsertCaseArgs {
            case_record: CaseRecord,
        }

        let serialized_args = Some(serde_json::to_string(&UpsertCaseArgs { case_record })?);

        let resp = Runtime::call_contract::<String>(
            self.contract_id.clone(),
            "upsert_case".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Get entity alerts
    pub fn get_entity_alerts(&self, entity_id: String, limit: u32) -> Result<Vec<Alert>> {
        #[derive(Debug, Serialize)]
        struct GetEntityAlertsArgs {
            entity_id: String,
            limit: u32,
        }

        let serialized_args = Some(serde_json::to_string(&GetEntityAlertsArgs {
            entity_id,
            limit,
        })?);

        let resp = Runtime::call_contract::<Vec<Alert>>(
            self.contract_id.clone(),
            "get_entity_alerts".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }
}
