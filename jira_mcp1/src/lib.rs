//! # Jira MCP Server
//!
//! Creates and manages Jira tickets for surveillance cases.
//! Enables team coordination and audit trail for investigations.
//!
//! ## Auth
//! Uses HTTP Basic auth with email:api_token (Base64-encoded).
//!
//! ## API
//! Jira Cloud REST API v3: https://developer.atlassian.com/cloud/jira/platform/rest/v3/

use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use weil_macros::{constructor, mutate, query, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::http::{HttpClient, HttpMethod};

// ===== CONFIGURATION =====

#[derive(Debug, Serialize, Deserialize, WeilType, Default, Clone)]
pub struct JiraConfig {
    pub jira_url: String,
    pub jira_email: String,
    pub jira_api_token: String,
    pub project_key: String,
    pub default_issue_type: String,
}

// ===== DATA STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct JiraTicket {
    pub ticket_id: String,
    pub key: String,
    pub summary: String,
    pub description: String,
    pub status: String,
    pub issue_type: String,
    pub priority: String,
    pub assignee: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct TicketResult {
    pub success: bool,
    pub ticket_key: String,
    pub ticket_url: String,
    pub error: String,
}

// Jira API response structures
#[derive(Debug, Deserialize)]
struct JiraIssueResponse {
    id: String,
    key: String,
    #[serde(rename = "self")]
    self_url: String,
}

#[derive(Debug, Deserialize)]
struct JiraIssueDetail {
    id: String,
    key: String,
    fields: JiraIssueFields,
}

#[derive(Debug, Deserialize)]
struct JiraIssueFields {
    summary: Option<String>,
    description: Option<serde_json::Value>,
    status: Option<JiraStatus>,
    issuetype: Option<JiraIssueType>,
    priority: Option<JiraPriority>,
    assignee: Option<JiraUser>,
    created: Option<String>,
    updated: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JiraStatus {
    name: String,
}

#[derive(Debug, Deserialize)]
struct JiraIssueType {
    name: String,
}

#[derive(Debug, Deserialize)]
struct JiraPriority {
    name: String,
}

#[derive(Debug, Deserialize)]
struct JiraUser {
    #[serde(rename = "displayName")]
    display_name: Option<String>,
}

// ===== TRAIT DEFINITION =====

trait JiraIntegration {
    fn new() -> Result<Self, String> where Self: Sized;
    async fn create_ticket(&self, summary: String, description: Option<String>, priority: Option<String>, issue_type: Option<String>) -> Result<TicketResult, String>;
    async fn create_case_ticket(&self, case_id: String, subject_entity: String, case_summary: String, priority: Option<String>) -> Result<TicketResult, String>;
    async fn close_ticket(&self, ticket_key: String, resolution: Option<String>) -> Result<TicketResult, String>;
    async fn get_ticket(&self, ticket_key: String) -> Result<JiraTicket, String>;
    async fn add_comment(&self, ticket_key: String, comment: String) -> Result<TicketResult, String>;
    async fn update_ticket_status(&self, ticket_key: String, new_status: String) -> Result<TicketResult, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

// ===== CONTRACT STATE =====

#[derive(Serialize, Deserialize, WeilType)]
pub struct JiraIntegrationContractState {
    secrets: Secrets<JiraConfig>,
}

// ===== HELPER METHODS =====

impl JiraIntegrationContractState {
    /// Get headers for Jira API requests (following Confluence pattern)
    fn get_headers(&self) -> HashMap<String, String> {
        let config = self.secrets.config();
        let credentials = format!("{}:{}", config.jira_email, config.jira_api_token);
        let encoded = general_purpose::STANDARD.encode(credentials.as_bytes());
        
        HashMap::from([
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Authorization".to_string(), format!("Basic {}", encoded)),
        ])
    }
    
    /// Make an authenticated HTTP request to Jira REST API
    async fn make_request(
        &self,
        method: HttpMethod,
        endpoint: &str,
        query_params: Vec<(String, String)>,
        body: Option<String>,
        expected_status_code: u16,
    ) -> Result<(u16, String), String> {
        let url = format!(
            "{}/rest/api/3/{}",
            self.secrets.config().jira_url,
            endpoint
        );

        let headers = self.get_headers();

        let mut request = HttpClient::request(&url, method)
            .headers(headers)
            .query(query_params);
        
        if let Some(body_str) = body {
            request = request.body(body_str);
        }

        let response = request.send().map_err(|err| err.to_string())?;
        let status = response.status();
        let text = response.text();

        if status != expected_status_code && !(200..300).contains(&status) {
            return Err(format!("HTTP {}: {}", status, text));
        }

        Ok((status, text))
    }
}

// ===== CONTRACT IMPLEMENTATION =====

#[smart_contract]
impl JiraIntegration for JiraIntegrationContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(JiraIntegrationContractState {
            secrets: Secrets::new(),
        })
    }

    /// Create a new Jira ticket via POST /rest/api/3/issue
    #[query]
    async fn create_ticket(
        &self, 
        summary: String, 
        description: Option<String>,
        priority: Option<String>,
        issue_type: Option<String>
    ) -> Result<TicketResult, String> {
        let config = self.secrets.config();
        let desc = description.unwrap_or_else(|| "Created via Surveillance MCP".to_string());
        let prio = priority.unwrap_or_else(|| "Medium".to_string());
        let itype = issue_type.unwrap_or_else(|| config.default_issue_type.clone());
        
        // Jira API v3 uses Atlassian Document Format for description
        let payload = serde_json::json!({
            "fields": {
                "project": { "key": config.project_key },
                "summary": summary,
                "description": {
                    "type": "doc",
                    "version": 1,
                    "content": [{
                        "type": "paragraph",
                        "content": [{ "type": "text", "text": desc }]
                    }]
                },
                "issuetype": { "name": itype },
                "priority": { "name": prio }
            }
        });
        
        let body = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
        
        let result = self.make_request(
            HttpMethod::Post,
            "issue",
            vec![],
            Some(body),
            201
        ).await;
            
        match result {
            Ok((_, response_text)) => {
                match serde_json::from_str::<JiraIssueResponse>(&response_text) {
                    Ok(issue) => Ok(TicketResult {
                        success: true,
                        ticket_key: issue.key.clone(),
                        ticket_url: format!("{}/browse/{}", config.jira_url, issue.key),
                        error: "".to_string(),
                    }),
                    Err(e) => Ok(TicketResult {
                        success: false,
                        ticket_key: "".to_string(),
                        ticket_url: "".to_string(),
                        error: format!("Parse error: {}. Response: {}", e, response_text),
                    }),
                }
            },
            Err(e) => Ok(TicketResult {
                success: false,
                ticket_key: "".to_string(),
                ticket_url: "".to_string(),
                error: e,
            }),
        }
    }

    /// Create a ticket specifically for a surveillance case
    #[query]
    async fn create_case_ticket(
        &self, 
        case_id: String, 
        subject_entity: String, 
        case_summary: String,
        priority: Option<String>
    ) -> Result<TicketResult, String> {
        let summary = format!("[CASE {}] Investigation: {}", case_id, subject_entity);
        let description = format!(
            "Surveillance Case Investigation\n\n- Case ID: {}\n- Subject Entity: {}\n- Summary: {}\n\nThis ticket was auto-created from the Market Surveillance System.",
            case_id, subject_entity, case_summary
        );
        
        self.create_ticket(summary, Some(description), priority, Some("Task".to_string())).await
    }

    /// Close a Jira ticket by transitioning to Done
    #[query]
    async fn close_ticket(&self, ticket_key: String, resolution: Option<String>) -> Result<TicketResult, String> {
        let config = self.secrets.config();
        let _res = resolution.unwrap_or_else(|| "Done".to_string());
        
        // Transition to Done (transition ID 31 is common, but may vary)
        let payload = serde_json::json!({
            "transition": { "id": "31" }
        });
        
        let body = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
        
        let result = self.make_request(
            HttpMethod::Post,
            &format!("issue/{}/transitions", ticket_key),
            vec![],
            Some(body),
            204
        ).await;
            
        match result {
            Ok(_) => Ok(TicketResult {
                success: true,
                ticket_key: ticket_key.clone(),
                ticket_url: format!("{}/browse/{}", config.jira_url, ticket_key),
                error: "".to_string(),
            }),
            Err(e) => Ok(TicketResult {
                success: false,
                ticket_key,
                ticket_url: "".to_string(),
                error: e,
            }),
        }
    }

    /// Get ticket details via GET /rest/api/3/issue/{key}
    #[query]
    async fn get_ticket(&self, ticket_key: String) -> Result<JiraTicket, String> {
        let config = self.secrets.config();
        
        let result = self.make_request(
            HttpMethod::Get,
            &format!("issue/{}", ticket_key),
            vec![],
            None,
            200
        ).await?;
        
        let response_text = result.1;
        
        // Parse Jira response
        match serde_json::from_str::<JiraIssueDetail>(&response_text) {
            Ok(issue) => Ok(JiraTicket {
                ticket_id: issue.id,
                key: issue.key.clone(),
                summary: issue.fields.summary.unwrap_or_default(),
                description: "".to_string(), // ADF is complex to parse
                status: issue.fields.status.map(|s| s.name).unwrap_or_default(),
                issue_type: issue.fields.issuetype.map(|t| t.name).unwrap_or_default(),
                priority: issue.fields.priority.map(|p| p.name).unwrap_or_default(),
                assignee: issue.fields.assignee.and_then(|a| a.display_name).unwrap_or_else(|| "Unassigned".to_string()),
                created_at: 0,
                updated_at: 0,
                url: format!("{}/browse/{}", config.jira_url, issue.key),
            }),
            Err(e) => Err(format!("Failed to parse response: {}. Response: {}", e, response_text)),
        }
    }

    /// Add comment to ticket via POST /rest/api/3/issue/{key}/comment
    #[query]
    async fn add_comment(&self, ticket_key: String, comment: String) -> Result<TicketResult, String> {
        let config = self.secrets.config();
        
        // Jira v3 uses Atlassian Document Format
        let payload = serde_json::json!({
            "body": {
                "type": "doc",
                "version": 1,
                "content": [{
                    "type": "paragraph",
                    "content": [{ "type": "text", "text": comment }]
                }]
            }
        });
        
        let body = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
        
        let result = self.make_request(
            HttpMethod::Post,
            &format!("issue/{}/comment", ticket_key),
            vec![],
            Some(body),
            201
        ).await;
            
        match result {
            Ok(_) => Ok(TicketResult {
                success: true,
                ticket_key: ticket_key.clone(),
                ticket_url: format!("{}/browse/{}", config.jira_url, ticket_key),
                error: "".to_string(),
            }),
            Err(e) => Ok(TicketResult {
                success: false,
                ticket_key,
                ticket_url: "".to_string(),
                error: e,
            }),
        }
    }

    /// Update ticket status via transitions
    #[query]
    async fn update_ticket_status(&self, ticket_key: String, new_status: String) -> Result<TicketResult, String> {
        let config = self.secrets.config();
        
        // Map common status names to transition IDs (these vary by workflow)
        let transition_id = match new_status.as_str() {
            "In Progress" => "21",
            "Done" => "31",
            "To Do" => "11",
            _ => "21", // Default to In Progress
        };
        
        let payload = serde_json::json!({
            "transition": { "id": transition_id }
        });
        
        let body = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
        
        let result = self.make_request(
            HttpMethod::Post,
            &format!("issue/{}/transitions", ticket_key),
            vec![],
            Some(body),
            204
        ).await;
            
        match result {
            Ok(_) => Ok(TicketResult {
                success: true,
                ticket_key: ticket_key.clone(),
                ticket_url: format!("{}/browse/{}", config.jira_url, ticket_key),
                error: "".to_string(),
            }),
            Err(e) => Ok(TicketResult {
                success: false,
                ticket_key,
                ticket_url: "".to_string(),
                error: e,
            }),
        }
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "create_ticket",
      "description": "Create a new Jira ticket. Defaults: priority=Medium, type=Task\n",
      "parameters": {
        "type": "object",
        "properties": {
          "summary": {
            "type": "string",
            "description": "Ticket title/summary (required)\n"
          },
          "description": {
            "type": "string",
            "description": "Optional ticket description\n"
          },
          "priority": {
            "type": "string",
            "description": "Optional priority level: High, Medium, Low\n"
          },
          "issue_type": {
            "type": "string",
            "description": "Optional issue type: Task, Bug, Story\n"
          }
        },
        "required": [
          "summary"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "create_case_ticket",
      "description": "Create a Jira ticket for a surveillance case investigation\n",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": {
            "type": "string",
            "description": "Case ID from case management system\n"
          },
          "subject_entity": {
            "type": "string",
            "description": "Entity under investigation\n"
          },
          "case_summary": {
            "type": "string",
            "description": "Brief summary of the case\n"
          },
          "priority": {
            "type": "string",
            "description": "Optional priority\n"
          }
        },
        "required": [
          "case_id",
          "subject_entity",
          "case_summary"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "close_ticket",
      "description": "Close a Jira ticket with resolution\n",
      "parameters": {
        "type": "object",
        "properties": {
          "ticket_key": {
            "type": "string",
            "description": "Jira ticket key (e.g., WEIL-123)\n"
          },
          "resolution": {
            "type": "string",
            "description": "Optional resolution note (default: Done)\n"
          }
        },
        "required": [
          "ticket_key"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_ticket",
      "description": "Get ticket details by key\n",
      "parameters": {
        "type": "object",
        "properties": {
          "ticket_key": {
            "type": "string",
            "description": "Jira ticket key (e.g., WEIL-123)\n"
          }
        },
        "required": [
          "ticket_key"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "add_comment",
      "description": "Add a comment to a ticket\n",
      "parameters": {
        "type": "object",
        "properties": {
          "ticket_key": {
            "type": "string",
            "description": "Jira ticket key\n"
          },
          "comment": {
            "type": "string",
            "description": "Comment text\n"
          }
        },
        "required": [
          "ticket_key",
          "comment"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "update_ticket_status",
      "description": "Update the status of a Jira ticket\n",
      "parameters": {
        "type": "object",
        "properties": {
          "ticket_key": {
            "type": "string",
            "description": "Jira ticket key\n"
          },
          "new_status": {
            "type": "string",
            "description": "New status: To Do, In Progress, Done\n"
          }
        },
        "required": [
          "ticket_key",
          "new_status"
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
