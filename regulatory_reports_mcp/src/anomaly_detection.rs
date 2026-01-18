use anyhow::Result;
use serde::{Deserialize, Serialize};
use weil_rs::runtime::Runtime;

pub struct AnomalyDetectionMcp {
    contract_id: String,
}

impl AnomalyDetectionMcp {
    pub fn new(contract_id: String) -> Self {
        AnomalyDetectionMcp { contract_id }
    }
}

// ===== Response Types =====

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnomalyResult {
    pub entity_id: String,
    pub symbol: String,
    pub anomaly_type: String,
    pub confidence_score: u32,
    pub details: String,
    pub timestamp: u64,
    pub supporting_evidence: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpoofingIndicator {
    pub order_id: String,
    pub is_spoof: bool,
    pub cancellation_rate: String,
    pub order_size_vs_market: String,
    pub price_impact: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WashTradeIndicator {
    pub entity_id: String,
    pub counterparty_id: String,
    pub is_wash_trade: bool,
    pub volume_match: bool,
    pub price_match: bool,
    pub time_gap_seconds: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PumpDumpIndicator {
    pub symbol: String,
    pub is_pump_dump: bool,
    pub price_velocity: String,
    pub volume_surge: String,
    pub social_sentiment_score: i32,
}

impl AnomalyDetectionMcp {
    pub fn scan_entity_anomalies(&self, entity_id: String) -> Result<Vec<AnomalyResult>> {
        #[derive(Debug, Serialize)]
        struct ScanEntityAnomaliesArgs {
            entity_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&ScanEntityAnomaliesArgs { entity_id })?);

        let resp = Runtime::call_contract::<Vec<AnomalyResult>>(
            self.contract_id.clone(),
            "scan_entity_anomalies".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn detect_pump_dump(&self, symbol: String, time_window_minutes: u32) -> Result<PumpDumpIndicator> {
        #[derive(Debug, Serialize)]
        struct DetectPumpDumpArgs {
            symbol: String,
            time_window_minutes: u32,
        }

        let serialized_args = Some(serde_json::to_string(&DetectPumpDumpArgs {
            symbol,
            time_window_minutes,
        })?);

        let resp = Runtime::call_contract::<PumpDumpIndicator>(
            self.contract_id.clone(),
            "detect_pump_dump".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn detect_wash_trading(
        &self,
        entity_id: String,
        counterparty_id: String,
        symbol: String,
        trade_timestamp: u64,
    ) -> Result<WashTradeIndicator> {
        #[derive(Debug, Serialize)]
        struct DetectWashTradingArgs {
            entity_id: String,
            counterparty_id: String,
            symbol: String,
            trade_timestamp: u64,
        }

        let serialized_args = Some(serde_json::to_string(&DetectWashTradingArgs {
            entity_id,
            counterparty_id,
            symbol,
            trade_timestamp,
        })?);

        let resp = Runtime::call_contract::<WashTradeIndicator>(
            self.contract_id.clone(),
            "detect_wash_trading".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn detect_spoofing(
        &self,
        order_id: String,
        entity_id: String,
        symbol: String,
        order_details: String,
    ) -> Result<SpoofingIndicator> {
        #[derive(Debug, Serialize)]
        struct DetectSpoofingArgs {
            order_id: String,
            entity_id: String,
            symbol: String,
            order_details: String,
        }

        let serialized_args = Some(serde_json::to_string(&DetectSpoofingArgs {
            order_id,
            entity_id,
            symbol,
            order_details,
        })?);

        let resp = Runtime::call_contract::<SpoofingIndicator>(
            self.contract_id.clone(),
            "detect_spoofing".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn analyze_volume_anomaly(&self, symbol: String, interval: String) -> Result<AnomalyResult> {
        #[derive(Debug, Serialize)]
        struct AnalyzeVolumeAnomalyArgs {
            symbol: String,
            interval: String,
        }

        let serialized_args = Some(serde_json::to_string(&AnalyzeVolumeAnomalyArgs {
            symbol,
            interval,
        })?);

        let resp = Runtime::call_contract::<AnomalyResult>(
            self.contract_id.clone(),
            "analyze_volume_anomaly".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }
}
