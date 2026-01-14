
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


#[derive(Debug, Serialize, Deserialize, WeilType, Default)]
pub struct TradeDataConfig {
    pub api_key_1: String,
    pub api_key_2: String,
    pub api_key_3: String,
    pub dashboard_contract_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub trade_id: String,
    pub symbol: String,
    pub account_id: String,
    pub trade_type: String,
    pub quantity: u64,
    pub price: String,
    pub value: String,
    pub exchange: String,
    pub segment: String,
    pub timestamp: u64,
    pub order_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeAnalysis {
    pub symbol: String,
    pub total_volume: u64,
    pub avg_price: String,
    pub high_price: String,
    pub low_price: String,
    pub buy_volume: u64,
    pub sell_volume: u64,
    pub trade_count: u32,
    pub concentration_ratio: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeAnomaly {
    pub symbol: String,
    pub current_volume: u64,
    pub avg_volume_30d: u64,
    pub volume_ratio: String,
    pub is_anomaly: bool,
    pub anomaly_score: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountActivity {
    pub account_id: String,
    pub symbol: String,
    pub buy_quantity: u64,
    pub sell_quantity: u64,
    pub net_position: i64,
    pub trade_count: u32,
    pub first_trade_time: u64,
    pub last_trade_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryHistory {
    pub method_name: String,
    pub symbol: String,
    pub account_id: String,
    pub timestamp: u64,
    pub natural_language_prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryContext {
    pub recent_queries: Vec<QueryHistory>,
    pub last_symbol: String,
    pub last_account_id: String,
}

trait TradeData {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn get_context(&mut self) -> QueryContext;
    async fn get_trade(&mut self, trade_id: String) -> Result<Trade, String>;
    async fn get_trades_by_symbol(&mut self, symbol: String, limit: u32) -> Result<Vec<Trade>, String>;
    async fn get_trades_by_account(&mut self, account_id: String, limit: u32) -> Result<Vec<Trade>, String>;
    async fn get_trades_by_accounts(&mut self, account_ids: String, symbol: String) -> Result<Vec<Trade>, String>;
    async fn analyze_volume(&mut self, symbol: String) -> Result<TradeAnalysis, String>;
    async fn detect_volume_anomaly(&mut self, symbol: String) -> Result<VolumeAnomaly, String>;
    async fn get_top_traders(&mut self, symbol: String, limit: u32) -> Result<Vec<AccountActivity>, String>;
    async fn get_large_orders(&mut self, min_value: u64) -> Result<Vec<Trade>, String>;
    async fn get_account_profile(&mut self, account_id: String) -> Result<Vec<AccountActivity>, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct TradeDataContractState {
    // define your contract state here!
    secrets: Secrets<TradeDataConfig>,
}

#[smart_contract]
impl TradeData for TradeDataContractState {
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
    async fn get_trade(&mut self, trade_id: String) -> Result<Trade, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_trades_by_symbol(&mut self, symbol: String, limit: u32) -> Result<Vec<Trade>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_trades_by_account(&mut self, account_id: String, limit: u32) -> Result<Vec<Trade>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_trades_by_accounts(&mut self, account_ids: String, symbol: String) -> Result<Vec<Trade>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn analyze_volume(&mut self, symbol: String) -> Result<TradeAnalysis, String> {
        unimplemented!();
    }

    #[mutate]
    async fn detect_volume_anomaly(&mut self, symbol: String) -> Result<VolumeAnomaly, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_top_traders(&mut self, symbol: String, limit: u32) -> Result<Vec<AccountActivity>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_large_orders(&mut self, min_value: u64) -> Result<Vec<Trade>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_account_profile(&mut self, account_id: String) -> Result<Vec<AccountActivity>, String> {
        unimplemented!();
    }


    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_context",
      "description": "CALL THIS FIRST - Get context from recent queries\nReturns cached symbols and account_ids for fuzzy resolution\n",
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
      "name": "get_trade",
      "description": "Fetch a single trade by ID\n",
      "parameters": {
        "type": "object",
        "properties": {
          "trade_id": {
            "type": "string",
            "description": "Unique trade identifier\n"
          }
        },
        "required": [
          "trade_id"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_trades_by_symbol",
      "description": "Fetch trades for a symbol\nTimestamps are optional - defaults to current day\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Stock symbol (e.g., \"IBM\", \"AAPL\") - supports fuzzy matching\n"
          },
          "limit": {
            "type": "integer",
            "description": "Max trades to return (default: 10)\n"
          }
        },
        "required": [
          "symbol",
          "limit"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_trades_by_account",
      "description": "Fetch trades for a specific account\n",
      "parameters": {
        "type": "object",
        "properties": {
          "account_id": {
            "type": "string",
            "description": "Trading account ID - supports fuzzy matching\n"
          },
          "limit": {
            "type": "integer",
            "description": "Max trades to return (default: 10)\n"
          }
        },
        "required": [
          "account_id",
          "limit"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_trades_by_accounts",
      "description": "Get trades by multiple accounts\n",
      "parameters": {
        "type": "object",
        "properties": {
          "account_ids": {
            "type": "string",
            "description": "Comma-separated list of account IDs\n"
          },
          "symbol": {
            "type": "string",
            "description": "Stock symbol to filter by\n"
          }
        },
        "required": [
          "account_ids",
          "symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "analyze_volume",
      "description": "Analyze volume for a symbol\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Stock symbol - supports fuzzy matching\n"
          }
        },
        "required": [
          "symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "detect_volume_anomaly",
      "description": "Detect volume anomalies\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Stock symbol - supports fuzzy matching\n"
          }
        },
        "required": [
          "symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_top_traders",
      "description": "Get top traders for a symbol\n",
      "parameters": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Stock symbol\n"
          },
          "limit": {
            "type": "integer",
            "description": "Number of top traders to return (default: 5)\n"
          }
        },
        "required": [
          "symbol",
          "limit"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_large_orders",
      "description": "Fetch large institutional orders\n",
      "parameters": {
        "type": "object",
        "properties": {
          "min_value": {
            "type": "integer",
            "description": "Minimum order value\n"
          }
        },
        "required": [
          "min_value"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_account_profile",
      "description": "Get trading activity for an account\n",
      "parameters": {
        "type": "object",
        "properties": {
          "account_id": {
            "type": "string",
            "description": "Trading account ID - supports fuzzy matching\n"
          }
        },
        "required": [
          "account_id"
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

