
use serde::{Deserialize, Serialize};
use anyhow::Result;
use weil_rs::runtime::Runtime;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackNotifierConfig {
    pub webhook_url: String,
    pub default_channel: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessage {
    pub channel: String,
    pub text: String,
    pub username: String,
    pub icon_emoji: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AlertNotification {
    pub alert_id: String,
    pub alert_type: String,
    pub severity: String,
    pub symbol: String,
    pub entity_id: String,
    pub description: String,
    pub risk_score: u32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationResult {
    pub success: bool,
    pub message_id: String,
    pub timestamp: u64,
    pub error: String,
}


pub struct SlackNotifierProxy {
    contract_id: String,
}

impl SlackNotifierProxy {
    pub fn new(contract_id: String) -> Self {
        SlackNotifierProxy {
            contract_id,
        }
    }
}

impl SlackNotifierProxy {
    pub fn send_message(&self, channel: String, message: String) -> Result<NotificationResult> {

        #[derive(Debug, Serialize)]
        struct send_messageArgs {
            channel: String,
            message: String,
        }

        let serialized_args = Some(serde_json::to_string(&send_messageArgs { channel, message }).unwrap());

        let resp = Runtime::call_contract::<NotificationResult>(
            self.contract_id.to_string(),
            "send_message".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn send_alert(&self, alert_type: String, severity: String, symbol: String, entity_id: String, description: String, risk_score: u32) -> Result<NotificationResult> {

        #[derive(Debug, Serialize)]
        struct send_alertArgs {
            alert_type: String,
            severity: String,
            symbol: String,
            entity_id: String,
            description: String,
            risk_score: u32,
        }

        let serialized_args = Some(serde_json::to_string(&send_alertArgs { alert_type, severity, symbol, entity_id, description, risk_score }).unwrap());

        let resp = Runtime::call_contract::<NotificationResult>(
            self.contract_id.to_string(),
            "send_alert".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn send_case_update(&self, case_id: String, status: String, update_message: String, assigned_to: String) -> Result<NotificationResult> {

        #[derive(Debug, Serialize)]
        struct send_case_updateArgs {
            case_id: String,
            status: String,
            update_message: String,
            assigned_to: String,
        }

        let serialized_args = Some(serde_json::to_string(&send_case_updateArgs { case_id, status, update_message, assigned_to }).unwrap());

        let resp = Runtime::call_contract::<NotificationResult>(
            self.contract_id.to_string(),
            "send_case_update".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn send_workflow_complete(&self, workflow_id: String, workflow_type: String, result_summary: String, alert_count: u32) -> Result<NotificationResult> {

        #[derive(Debug, Serialize)]
        struct send_workflow_completeArgs {
            workflow_id: String,
            workflow_type: String,
            result_summary: String,
            alert_count: u32,
        }

        let serialized_args = Some(serde_json::to_string(&send_workflow_completeArgs { workflow_id, workflow_type, result_summary, alert_count }).unwrap());

        let resp = Runtime::call_contract::<NotificationResult>(
            self.contract_id.to_string(),
            "send_workflow_complete".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn send_daily_summary(&self, date: String, total_alerts: u32, critical_alerts: u32, open_cases: u32, new_cases: u32) -> Result<NotificationResult> {

        #[derive(Debug, Serialize)]
        struct send_daily_summaryArgs {
            date: String,
            total_alerts: u32,
            critical_alerts: u32,
            open_cases: u32,
            new_cases: u32,
        }

        let serialized_args = Some(serde_json::to_string(&send_daily_summaryArgs { date, total_alerts, critical_alerts, open_cases, new_cases }).unwrap());

        let resp = Runtime::call_contract::<NotificationResult>(
            self.contract_id.to_string(),
            "send_daily_summary".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

}
