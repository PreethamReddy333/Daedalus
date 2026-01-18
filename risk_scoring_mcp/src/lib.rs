
//! ## Risk Levels:
//! - 0-30: LOW
//! - 31-60: MEDIUM
//! - 61-80: HIGH
//! - 81-100: CRITICAL

use serde::{Deserialize, Serialize};
use weil_macros::{constructor, query, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::runtime::Runtime;

// ===== CONFIGURATION =====

#[derive(Debug, Serialize, Deserialize, WeilType, Default, Clone)]
pub struct RiskScoringConfig {
    pub dashboard_contract_id: String,
    pub high_risk_threshold: String,
    pub critical_risk_threshold: String,
}

// ===== DATA STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct RiskFactor {
    pub factor_name: String,
    pub factor_weight: u32,
    pub factor_value: String,
    pub contribution: u32,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct RiskScore {
    pub score: u32,
    pub risk_level: String,
    pub factors: Vec<RiskFactor>,
    pub recommendation: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct EntityRiskProfile {
    pub entity_id: String,
    pub overall_score: u32,
    pub insider_risk: u32,
    pub manipulation_risk: u32,
    pub aml_risk: u32,
    pub historical_alerts: u32,
    pub last_updated: u64,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct PatternRiskResult {
    pub pattern_type: String,
    pub confidence: u32,
    pub affected_trades: Vec<String>,
    pub affected_entities: Vec<String>,
    pub risk_score: u32,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct Alert {
    pub id: String,
    pub alert_type: String,
    pub severity: String,
    pub risk_score: u32,
    pub entity_id: String,
    pub symbol: String,
    pub description: String,
    pub workflow_id: String,
    pub timestamp: u64,
}

// ===== TRAIT DEFINITION =====

trait RiskScoring {
    fn new() -> Result<Self, String> where Self: Sized;
    async fn calculate_trade_risk(&self, trade_id: String, symbol: String, account_id: String, trade_type: String, quantity: u64, price: String, volume_ratio: String, is_pre_announcement: String, is_connected_entity: String) -> Result<RiskScore, String>;
    async fn calculate_entity_risk(&self, entity_id: String, days_back: u32) -> Result<EntityRiskProfile, String>;
    async fn evaluate_pattern_risk(&self, pattern_type: String, symbol: String, trade_ids: String, account_ids: String) -> Result<PatternRiskResult, String>;
    async fn evaluate_insider_risk(&self, symbol: String, account_id: String, announcement_timestamp: u64, lookback_days: u32) -> Result<RiskScore, String>;
    async fn get_risk_factors(&self, target_id: String, target_type: String) -> Result<Vec<RiskFactor>, String>;
    async fn get_symbol_risk(&self, symbol: String, as_of_timestamp: u64) -> Result<RiskScore, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

// ===== CONTRACT STATE =====

#[derive(Serialize, Deserialize, WeilType)]
pub struct RiskScoringContractState {
    secrets: Secrets<RiskScoringConfig>,
}

// ===== HELPER METHODS =====

impl RiskScoringContractState {
    fn get_risk_level(&self, score: u32) -> String {
        let config = self.secrets.config();
        let high_threshold = config.high_risk_threshold.parse::<u32>().unwrap_or(70);
        let critical_threshold = config.critical_risk_threshold.parse::<u32>().unwrap_or(90);
        
        if score >= critical_threshold {
            "CRITICAL".to_string()
        } else if score >= high_threshold {
            "HIGH".to_string()
        } else if score >= 40 {
            "MEDIUM".to_string()
        } else {
            "LOW".to_string()
        }
    }
    
    fn get_recommendation(&self, risk_level: &str) -> String {
        match risk_level {
            "CRITICAL" => "Immediate investigation required. Escalate to compliance head.".to_string(),
            "HIGH" => "Open investigation case. Review all trades by this entity.".to_string(),
            "MEDIUM" => "Add to watchlist. Monitor for next 30 days.".to_string(),
            _ => "No immediate action required. Continue routine monitoring.".to_string(),
        }
    }
    
    async fn maybe_push_alert(&self, risk_score: &RiskScore, entity_id: &str, symbol: &str, trade_id: &str) -> Result<(), String> {
        if risk_score.risk_level == "HIGH" || risk_score.risk_level == "CRITICAL" {
            let config = self.secrets.config();
            if !config.dashboard_contract_id.is_empty() {
                let alert = Alert {
                    id: format!("ALERT-{}", trade_id),
                    alert_type: if risk_score.factors.iter().any(|f| f.factor_name.contains("Insider")) {
                        "INSIDER".to_string()
                    } else {
                        "HIGH_RISK_TRADE".to_string()
                    },
                    severity: risk_score.risk_level.clone(),
                    risk_score: risk_score.score,
                    entity_id: entity_id.to_string(),
                    symbol: symbol.to_string(),
                    description: risk_score.recommendation.clone(),
                    workflow_id: "".to_string(),
                    timestamp: 0,
                };
                
                let args = serde_json::to_string(&alert).unwrap();
                let _ = Runtime::call_contract::<String>(
                    config.dashboard_contract_id.clone(),
                    "push_alert".to_string(),
                    Some(args),
                );
            }
        }
        Ok(())
    }
}

// ===== CONTRACT IMPLEMENTATION =====

#[smart_contract]
impl RiskScoring for RiskScoringContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(RiskScoringContractState {
            secrets: Secrets::new(),
        })
    }

    #[query]
    async fn calculate_trade_risk(
        &self, 
        trade_id: String, 
        symbol: String, 
        account_id: String, 
        trade_type: String, 
        quantity: u64, 
        price: String, 
        volume_ratio: String, 
        is_pre_announcement: String, 
        is_connected_entity: String
    ) -> Result<RiskScore, String> {
        let mut factors = Vec::new();
        let mut total_score: u32 = 0;
        
        let vol_ratio = volume_ratio.parse::<f64>().unwrap_or(1.0);
        let volume_score = if vol_ratio > 5.0 {
            25
        } else if vol_ratio > 3.0 {
            ((vol_ratio - 3.0) / 2.0 * 15.0 + 10.0) as u32
        } else if vol_ratio > 2.0 {
            ((vol_ratio - 2.0) * 10.0) as u32
        } else {
            0
        };
        if volume_score > 0 {
            factors.push(RiskFactor {
                factor_name: "Volume Anomaly".to_string(),
                factor_weight: 25,
                factor_value: format!("{}x normal", vol_ratio),
                contribution: volume_score,
            });
            total_score += volume_score;
        }
        
        let is_pre = is_pre_announcement.to_lowercase() == "true";
        if is_pre {
            let pre_score = 30u32;
            factors.push(RiskFactor {
                factor_name: "Pre-Announcement Trading".to_string(),
                factor_weight: 30,
                factor_value: "Trade within 30 days before material announcement".to_string(),
                contribution: pre_score,
            });
            total_score += pre_score;
        }
        
        let is_connected = is_connected_entity.to_lowercase() == "true";
        if is_connected {
            let connected_score = 25u32;
            factors.push(RiskFactor {
                factor_name: "Insider Connection".to_string(),
                factor_weight: 25,
                factor_value: "Account connected to company insider".to_string(),
                contribution: connected_score,
            });
            total_score += connected_score;
        }
        
        let price_val = price.parse::<f64>().unwrap_or(0.0);
        let trade_value = quantity as f64 * price_val;
        let size_score = if trade_value > 10_000_000.0 {
            10
        } else if trade_value > 1_000_000.0 {
            ((trade_value - 1_000_000.0) / 9_000_000.0 * 7.0 + 3.0) as u32
        } else if trade_value > 100_000.0 {
            ((trade_value - 100_000.0) / 900_000.0 * 3.0) as u32
        } else {
            0
        };
        if size_score > 0 {
            factors.push(RiskFactor {
                factor_name: "Large Trade Size".to_string(),
                factor_weight: 10,
                factor_value: format!("₹{:.0} value", trade_value),
                contribution: size_score,
            });
            total_score += size_score;
        }
        
        let direction_score = if is_pre && trade_type == "BUY" { 10 } else { 0 };
        if direction_score > 0 {
            factors.push(RiskFactor {
                factor_name: "Directional Bet".to_string(),
                factor_weight: 10,
                factor_value: format!("{} before positive announcement", trade_type),
                contribution: direction_score,
            });
            total_score += direction_score;
        }
        
        total_score = total_score.min(100);
        
        let risk_level = self.get_risk_level(total_score);
        let recommendation = self.get_recommendation(&risk_level);
        
        let risk_score = RiskScore {
            score: total_score,
            risk_level,
            factors,
            recommendation,
        };
        
        let _ = self.maybe_push_alert(&risk_score, &account_id, &symbol, &trade_id).await;
        
        Ok(risk_score)
    }

    #[query]
    async fn calculate_entity_risk(&self, entity_id: String, _days_back: u32) -> Result<EntityRiskProfile, String> {
        
        Ok(EntityRiskProfile {
            entity_id,
            overall_score: 45,
            insider_risk: 30,
            manipulation_risk: 20,
            aml_risk: 15,
            historical_alerts: 3,
            last_updated: 0,
        })
    }

    #[query]
    async fn evaluate_pattern_risk(
        &self, 
        pattern_type: String, 
        symbol: String, 
        trade_ids: String, 
        account_ids: String
    ) -> Result<PatternRiskResult, String> {
        let trades: Vec<String> = trade_ids.split(',').map(|s| s.trim().to_string()).collect();
        let accounts: Vec<String> = account_ids.split(',').map(|s| s.trim().to_string()).collect();
        
        
        let (confidence, risk_score) = match pattern_type.as_str() {
            "SPOOFING" => {
                (75, 80)
            },
            "WASH_TRADE" => {
                let is_wash = accounts.len() >= 2;
                if is_wash { (85, 90) } else { (20, 30) }
            },
            "CIRCULAR" => {
                (60, 70)
            },
            "PUMP_DUMP" => {
                (70, 75)
            },
            _ => (0, 0),
        };
        
        Ok(PatternRiskResult {
            pattern_type,
            confidence,
            affected_trades: trades,
            affected_entities: accounts,
            risk_score,
        })
    }

    #[query]
    async fn evaluate_insider_risk(
        &self, 
        symbol: String, 
        account_id: String, 
        _announcement_timestamp: u64, 
        lookback_days: u32
    ) -> Result<RiskScore, String> {
        let mut factors = Vec::new();
        let mut total_score: u32 = 0;
        
        let timing_score = if lookback_days <= 7 { 40 } else if lookback_days <= 14 { 30 } else if lookback_days <= 30 { 20 } else { 10 };
        factors.push(RiskFactor {
            factor_name: "Pre-Announcement Timing".to_string(),
            factor_weight: 40,
            factor_value: format!("{} days before announcement", lookback_days),
            contribution: timing_score,
        });
        total_score += timing_score;
        
        let profit_score = 25;
        factors.push(RiskFactor {
            factor_name: "Post-Announcement Profit".to_string(),
            factor_weight: 30,
            factor_value: "Significant profit realized".to_string(),
            contribution: profit_score,
        });
        total_score += profit_score;
        
        let pattern_score = 15;
        factors.push(RiskFactor {
            factor_name: "Unusual Trading Pattern".to_string(),
            factor_weight: 20,
            factor_value: "First trade in this stock in 90 days".to_string(),
            contribution: pattern_score,
        });
        total_score += pattern_score;
        
        total_score = total_score.min(100);
        let risk_level = self.get_risk_level(total_score);
        let recommendation = self.get_recommendation(&risk_level);
        
        let risk_score = RiskScore {
            score: total_score,
            risk_level,
            factors,
            recommendation,
        };
        
        let _ = self.maybe_push_alert(&risk_score, &account_id, &symbol, &format!("INSIDER-{}", account_id)).await;
        
        Ok(risk_score)
    }

    #[query]
    async fn get_risk_factors(&self, _target_id: String, target_type: String) -> Result<Vec<RiskFactor>, String> {

        let factors = if target_type == "TRADE" {
            vec![
                RiskFactor { factor_name: "Volume Anomaly".to_string(), factor_weight: 25, factor_value: "3.5x normal".to_string(), contribution: 20 },
                RiskFactor { factor_name: "Pre-Announcement".to_string(), factor_weight: 30, factor_value: "14 days before".to_string(), contribution: 25 },
            ]
        } else {
            vec![
                RiskFactor { factor_name: "Historical Alerts".to_string(), factor_weight: 20, factor_value: "3 alerts in 90 days".to_string(), contribution: 15 },
                RiskFactor { factor_name: "Insider Connection".to_string(), factor_weight: 30, factor_value: "2 hop from insider".to_string(), contribution: 20 },
            ]
        };
        
        Ok(factors)
    }

    #[query]
    async fn get_symbol_risk(&self, symbol: String, _as_of_timestamp: u64) -> Result<RiskScore, String> {
        let factors = vec![
            RiskFactor {
                factor_name: "Volume Spike".to_string(),
                factor_weight: 25,
                factor_value: "2.8x 30-day average".to_string(),
                contribution: 15,
            },
            RiskFactor {
                factor_name: "Price Volatility".to_string(),
                factor_weight: 20,
                factor_value: "12% intraday swing".to_string(),
                contribution: 12,
            },
            RiskFactor {
                factor_name: "Concentrated Trading".to_string(),
                factor_weight: 25,
                factor_value: "Top 5 accounts = 68% volume".to_string(),
                contribution: 18,
            },
        ];
        
        let total_score: u32 = factors.iter().map(|f| f.contribution).sum();
        let risk_level = self.get_risk_level(total_score);
        let recommendation = self.get_recommendation(&risk_level);
        
        Ok(RiskScore {
            score: total_score,
            risk_level,
            factors,
            recommendation,
        })
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {"type": "function", "function": {"name": "calculate_trade_risk", "description": "Calculate risk score for a trade. Returns 0-100 score with level and factors.", "parameters": {"type": "object", "properties": {"trade_id": {"type": "string"}, "symbol": {"type": "string"}, "account_id": {"type": "string"}, "trade_type": {"type": "string"}, "quantity": {"type": "integer"}, "price": {"type": "string"}, "volume_ratio": {"type": "string"}, "is_pre_announcement": {"type": "string"}, "is_connected_entity": {"type": "string"}}, "required": ["trade_id", "symbol", "account_id", "trade_type", "quantity", "price", "volume_ratio", "is_pre_announcement", "is_connected_entity"]}}},
  {"type": "function", "function": {"name": "calculate_entity_risk", "description": "Calculate risk profile for entity (trader/company).", "parameters": {"type": "object", "properties": {"entity_id": {"type": "string"}, "days_back": {"type": "integer"}}, "required": ["entity_id", "days_back"]}}},
  {"type": "function", "function": {"name": "evaluate_pattern_risk", "description": "Evaluate manipulation pattern: SPOOFING, WASH_TRADE, CIRCULAR, PUMP_DUMP.", "parameters": {"type": "object", "properties": {"pattern_type": {"type": "string"}, "symbol": {"type": "string"}, "trade_ids": {"type": "string"}, "account_ids": {"type": "string"}}, "required": ["pattern_type", "symbol", "trade_ids", "account_ids"]}}},
  {"type": "function", "function": {"name": "evaluate_insider_risk", "description": "Evaluate insider trading risk for trades before announcement.", "parameters": {"type": "object", "properties": {"symbol": {"type": "string"}, "account_id": {"type": "string"}, "announcement_timestamp": {"type": "integer"}, "lookback_days": {"type": "integer"}}, "required": ["symbol", "account_id", "announcement_timestamp", "lookback_days"]}}},
  {"type": "function", "function": {"name": "get_risk_factors", "description": "Get detailed breakdown of risk factors.", "parameters": {"type": "object", "properties": {"target_id": {"type": "string"}, "target_type": {"type": "string"}}, "required": ["target_id", "target_type"]}}},
  {"type": "function", "function": {"name": "get_symbol_risk", "description": "Get aggregated risk for a stock symbol.", "parameters": {"type": "object", "properties": {"symbol": {"type": "string"}, "as_of_timestamp": {"type": "integer"}}, "required": ["symbol", "as_of_timestamp"]}}}
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{ "prompts": [] }"#.to_string()
    }
}
