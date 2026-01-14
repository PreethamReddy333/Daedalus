
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


#[derive(Debug, Serialize, Deserialize, WeilType, Default)]
pub struct JiraConfig {
    pub jira_url: String,
    pub jira_email: String,
    pub jira_api_token: String,
    pub project_key: String,
    pub default_issue_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketResult {
    pub success: bool,
    pub ticket_key: String,
    pub ticket_url: String,
    pub error: String,
}

trait JiraIntegration {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn create_ticket(&mut self, summary: String, description: Option<String>, priority: Option<String>, issue_type: Option<String>) -> Result<TicketResult, String>;
    async fn create_case_ticket(&mut self, case_id: String, case_type: String, subject_entity: String, symbol: String, risk_score: u32, case_summary: String, priority: String, assignee: Option<String>) -> Result<TicketResult, String>;
    async fn update_case_status(&mut self, ticket_key: String, new_status: String, status_note: String) -> Result<TicketResult, String>;
    async fn assign_case(&mut self, ticket_key: String, assigned_to: String, assignment_note: Option<String>) -> Result<TicketResult, String>;
    async fn add_evidence(&mut self, ticket_key: String, evidence_type: String, description: String, source: String) -> Result<TicketResult, String>;
    async fn add_note(&mut self, ticket_key: String, author: String, content: String) -> Result<TicketResult, String>;
    async fn add_comment(&mut self, ticket_key: String, comment: String) -> Result<TicketResult, String>;
    async fn get_ticket(&self, ticket_key: String) -> Result<JiraTicket, String>;
    async fn get_case_by_id(&self, case_id: String) -> Result<JiraTicket, String>;
    async fn list_open_cases(&self, priority_filter: String, limit: u32) -> Result<Vec<JiraTicket>, String>;
    async fn get_entity_cases(&self, entity_id: String, limit: u32) -> Result<Vec<JiraTicket>, String>;
    async fn get_case_timeline(&self, ticket_key: String) -> Result<String, String>;
    async fn close_case(&mut self, ticket_key: String, resolution: String, closure_summary: String) -> Result<TicketResult, String>;
    async fn get_case_stats(&self) -> Result<String, String>;
    async fn search_cases_jql(&self, jql: String, limit: u32) -> Result<Vec<JiraTicket>, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct JiraIntegrationContractState {
    // define your contract state here!
    secrets: Secrets<JiraConfig>,
}

#[smart_contract]
impl JiraIntegration for JiraIntegrationContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!();
    }


    #[mutate]
    async fn create_ticket(&mut self, summary: String, description: Option<String>, priority: Option<String>, issue_type: Option<String>) -> Result<TicketResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn create_case_ticket(&mut self, case_id: String, case_type: String, subject_entity: String, symbol: String, risk_score: u32, case_summary: String, priority: String, assignee: Option<String>) -> Result<TicketResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn update_case_status(&mut self, ticket_key: String, new_status: String, status_note: String) -> Result<TicketResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn assign_case(&mut self, ticket_key: String, assigned_to: String, assignment_note: Option<String>) -> Result<TicketResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn add_evidence(&mut self, ticket_key: String, evidence_type: String, description: String, source: String) -> Result<TicketResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn add_note(&mut self, ticket_key: String, author: String, content: String) -> Result<TicketResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn add_comment(&mut self, ticket_key: String, comment: String) -> Result<TicketResult, String> {
        unimplemented!();
    }

    #[query]
    async fn get_ticket(&self, ticket_key: String) -> Result<JiraTicket, String> {
        unimplemented!();
    }

    #[query]
    async fn get_case_by_id(&self, case_id: String) -> Result<JiraTicket, String> {
        unimplemented!();
    }

    #[query]
    async fn list_open_cases(&self, priority_filter: String, limit: u32) -> Result<Vec<JiraTicket>, String> {
        unimplemented!();
    }

    #[query]
    async fn get_entity_cases(&self, entity_id: String, limit: u32) -> Result<Vec<JiraTicket>, String> {
        unimplemented!();
    }

    #[query]
    async fn get_case_timeline(&self, ticket_key: String) -> Result<String, String> {
        unimplemented!();
    }

    #[mutate]
    async fn close_case(&mut self, ticket_key: String, resolution: String, closure_summary: String) -> Result<TicketResult, String> {
        unimplemented!();
    }

    #[query]
    async fn get_case_stats(&self) -> Result<String, String> {
        unimplemented!();
    }

    #[query]
    async fn search_cases_jql(&self, jql: String, limit: u32) -> Result<Vec<JiraTicket>, String> {
        unimplemented!();
    }


    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "create_ticket",
      "description": "===== TICKET CREATION =====\nCreate a new Jira ticket\n",
      "parameters": {
        "type": "object",
        "properties": {
          "summary": {
            "type": "string",
            "description": "Ticket title/summary\n"
          },
          "description": {
            "type": "string",
            "description": "Optional description\n"
          },
          "priority": {
            "type": "string",
            "description": "Optional priority: High, Medium, Low\n"
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
      "description": "Create a ticket specifically for a surveillance case\n",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": {
            "type": "string",
            "description": "Case ID\n"
          },
          "case_type": {
            "type": "string",
            "description": "Case type: INSIDER_TRADING, SPOOFING, WASH_TRADING, PUMP_DUMP\n"
          },
          "subject_entity": {
            "type": "string",
            "description": "Subject entity under investigation\n"
          },
          "symbol": {
            "type": "string",
            "description": "Stock symbol involved\n"
          },
          "risk_score": {
            "type": "integer",
            "description": "Risk score (0-100)\n"
          },
          "case_summary": {
            "type": "string",
            "description": "Case summary description\n"
          },
          "priority": {
            "type": "string",
            "description": "Priority: CRITICAL, HIGH, MEDIUM, LOW\n"
          },
          "assignee": {
            "type": "string",
            "description": "Optional assignee\n"
          }
        },
        "required": [
          "case_id",
          "case_type",
          "subject_entity",
          "symbol",
          "risk_score",
          "case_summary",
          "priority"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "update_case_status",
      "description": "===== CASE LIFECYCLE MANAGEMENT =====\nUpdate case status (replaces Case Management MCP)\nAllowed transitions: Open -> In Progress -> Escalated -> Closed\n",
      "parameters": {
        "type": "object",
        "properties": {
          "ticket_key": {
            "type": "string",
            "description": "Jira ticket key\n"
          },
          "new_status": {
            "type": "string",
            "description": "New status: Open, In Progress, Escalated, Closed\n"
          },
          "status_note": {
            "type": "string",
            "description": "Note explaining the status change\n"
          }
        },
        "required": [
          "ticket_key",
          "new_status",
          "status_note"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "assign_case",
      "description": "Assign case to an investigator\n",
      "parameters": {
        "type": "object",
        "properties": {
          "ticket_key": {
            "type": "string",
            "description": "Jira ticket key\n"
          },
          "assigned_to": {
            "type": "string",
            "description": "Investigator email or username\n"
          },
          "assignment_note": {
            "type": "string",
            "description": "Optional note about assignment\n"
          }
        },
        "required": [
          "ticket_key",
          "assigned_to"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "add_evidence",
      "description": "===== EVIDENCE & NOTES =====\nAdd evidence to a case as a comment with special label\n",
      "parameters": {
        "type": "object",
        "properties": {
          "ticket_key": {
            "type": "string",
            "description": "Jira ticket key\n"
          },
          "evidence_type": {
            "type": "string",
            "description": "Evidence type: TRADE, COMMUNICATION, DOCUMENT, ANALYSIS\n"
          },
          "description": {
            "type": "string",
            "description": "Description of the evidence\n"
          },
          "source": {
            "type": "string",
            "description": "Source of the evidence (e.g., Trade ID, email, document link)\n"
          }
        },
        "required": [
          "ticket_key",
          "evidence_type",
          "description",
          "source"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "add_note",
      "description": "Add an investigation note to a case\n",
      "parameters": {
        "type": "object",
        "properties": {
          "ticket_key": {
            "type": "string",
            "description": "Jira ticket key\n"
          },
          "author": {
            "type": "string",
            "description": "Author of the note\n"
          },
          "content": {
            "type": "string",
            "description": "Note content\n"
          }
        },
        "required": [
          "ticket_key",
          "author",
          "content"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "add_comment",
      "description": "Add a comment to a ticket (generic)\n",
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
      "name": "get_ticket",
      "description": "===== CASE QUERIES =====\nGet ticket details by key\n",
      "parameters": {
        "type": "object",
        "properties": {
          "ticket_key": {
            "type": "string",
            "description": "Jira ticket key\n"
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
      "name": "get_case_by_id",
      "description": "Get case by case ID (custom field search)\n",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": {
            "type": "string",
            "description": "Case ID\n"
          }
        },
        "required": [
          "case_id"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "list_open_cases",
      "description": "List open cases by priority\n",
      "parameters": {
        "type": "object",
        "properties": {
          "priority_filter": {
            "type": "string",
            "description": "Priority filter: ALL, CRITICAL, HIGH, MEDIUM, LOW\n"
          },
          "limit": {
            "type": "integer",
            "description": "Maximum number of cases to return\n"
          }
        },
        "required": [
          "priority_filter",
          "limit"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_entity_cases",
      "description": "Get cases for a specific entity (subject_entity custom field)\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity ID to search for\n"
          },
          "limit": {
            "type": "integer",
            "description": "Maximum results\n"
          }
        },
        "required": [
          "entity_id",
          "limit"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_case_timeline",
      "description": "Get all comments/timeline for a case\n",
      "parameters": {
        "type": "object",
        "properties": {
          "ticket_key": {
            "type": "string",
            "description": "Jira ticket key\n"
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
      "name": "close_case",
      "description": "===== CASE CLOSURE =====\nClose a case with resolution\n",
      "parameters": {
        "type": "object",
        "properties": {
          "ticket_key": {
            "type": "string",
            "description": "Jira ticket key (e.g., SURV-123)\n"
          },
          "resolution": {
            "type": "string",
            "description": "Resolution: Done, Won't Fix, Duplicate, False Positive\n"
          },
          "closure_summary": {
            "type": "string",
            "description": "Closure summary\n"
          }
        },
        "required": [
          "ticket_key",
          "resolution",
          "closure_summary"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_case_stats",
      "description": "===== BULK OPERATIONS =====\nGet case statistics (open, closed, by priority)\n",
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
      "name": "search_cases_jql",
      "description": "Search cases by JQL (advanced)\n",
      "parameters": {
        "type": "object",
        "properties": {
          "jql": {
            "type": "string",
            "description": "Jira Query Language string\n"
          },
          "limit": {
            "type": "integer",
            "description": "Maximum results\n"
          }
        },
        "required": [
          "jql",
          "limit"
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

