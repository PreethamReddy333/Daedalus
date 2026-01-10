
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


#[derive(Debug, Serialize, Deserialize, WeilType, Default)]
pub struct UPSIDatabaseConfig {
    pub dashboard_contract_id: String,
    pub supabase_url: String,
    pub supabase_anon_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UPSIRecord {
    pub upsi_id: String,
    pub company_symbol: String,
    pub upsi_type: String,
    pub description: String,
    pub nature: String,
    pub created_date: u64,
    pub public_date: u64,
    pub is_public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UPSIAccessLog {
    pub access_id: String,
    pub upsi_id: String,
    pub accessor_entity_id: String,
    pub accessor_name: String,
    pub accessor_designation: String,
    pub access_timestamp: u64,
    pub access_reason: String,
    pub access_mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradingWindowStatus {
    pub company_symbol: String,
    pub window_status: String,
    pub closure_reason: String,
    pub closure_start: u64,
    pub expected_opening: u64,
}

trait UPSIDatabase {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn get_upsi(&self, upsi_id: String) -> Result<UPSIRecord, String>;
    async fn get_active_upsi(&self, company_symbol: String) -> Result<Vec<UPSIRecord>, String>;
    async fn get_upsi_access_log(&self, upsi_id: String, from_timestamp: u64, to_timestamp: u64) -> Result<Vec<UPSIAccessLog>, String>;
    async fn get_access_by_person(&self, accessor_entity_id: String, days_back: u32) -> Result<Vec<UPSIAccessLog>, String>;
    async fn check_upsi_access_before(&self, entity_id: String, company_symbol: String, before_timestamp: u64) -> Result<Vec<UPSIAccessLog>, String>;
    async fn get_trading_window(&self, company_symbol: String) -> Result<TradingWindowStatus, String>;
    async fn check_window_violation(&self, entity_id: String, company_symbol: String, trade_timestamp: u64) -> Result<bool, String>;
    async fn get_upsi_accessors(&self, upsi_id: String) -> Result<Vec<UPSIAccessLog>, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct UPSIDatabaseContractState {
    // define your contract state here!
    secrets: Secrets<UPSIDatabaseConfig>,
}

#[smart_contract]
impl UPSIDatabase for UPSIDatabaseContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!();
    }


    #[query]
    async fn get_upsi(&self, upsi_id: String) -> Result<UPSIRecord, String> {
        unimplemented!();
    }

    #[query]
    async fn get_active_upsi(&self, company_symbol: String) -> Result<Vec<UPSIRecord>, String> {
        unimplemented!();
    }

    #[query]
    async fn get_upsi_access_log(&self, upsi_id: String, from_timestamp: u64, to_timestamp: u64) -> Result<Vec<UPSIAccessLog>, String> {
        unimplemented!();
    }

    #[query]
    async fn get_access_by_person(&self, accessor_entity_id: String, days_back: u32) -> Result<Vec<UPSIAccessLog>, String> {
        unimplemented!();
    }

    #[query]
    async fn check_upsi_access_before(&self, entity_id: String, company_symbol: String, before_timestamp: u64) -> Result<Vec<UPSIAccessLog>, String> {
        unimplemented!();
    }

    #[query]
    async fn get_trading_window(&self, company_symbol: String) -> Result<TradingWindowStatus, String> {
        unimplemented!();
    }

    #[query]
    async fn check_window_violation(&self, entity_id: String, company_symbol: String, trade_timestamp: u64) -> Result<bool, String> {
        unimplemented!();
    }

    #[query]
    async fn get_upsi_accessors(&self, upsi_id: String) -> Result<Vec<UPSIAccessLog>, String> {
        unimplemented!();
    }


    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_upsi",
      "description": "Get UPSI record by ID from Supabase\n",
      "parameters": {
        "type": "object",
        "properties": {
          "upsi_id": {
            "type": "string",
            "description": "UPSI record ID\n"
          }
        },
        "required": [
          "upsi_id"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_active_upsi",
      "description": "Get all active (non-public) UPSI for a company\n",
      "parameters": {
        "type": "object",
        "properties": {
          "company_symbol": {
            "type": "string",
            "description": "Company symbol\n"
          }
        },
        "required": [
          "company_symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_upsi_access_log",
      "description": "Get UPSI access log for a specific UPSI\n",
      "parameters": {
        "type": "object",
        "properties": {
          "upsi_id": {
            "type": "string",
            "description": "UPSI ID\n"
          },
          "from_timestamp": {
            "type": "integer",
            "description": "Start timestamp\n"
          },
          "to_timestamp": {
            "type": "integer",
            "description": "End timestamp\n"
          }
        },
        "required": [
          "upsi_id",
          "from_timestamp",
          "to_timestamp"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_access_by_person",
      "description": "Get all UPSI accesses by a specific person\n",
      "parameters": {
        "type": "object",
        "properties": {
          "accessor_entity_id": {
            "type": "string",
            "description": "Entity ID of accessor\n"
          },
          "days_back": {
            "type": "integer",
            "description": "Number of days to look back\n"
          }
        },
        "required": [
          "accessor_entity_id",
          "days_back"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "check_upsi_access_before",
      "description": "Check if an entity had UPSI access before a date\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity to check\n"
          },
          "company_symbol": {
            "type": "string",
            "description": "Company symbol\n"
          },
          "before_timestamp": {
            "type": "integer",
            "description": "Date to check before\n"
          }
        },
        "required": [
          "entity_id",
          "company_symbol",
          "before_timestamp"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_trading_window",
      "description": "Get trading window status for a company\n",
      "parameters": {
        "type": "object",
        "properties": {
          "company_symbol": {
            "type": "string",
            "description": "Company symbol\n"
          }
        },
        "required": [
          "company_symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "check_window_violation",
      "description": "Check if entity traded during closed window\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity ID\n"
          },
          "company_symbol": {
            "type": "string",
            "description": "Company symbol\n"
          },
          "trade_timestamp": {
            "type": "integer",
            "description": "Trade timestamp to check\n"
          }
        },
        "required": [
          "entity_id",
          "company_symbol",
          "trade_timestamp"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_upsi_accessors",
      "description": "Get all entities who accessed a specific UPSI\n",
      "parameters": {
        "type": "object",
        "properties": {
          "upsi_id": {
            "type": "string",
            "description": "UPSI ID\n"
          }
        },
        "required": [
          "upsi_id"
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

