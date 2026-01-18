
use anyhow::Result;
use serde::{Deserialize, Serialize};
use weil_rs::runtime::Runtime;

pub struct JiraMcp {
    contract_id: String,
}

impl JiraMcp {
    pub fn new(contract_id: String) -> Self {
        JiraMcp { contract_id }
    }
}

// ===== Response Types =====

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TicketResult {
    pub success: bool,
    pub ticket_key: String,
    pub ticket_url: String,
    pub error: String,
}

impl JiraMcp {
    pub fn get_ticket(&self, ticket_key: String) -> Result<JiraTicket> {
        #[derive(Debug, Serialize)]
        struct GetTicketArgs {
            ticket_key: String,
        }

        let serialized_args = Some(serde_json::to_string(&GetTicketArgs { ticket_key })?);

        let resp = Runtime::call_contract::<JiraTicket>(
            self.contract_id.clone(),
            "get_ticket".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn create_case_ticket(
        &self,
        case_id: String,
        subject_entity: String,
        case_summary: String,
        priority: Option<String>,
    ) -> Result<TicketResult> {
        #[derive(Debug, Serialize)]
        struct CreateCaseTicketArgs {
            case_id: String,
            subject_entity: String,
            case_summary: String,
            priority: Option<String>,
        }

        let serialized_args = Some(serde_json::to_string(&CreateCaseTicketArgs {
            case_id,
            subject_entity,
            case_summary,
            priority,
        })?);

        let resp = Runtime::call_contract::<TicketResult>(
            self.contract_id.clone(),
            "create_case_ticket".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn add_comment(&self, ticket_key: String, comment: String) -> Result<TicketResult> {
        #[derive(Debug, Serialize)]
        struct AddCommentArgs {
            ticket_key: String,
            comment: String,
        }

        let serialized_args = Some(serde_json::to_string(&AddCommentArgs {
            ticket_key,
            comment,
        })?);

        let resp = Runtime::call_contract::<TicketResult>(
            self.contract_id.clone(),
            "add_comment".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }
}
