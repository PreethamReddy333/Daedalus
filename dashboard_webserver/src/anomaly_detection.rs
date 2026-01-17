
use serde::{Deserialize, Serialize};
use anyhow::Result;
use weil_rs::runtime::Runtime;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnomalyDetectionConfig {
    pub dashboard_contract_id: String,
    pub alpha_vantage_key: String,
    pub taapi_secret: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AnomalyResult {
    pub entity_id: String,
    pub symbol: String,
    pub anomaly_type: String,
    pub confidence_score: u32,
    pub details: String,
    pub timestamp: u64,
    pub supporting_evidence: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SpoofingIndicator {
    pub order_id: String,
    pub is_spoof: bool,
    pub cancellation_rate: String,
    pub order_size_vs_market: String,
    pub price_impact: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct WashTradeIndicator {
    pub entity_id: String,
    pub counterparty_id: String,
    pub is_wash_trade: bool,
    pub volume_match: bool,
    pub price_match: bool,
    pub time_gap_seconds: u32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PumpDumpIndicator {
    pub symbol: String,
    pub is_pump_dump: bool,
    pub price_velocity: String,
    pub volume_surge: String,
    pub social_sentiment_score: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct QueryHistory {
    pub method_name: String,
    pub entity_id: String,
    pub symbol: String,
    pub timestamp: u64,
    pub natural_language_prompt: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct QueryContext {
    pub recent_queries: Vec<QueryHistory>,
    pub last_entity_id: String,
    pub last_symbol: String,
}


pub struct AnomalyDetectionProxy {
    contract_id: String,
}

impl AnomalyDetectionProxy {
    pub fn new(contract_id: String) -> Self {
        AnomalyDetectionProxy {
            contract_id,
        }
    }
}

impl AnomalyDetectionProxy {
    pub fn get_context(&self) -> Result<QueryContext> {
        let serialized_args = None;

        let resp = Runtime::call_contract::<QueryContext>(
            self.contract_id.to_string(),
            "get_context".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn detect_spoofing(&self, order_id: String, entity_id: String, symbol: String, order_details: String) -> Result<SpoofingIndicator> {

        #[derive(Debug, Serialize)]
        struct detect_spoofingArgs {
            order_id: String,
            entity_id: String,
            symbol: String,
            order_details: String,
        }

        let serialized_args = Some(serde_json::to_string(&detect_spoofingArgs { order_id, entity_id, symbol, order_details }).unwrap());

        let resp = Runtime::call_contract::<SpoofingIndicator>(
            self.contract_id.to_string(),
            "detect_spoofing".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn detect_wash_trading(&self, entity_id: String, counterparty_id: String, symbol: String, trade_timestamp: u64) -> Result<WashTradeIndicator> {

        #[derive(Debug, Serialize)]
        struct detect_wash_tradingArgs {
            entity_id: String,
            counterparty_id: String,
            symbol: String,
            trade_timestamp: u64,
        }

        let serialized_args = Some(serde_json::to_string(&detect_wash_tradingArgs { entity_id, counterparty_id, symbol, trade_timestamp }).unwrap());

        let resp = Runtime::call_contract::<WashTradeIndicator>(
            self.contract_id.to_string(),
            "detect_wash_trading".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn detect_pump_dump(&self, symbol: String, time_window_minutes: u32) -> Result<PumpDumpIndicator> {

        #[derive(Debug, Serialize)]
        struct detect_pump_dumpArgs {
            symbol: String,
            time_window_minutes: u32,
        }

        let serialized_args = Some(serde_json::to_string(&detect_pump_dumpArgs { symbol, time_window_minutes }).unwrap());

        let resp = Runtime::call_contract::<PumpDumpIndicator>(
            self.contract_id.to_string(),
            "detect_pump_dump".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn detect_front_running(&self, entity_id: String, symbol: String, client_trade_timestamp: u64, prop_trade_timestamp: u64) -> Result<AnomalyResult> {

        #[derive(Debug, Serialize)]
        struct detect_front_runningArgs {
            entity_id: String,
            symbol: String,
            client_trade_timestamp: u64,
            prop_trade_timestamp: u64,
        }

        let serialized_args = Some(serde_json::to_string(&detect_front_runningArgs { entity_id, symbol, client_trade_timestamp, prop_trade_timestamp }).unwrap());

        let resp = Runtime::call_contract::<AnomalyResult>(
            self.contract_id.to_string(),
            "detect_front_running".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn analyze_volume_anomaly(&self, symbol: String, interval: String) -> Result<AnomalyResult> {

        #[derive(Debug, Serialize)]
        struct analyze_volume_anomalyArgs {
            symbol: String,
            interval: String,
        }

        let serialized_args = Some(serde_json::to_string(&analyze_volume_anomalyArgs { symbol, interval }).unwrap());

        let resp = Runtime::call_contract::<AnomalyResult>(
            self.contract_id.to_string(),
            "analyze_volume_anomaly".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn check_rsi_levels(&self, symbol: String) -> Result<String> {

        #[derive(Debug, Serialize)]
        struct check_rsi_levelsArgs {
            symbol: String,
        }

        let serialized_args = Some(serde_json::to_string(&check_rsi_levelsArgs { symbol }).unwrap());

        let resp = Runtime::call_contract::<String>(
            self.contract_id.to_string(),
            "check_rsi_levels".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn scan_entity_anomalies(&self, entity_id: String) -> Result<Vec<AnomalyResult>> {

        #[derive(Debug, Serialize)]
        struct scan_entity_anomaliesArgs {
            entity_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&scan_entity_anomaliesArgs { entity_id }).unwrap());

        let resp = Runtime::call_contract::<Vec<AnomalyResult>>(
            self.contract_id.to_string(),
            "scan_entity_anomalies".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

}
