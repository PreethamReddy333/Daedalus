
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use weil_macros::{constructor, query, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::http::{HttpClient, HttpMethod};

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
            "text": text
        });
        
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        let response = HttpClient::request(&config.webhook_url, HttpMethod::Post)
            .headers(headers)
            .body(payload.to_string())
            .send();
            
        match response {
            Ok(resp) => {
                let status = resp.status();
                let text = resp.text();
                
                if status == 200 {
                    Ok(NotificationResult {
                        success: true,
                        message_id: format!("MSG-{}", 0),
                        timestamp: 0,
                        error: "".to_string(),
                    })
                } else {
                    Ok(NotificationResult {
                        success: false,
                        message_id: "".to_string(),
                        timestamp: 0,
                        error: format!("Slack returned HTTP {}: {}", status, text),
                    })
                }
            },
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

    #[query]
    async fn send_message(&self, channel: String, message: String) -> Result<NotificationResult, String> {
        let text = format!("📢 *{}*\n{}", channel, message);
        self.send_to_slack(text).await
    }

    #[query]
    async fn send_alert(&self, alert_type: String, severity: String, symbol: String, entity_id: String, description: String, risk_score: u32) -> Result<NotificationResult, String> {
        let emoji = self.get_severity_emoji(&severity);
        let text = format!(
            "{} *{} Alert - {}*\n\n*Symbol:* {}\n*Entity:* {}\n*Risk Score:* {}/100\n*Description:* {}",
            emoji, severity, alert_type, symbol, entity_id, risk_score, description
        );
        self.send_to_slack(text).await
    }

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

    #[query]
    async fn send_workflow_complete(&self, workflow_id: String, workflow_type: String, result_summary: String, alert_count: u32) -> Result<NotificationResult, String> {
        let alert_indicator = if alert_count > 0 { "🚨" } else { "✅" };
        
        let text = format!(
            "{} *Workflow Complete: {}*\n\n*Type:* {}\n*Alerts Generated:* {}\n*Summary:* {}",
            alert_indicator, workflow_id, workflow_type, alert_count, result_summary
        );
        self.send_to_slack(text).await
    }

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
  {
    "type": "function",
    "function": {
      "name": "send_message",
      "description": "Send a simple text message to a Slack channel\n",
      "parameters": {
        "type": "object",
        "properties": {
          "channel": {
            "type": "string",
            "description": "Target Slack channel (e.g., #alerts)\n"
          },
          "message": {
            "type": "string",
            "description": "Message text to send\n"
          }
        },
        "required": [
          "channel",
          "message"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "send_alert",
      "description": "Send a formatted surveillance alert notification to Slack\n",
      "parameters": {
        "type": "object",
        "properties": {
          "alert_type": {
            "type": "string",
            "description": "Type of alert: INSIDER, SPOOFING, WASH_TRADE, PUMP_DUMP\n"
          },
          "severity": {
            "type": "string",
            "description": "Severity level: CRITICAL, HIGH, MEDIUM, LOW\n"
          },
          "symbol": {
            "type": "string",
            "description": "Stock/security symbol (e.g., RELIANCE)\n"
          },
          "entity_id": {
            "type": "string",
            "description": "Entity ID involved in the alert\n"
          },
          "description": {
            "type": "string",
            "description": "Alert description text\n"
          },
          "risk_score": {
            "type": "integer",
            "description": "Risk score from 0-100\n"
          }
        },
        "required": [
          "alert_type",
          "severity",
          "symbol",
          "entity_id",
          "description",
          "risk_score"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "send_case_update",
      "description": "Send a case status update notification to Slack\n",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": {
            "type": "string",
            "description": "Case ID from case management system\n"
          },
          "status": {
            "type": "string",
            "description": "Case status: OPEN, INVESTIGATING, ESCALATED, CLOSED\n"
          },
          "update_message": {
            "type": "string",
            "description": "Update message describing the change\n"
          },
          "assigned_to": {
            "type": "string",
            "description": "Name of assigned investigator\n"
          }
        },
        "required": [
          "case_id",
          "status",
          "update_message",
          "assigned_to"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "send_daily_summary",
      "description": "Send daily surveillance summary report to Slack\n",
      "parameters": {
        "type": "object",
        "properties": {
          "date": {
            "type": "string",
            "description": "Report date (e.g., 2026-01-12)\n"
          },
          "total_alerts": {
            "type": "integer",
            "description": "Total number of alerts for the day\n"
          },
          "critical_alerts": {
            "type": "integer",
            "description": "Number of critical severity alerts\n"
          },
          "open_cases": {
            "type": "integer",
            "description": "Total open investigation cases\n"
          },
          "new_cases": {
            "type": "integer",
            "description": "New cases opened today\n"
          }
        },
        "required": [
          "date",
          "total_alerts",
          "critical_alerts",
          "open_cases",
          "new_cases"
        ]
      }
    }
  }
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{
  "prompts": []
}"#.to_string()
    }
}
