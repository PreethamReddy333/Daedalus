//! Cross-contract bindings for Risk Scoring MCP
//!
//! Provides proxy methods to call the deployed Risk Scoring MCP contract.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use weil_rs::runtime::Runtime;

/// Proxy struct for Risk Scoring MCP cross-contract calls
pub struct RiskScoringMcp {
    contract_id: String,
}

impl RiskScoringMcp {
    pub fn new(contract_id: String) -> Self {
        RiskScoringMcp { contract_id }
    }
}

// ===== Response Types =====

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskFactor {
    pub factor_name: String,
    pub factor_weight: u32,
    pub factor_value: String,
    pub contribution: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskScore {
    pub score: u32,
    pub risk_level: String,
    pub factors: Vec<RiskFactor>,
    pub recommendation: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityRiskProfile {
    pub entity_id: String,
    pub overall_score: u32,
    pub insider_risk: u32,
    pub manipulation_risk: u32,
    pub aml_risk: u32,
    pub historical_alerts: u32,
    pub last_updated: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatternRiskResult {
    pub pattern_type: String,
    pub confidence: u32,
    pub affected_trades: Vec<String>,
    pub affected_entities: Vec<String>,
    pub risk_score: u32,
}

impl RiskScoringMcp {
    /// Calculate risk score for an entity (trader/company)
    pub fn calculate_entity_risk(&self, entity_id: String, days_back: u32) -> Result<EntityRiskProfile> {
        #[derive(Debug, Serialize)]
        struct CalculateEntityRiskArgs {
            entity_id: String,
            days_back: u32,
        }

        let serialized_args = Some(serde_json::to_string(&CalculateEntityRiskArgs {
            entity_id,
            days_back,
        })?);

        let resp = Runtime::call_contract::<EntityRiskProfile>(
            self.contract_id.clone(),
            "calculate_entity_risk".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Get risk factors breakdown for a score
    pub fn get_risk_factors(&self, target_id: String, target_type: String) -> Result<Vec<RiskFactor>> {
        #[derive(Debug, Serialize)]
        struct GetRiskFactorsArgs {
            target_id: String,
            target_type: String,
        }

        let serialized_args = Some(serde_json::to_string(&GetRiskFactorsArgs {
            target_id,
            target_type,
        })?);

        let resp = Runtime::call_contract::<Vec<RiskFactor>>(
            self.contract_id.clone(),
            "get_risk_factors".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Evaluate trading pattern for manipulation
    pub fn evaluate_pattern_risk(
        &self,
        pattern_type: String,
        symbol: String,
        trade_ids: String,
        account_ids: String,
    ) -> Result<PatternRiskResult> {
        #[derive(Debug, Serialize)]
        struct EvaluatePatternRiskArgs {
            pattern_type: String,
            symbol: String,
            trade_ids: String,
            account_ids: String,
        }

        let serialized_args = Some(serde_json::to_string(&EvaluatePatternRiskArgs {
            pattern_type,
            symbol,
            trade_ids,
            account_ids,
        })?);

        let resp = Runtime::call_contract::<PatternRiskResult>(
            self.contract_id.clone(),
            "evaluate_pattern_risk".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Get aggregated risk for a stock symbol
    pub fn get_symbol_risk(&self, symbol: String, as_of_timestamp: u64) -> Result<RiskScore> {
        #[derive(Debug, Serialize)]
        struct GetSymbolRiskArgs {
            symbol: String,
            as_of_timestamp: u64,
        }

        let serialized_args = Some(serde_json::to_string(&GetSymbolRiskArgs {
            symbol,
            as_of_timestamp,
        })?);

        let resp = Runtime::call_contract::<RiskScore>(
            self.contract_id.clone(),
            "get_symbol_risk".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }
}
