
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


#[derive(Debug, Serialize, Deserialize, WeilType, Default)]
pub struct RegulatoryReportsConfig {
    pub dashboard_contract_id: String,
    pub jira_contract_id: String,
    pub risk_scoring_contract_id: String,
    pub anomaly_detection_contract_id: String,
    pub entity_relationship_contract_id: String,
    pub supabase_url: String,
    pub supabase_api_key: String,
    pub supabase_storage_bucket: String,
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

trait RegulatoryReports {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
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

#[derive(Serialize, Deserialize, WeilType)]
pub struct RegulatoryReportsContractState {
    // define your contract state here!
    secrets: Secrets<RegulatoryReportsConfig>,
}

#[smart_contract]
impl RegulatoryReports for RegulatoryReportsContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!();
    }


    #[mutate]
    async fn get_context(&mut self) -> QueryContext {
        unimplemented!();
    }

    #[mutate]
    async fn generate_str(&mut self, case_id: String, entity_id: String, suspicious_activity_type: String, suspicion_reason: String) -> Result<ReportResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn generate_surveillance_report(&mut self, from_date: String, to_date: String, report_type: String) -> Result<ReportResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn generate_compliance_scorecard(&mut self, entity_id: String, period: String) -> Result<ReportResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn generate_entity_risk_report(&mut self, entity_id: String) -> Result<ReportResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn generate_gsm_report(&mut self, report_date: String) -> Result<ReportResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn generate_esm_report(&mut self, report_date: String) -> Result<ReportResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_pending_strs(&mut self, limit: u32) -> Result<Vec<STRReport>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn submit_str(&mut self, str_id: String) -> Result<ReportResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn generate_investigation_report(&mut self, case_id: String, include_evidence: bool) -> Result<ReportResult, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_report_url(&mut self, report_id: String) -> Result<ReportResult, String> {
        unimplemented!();
    }


    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_context",
      "description": "CALL THIS FIRST - Get context from recent queries\n",
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
      "name": "generate_str",
      "description": "Generate Suspicious Transaction Report and upload to Supabase Storage\n",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": {
            "type": "string",
            "description": ""
          },
          "entity_id": {
            "type": "string",
            "description": ""
          },
          "suspicious_activity_type": {
            "type": "string",
            "description": ""
          },
          "suspicion_reason": {
            "type": "string",
            "description": ""
          }
        },
        "required": [
          "case_id",
          "entity_id",
          "suspicious_activity_type",
          "suspicion_reason"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "generate_surveillance_report",
      "description": "Generate periodic market surveillance report\n",
      "parameters": {
        "type": "object",
        "properties": {
          "from_date": {
            "type": "string",
            "description": ""
          },
          "to_date": {
            "type": "string",
            "description": ""
          },
          "report_type": {
            "type": "string",
            "description": ""
          }
        },
        "required": [
          "from_date",
          "to_date",
          "report_type"
        ]
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
          "entity_id": {
            "type": "string",
            "description": ""
          },
          "period": {
            "type": "string",
            "description": ""
          }
        },
        "required": [
          "entity_id",
          "period"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "generate_entity_risk_report",
      "description": "Generate full entity risk report\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": ""
          }
        },
        "required": [
          "entity_id"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "generate_gsm_report",
      "description": "Generate GSM (Graded Surveillance Measure) report\n",
      "parameters": {
        "type": "object",
        "properties": {
          "report_date": {
            "type": "string",
            "description": ""
          }
        },
        "required": [
          "report_date"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "generate_esm_report",
      "description": "Generate ESM (Enhanced Surveillance Measure) report\n",
      "parameters": {
        "type": "object",
        "properties": {
          "report_date": {
            "type": "string",
            "description": ""
          }
        },
        "required": [
          "report_date"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_pending_strs",
      "description": "Get pending STRs awaiting submission\n",
      "parameters": {
        "type": "object",
        "properties": {
          "limit": {
            "type": "integer",
            "description": ""
          }
        },
        "required": [
          "limit"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "submit_str",
      "description": "Submit STR to regulatory authority\n",
      "parameters": {
        "type": "object",
        "properties": {
          "str_id": {
            "type": "string",
            "description": ""
          }
        },
        "required": [
          "str_id"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "generate_investigation_report",
      "description": "Generate investigation report with evidence\n",
      "parameters": {
        "type": "object",
        "properties": {
          "case_id": {
            "type": "string",
            "description": ""
          },
          "include_evidence": {
            "type": "boolean",
            "description": ""
          }
        },
        "required": [
          "case_id",
          "include_evidence"
        ]
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
          "report_id": {
            "type": "string",
            "description": ""
          }
        },
        "required": [
          "report_id"
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

