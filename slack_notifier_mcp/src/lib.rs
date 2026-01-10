//! # Slack Notifier MCP Server
//!
//! Sends alerts and notifications to Slack channels via webhooks.
//! Provides real-time surveillance notifications to compliance teams.

use serde::{Deserialize, Serialize};
use weil_macros::{constructor, query, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::http::HttpClient;

// ===== CONFIGURATION =====

#[derive(Debug, Serialize, Deserialize, WeilType, Default, Clone)]
pub struct SlackNotifierConfig {
    pub webhook_url: String,
    pub default_channel: String,
}

// ===== DATA STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct SlackMessage {
    pub channel: String,
    pub text: String,
    pub username: String,
    pub icon_emoji: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct AlertNotification {
    pub alert_id: String,
    pub alert_type: String,
    pub severity: String,
    pub symbol: String,
    pub entity_id: String,
    pub description: String,
    pub risk_score: u32,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct NotificationResult {
    pub success: bool,
    pub message_id: String,
    pub timestamp: u64,
    pub error: String,
}

// ===== TRAIT DEFINITION =====

trait SlackNotifier {
    fn new() -> Result<Self, String> where Self: Sized;
    async fn send_message(&self, channel: String, message: String) -> Result<NotificationResult, String>;
    async fn send_alert(&self, alert_type: String, severity: String, symbol: String, entity_id: String, description: String, risk_score: u32) -> Result<NotificationResult, String>;
    async fn send_case_update(&self, case_id: String, status: String, update_message: String, assigned_to: String) -> Result<NotificationResult, String>;
    async fn send_workflow_complete(&self, workflow_id: String, workflow_type: String, result_summary: String, alert_count: u32) -> Result<NotificationResult, String>;
    async fn send_daily_summary(&self, date: String, total_alerts: u32, critical_alerts: u32, open_cases: u32, new_cases: u32) -> Result<NotificationResult, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

// ===== CONTRACT STATE =====

#[derive(Serialize, Deserialize, WeilType)]
pub struct SlackNotifierContractState {
    secrets: Secrets<SlackNotifierConfig>,
}

// ===== HELPER METHODS =====

impl SlackNotifierContractState {
    fn get_severity_emoji(&self, severity: &str) -> &'static str {
        match severity {
            "CRITICAL" => "🚨",
            "HIGH" => "🔴",
            "MEDIUM" => "🟡",
            "LOW" => "🟢",
            _ => "ℹ️",
        }
    }
    
    async fn send_to_slack(&self, text: String) -> Result<NotificationResult, String> {
        let config = self.secrets.config();
        
        if config.webhook_url.is_empty() {
            return Ok(NotificationResult {
                success: false,
                message_id: "".to_string(),
                timestamp: 0,
                error: "Webhook URL not configured".to_string(),
            });
        }
        
        let payload = serde_json::json!({
            "text": text,
            "channel": config.default_channel
        });
        
        let client = HttpClient::new();
        let response = client
            .post(&config.webhook_url)
            .header("Content-Type", "application/json")
            .body(payload.to_string())
            .send()
            .await;
            
        match response {
            Ok(_) => Ok(NotificationResult {
                success: true,
                message_id: format!("MSG-{}", 0),
                timestamp: 0,
                error: "".to_string(),
            }),
            Err(e) => Ok(NotificationResult {
                success: false,
                message_id: "".to_string(),
                timestamp: 0,
                error: format!("{:?}", e),
            }),
        }
    }
}

// ===== CONTRACT IMPLEMENTATION =====

#[smart_contract]
impl SlackNotifier for SlackNotifierContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(SlackNotifierContractState {
            secrets: Secrets::new(),
        })
    }

    /// Send a simple text message to Slack
    #[query]
    async fn send_message(&self, channel: String, message: String) -> Result<NotificationResult, String> {
        let text = format!("📢 *{}*\n{}", channel, message);
        self.send_to_slack(text).await
    }

    /// Send a formatted alert notification
    #[query]
    async fn send_alert(&self, alert_type: String, severity: String, symbol: String, entity_id: String, description: String, risk_score: u32) -> Result<NotificationResult, String> {
        let emoji = self.get_severity_emoji(&severity);
        let text = format!(
            "{} *{} Alert - {}*\n\n*Symbol:* {}\n*Entity:* {}\n*Risk Score:* {}/100\n*Description:* {}",
            emoji, severity, alert_type, symbol, entity_id, risk_score, description
        );
        self.send_to_slack(text).await
    }

    /// Send case update notification
    #[query]
    async fn send_case_update(&self, case_id: String, status: String, update_message: String, assigned_to: String) -> Result<NotificationResult, String> {
        let status_emoji = match status.as_str() {
            "OPEN" => "📂",
            "INVESTIGATING" => "🔍",
            "ESCALATED" => "⚠️",
            "CLOSED" => "✅",
            _ => "📋",
        };
        
        let text = format!(
            "{} *Case Update: {}*\n\n*Status:* {}\n*Assigned To:* {}\n*Update:* {}",
            status_emoji, case_id, status, assigned_to, update_message
        );
        self.send_to_slack(text).await
    }

    /// Send workflow completion notification
    #[query]
    async fn send_workflow_complete(&self, workflow_id: String, workflow_type: String, result_summary: String, alert_count: u32) -> Result<NotificationResult, String> {
        let alert_indicator = if alert_count > 0 { "🚨" } else { "✅" };
        
        let text = format!(
            "{} *Workflow Complete: {}*\n\n*Type:* {}\n*Alerts Generated:* {}\n*Summary:* {}",
            alert_indicator, workflow_id, workflow_type, alert_count, result_summary
        );
        self.send_to_slack(text).await
    }

    /// Send daily summary report
    #[query]
    async fn send_daily_summary(&self, date: String, total_alerts: u32, critical_alerts: u32, open_cases: u32, new_cases: u32) -> Result<NotificationResult, String> {
        let text = format!(
            "📊 *Daily Surveillance Summary - {}*\n\n• Total Alerts: {}\n• Critical Alerts: {}\n• Open Cases: {}\n• New Cases Today: {}",
            date, total_alerts, critical_alerts, open_cases, new_cases
        );
        self.send_to_slack(text).await
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {"type": "function", "function": {"name": "send_message", "description": "Send a simple text message to Slack", "parameters": {"type": "object", "properties": {"channel": {"type": "string"}, "message": {"type": "string"}}, "required": ["channel", "message"]}}},
  {"type": "function", "function": {"name": "send_alert", "description": "Send formatted alert notification to Slack", "parameters": {"type": "object", "properties": {"alert_type": {"type": "string"}, "severity": {"type": "string"}, "symbol": {"type": "string"}, "entity_id": {"type": "string"}, "description": {"type": "string"}, "risk_score": {"type": "integer"}}, "required": ["alert_type", "severity", "symbol", "entity_id", "description", "risk_score"]}}},
  {"type": "function", "function": {"name": "send_case_update", "description": "Send case update notification", "parameters": {"type": "object", "properties": {"case_id": {"type": "string"}, "status": {"type": "string"}, "update_message": {"type": "string"}, "assigned_to": {"type": "string"}}, "required": ["case_id", "status", "update_message", "assigned_to"]}}},
  {"type": "function", "function": {"name": "send_workflow_complete", "description": "Send workflow completion notification", "parameters": {"type": "object", "properties": {"workflow_id": {"type": "string"}, "workflow_type": {"type": "string"}, "result_summary": {"type": "string"}, "alert_count": {"type": "integer"}}, "required": ["workflow_id", "workflow_type", "result_summary", "alert_count"]}}},
  {"type": "function", "function": {"name": "send_daily_summary", "description": "Send daily summary report to Slack", "parameters": {"type": "object", "properties": {"date": {"type": "string"}, "total_alerts": {"type": "integer"}, "critical_alerts": {"type": "integer"}, "open_cases": {"type": "integer"}, "new_cases": {"type": "integer"}}, "required": ["date", "total_alerts", "critical_alerts", "open_cases", "new_cases"]}}}
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{ "prompts": [] }"#.to_string()
    }
}
