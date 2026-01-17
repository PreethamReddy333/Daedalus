
use serde::{Deserialize, Serialize};
use anyhow::Result;
use weil_rs::runtime::Runtime;


#[derive(Debug, Serialize, Deserialize, Clone)]
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


pub struct UPSIDatabaseProxy {
    contract_id: String,
}

impl UPSIDatabaseProxy {
    pub fn new(contract_id: String) -> Self {
        UPSIDatabaseProxy {
            contract_id,
        }
    }
}

impl UPSIDatabaseProxy {
    pub fn get_upsi(&self, upsi_id: String) -> Result<UPSIRecord> {

        #[derive(Debug, Serialize)]
        struct get_upsiArgs {
            upsi_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&get_upsiArgs { upsi_id }).unwrap());

        let resp = Runtime::call_contract::<UPSIRecord>(
            self.contract_id.to_string(),
            "get_upsi".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_active_upsi(&self, company_symbol: String) -> Result<Vec<UPSIRecord>> {

        #[derive(Debug, Serialize)]
        struct get_active_upsiArgs {
            company_symbol: String,
        }

        let serialized_args = Some(serde_json::to_string(&get_active_upsiArgs { company_symbol }).unwrap());

        let resp = Runtime::call_contract::<Vec<UPSIRecord>>(
            self.contract_id.to_string(),
            "get_active_upsi".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_upsi_access_log(&self, upsi_id: String, from_timestamp: u64, to_timestamp: u64) -> Result<Vec<UPSIAccessLog>> {

        #[derive(Debug, Serialize)]
        struct get_upsi_access_logArgs {
            upsi_id: String,
            from_timestamp: u64,
            to_timestamp: u64,
        }

        let serialized_args = Some(serde_json::to_string(&get_upsi_access_logArgs { upsi_id, from_timestamp, to_timestamp }).unwrap());

        let resp = Runtime::call_contract::<Vec<UPSIAccessLog>>(
            self.contract_id.to_string(),
            "get_upsi_access_log".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_access_by_person(&self, accessor_entity_id: String, days_back: u32) -> Result<Vec<UPSIAccessLog>> {

        #[derive(Debug, Serialize)]
        struct get_access_by_personArgs {
            accessor_entity_id: String,
            days_back: u32,
        }

        let serialized_args = Some(serde_json::to_string(&get_access_by_personArgs { accessor_entity_id, days_back }).unwrap());

        let resp = Runtime::call_contract::<Vec<UPSIAccessLog>>(
            self.contract_id.to_string(),
            "get_access_by_person".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn check_upsi_access_before(&self, entity_id: String, company_symbol: String, before_timestamp: u64) -> Result<Vec<UPSIAccessLog>> {

        #[derive(Debug, Serialize)]
        struct check_upsi_access_beforeArgs {
            entity_id: String,
            company_symbol: String,
            before_timestamp: u64,
        }

        let serialized_args = Some(serde_json::to_string(&check_upsi_access_beforeArgs { entity_id, company_symbol, before_timestamp }).unwrap());

        let resp = Runtime::call_contract::<Vec<UPSIAccessLog>>(
            self.contract_id.to_string(),
            "check_upsi_access_before".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_trading_window(&self, company_symbol: String) -> Result<TradingWindowStatus> {

        #[derive(Debug, Serialize)]
        struct get_trading_windowArgs {
            company_symbol: String,
        }

        let serialized_args = Some(serde_json::to_string(&get_trading_windowArgs { company_symbol }).unwrap());

        let resp = Runtime::call_contract::<TradingWindowStatus>(
            self.contract_id.to_string(),
            "get_trading_window".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn check_window_violation(&self, entity_id: String, company_symbol: String, trade_timestamp: u64) -> Result<bool> {

        #[derive(Debug, Serialize)]
        struct check_window_violationArgs {
            entity_id: String,
            company_symbol: String,
            trade_timestamp: u64,
        }

        let serialized_args = Some(serde_json::to_string(&check_window_violationArgs { entity_id, company_symbol, trade_timestamp }).unwrap());

        let resp = Runtime::call_contract::<bool>(
            self.contract_id.to_string(),
            "check_window_violation".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_upsi_accessors(&self, upsi_id: String) -> Result<Vec<UPSIAccessLog>> {

        #[derive(Debug, Serialize)]
        struct get_upsi_accessorsArgs {
            upsi_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&get_upsi_accessorsArgs { upsi_id }).unwrap());

        let resp = Runtime::call_contract::<Vec<UPSIAccessLog>>(
            self.contract_id.to_string(),
            "get_upsi_accessors".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

}
