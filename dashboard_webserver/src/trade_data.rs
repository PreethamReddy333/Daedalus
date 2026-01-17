
use serde::{Deserialize, Serialize};
use anyhow::Result;
use weil_rs::runtime::Runtime;


#[derive(Debug, Serialize, Deserialize, Clone)]
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


pub struct TradeDataProxy {
    contract_id: String,
}

impl TradeDataProxy {
    pub fn new(contract_id: String) -> Self {
        TradeDataProxy {
            contract_id,
        }
    }
}

impl TradeDataProxy {
    pub fn get_context(&self) -> Result<QueryContext> {
        let serialized_args = None;

        let resp = Runtime::call_contract::<QueryContext>(
            self.contract_id.to_string(),
            "get_context".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_trade(&self, trade_id: String) -> Result<Trade> {

        #[derive(Debug, Serialize)]
        struct get_tradeArgs {
            trade_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&get_tradeArgs { trade_id }).unwrap());

        let resp = Runtime::call_contract::<Trade>(
            self.contract_id.to_string(),
            "get_trade".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_trades_by_symbol(&self, symbol: String, limit: u32) -> Result<Vec<Trade>> {

        #[derive(Debug, Serialize)]
        struct get_trades_by_symbolArgs {
            symbol: String,
            limit: u32,
        }

        let serialized_args = Some(serde_json::to_string(&get_trades_by_symbolArgs { symbol, limit }).unwrap());

        let resp = Runtime::call_contract::<Vec<Trade>>(
            self.contract_id.to_string(),
            "get_trades_by_symbol".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_trades_by_account(&self, account_id: String, limit: u32) -> Result<Vec<Trade>> {

        #[derive(Debug, Serialize)]
        struct get_trades_by_accountArgs {
            account_id: String,
            limit: u32,
        }

        let serialized_args = Some(serde_json::to_string(&get_trades_by_accountArgs { account_id, limit }).unwrap());

        let resp = Runtime::call_contract::<Vec<Trade>>(
            self.contract_id.to_string(),
            "get_trades_by_account".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_trades_by_accounts(&self, account_ids: String, symbol: String) -> Result<Vec<Trade>> {

        #[derive(Debug, Serialize)]
        struct get_trades_by_accountsArgs {
            account_ids: String,
            symbol: String,
        }

        let serialized_args = Some(serde_json::to_string(&get_trades_by_accountsArgs { account_ids, symbol }).unwrap());

        let resp = Runtime::call_contract::<Vec<Trade>>(
            self.contract_id.to_string(),
            "get_trades_by_accounts".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn analyze_volume(&self, symbol: String) -> Result<TradeAnalysis> {

        #[derive(Debug, Serialize)]
        struct analyze_volumeArgs {
            symbol: String,
        }

        let serialized_args = Some(serde_json::to_string(&analyze_volumeArgs { symbol }).unwrap());

        let resp = Runtime::call_contract::<TradeAnalysis>(
            self.contract_id.to_string(),
            "analyze_volume".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn detect_volume_anomaly(&self, symbol: String) -> Result<VolumeAnomaly> {

        #[derive(Debug, Serialize)]
        struct detect_volume_anomalyArgs {
            symbol: String,
        }

        let serialized_args = Some(serde_json::to_string(&detect_volume_anomalyArgs { symbol }).unwrap());

        let resp = Runtime::call_contract::<VolumeAnomaly>(
            self.contract_id.to_string(),
            "detect_volume_anomaly".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_top_traders(&self, symbol: String, limit: u32) -> Result<Vec<AccountActivity>> {

        #[derive(Debug, Serialize)]
        struct get_top_tradersArgs {
            symbol: String,
            limit: u32,
        }

        let serialized_args = Some(serde_json::to_string(&get_top_tradersArgs { symbol, limit }).unwrap());

        let resp = Runtime::call_contract::<Vec<AccountActivity>>(
            self.contract_id.to_string(),
            "get_top_traders".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_large_orders(&self, min_value: u64) -> Result<Vec<Trade>> {

        #[derive(Debug, Serialize)]
        struct get_large_ordersArgs {
            min_value: u64,
        }

        let serialized_args = Some(serde_json::to_string(&get_large_ordersArgs { min_value }).unwrap());

        let resp = Runtime::call_contract::<Vec<Trade>>(
            self.contract_id.to_string(),
            "get_large_orders".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_account_profile(&self, account_id: String) -> Result<Vec<AccountActivity>> {

        #[derive(Debug, Serialize)]
        struct get_account_profileArgs {
            account_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&get_account_profileArgs { account_id }).unwrap());

        let resp = Runtime::call_contract::<Vec<AccountActivity>>(
            self.contract_id.to_string(),
            "get_account_profile".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

}
