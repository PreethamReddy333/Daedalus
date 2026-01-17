use serde::{Deserialize, Serialize};
use weil_rs::errors::WeilError;
use weil_macros::WeilType;
use weil_wallet::{contract::ContractId, streaming::ByteStream, wallet::{PrivateKey, Wallet}, WeilClient, WeilContractClient};

#[derive(Debug, Serialize, Deserialize, WeilType, Default)]
pub struct DashboardConfig {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub id: String,
    pub workflow_type: String,
    pub trigger: String,
    pub steps_completed: u32,
    pub total_steps: u32,
    pub status: String,
    pub started_at: u64,
    pub completed_at: u64,
    pub result_summary: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseRecord {
    pub case_id: String,
    pub case_type: String,
    pub status: String,
    pub priority: String,
    pub subject_entity: String,
    pub symbol: String,
    pub risk_score: u32,
    pub assigned_to: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub summary: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurveillanceStats {
    pub total_alerts_today: u32,
    pub total_workflows_today: u32,
    pub open_cases: u32,
    pub high_risk_entities: u32,
    pub compliance_score: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiskEntity {
    pub entity_id: String,
    pub entity_name: String,
    pub risk_score: u32,
    pub alert_count: u32,
    pub last_alert_at: u64,
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
pub struct Entity {
    pub entity_id: String,
    pub entity_type: String,
    pub name: String,
    pub pan_number: String,
    pub registration_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub source_entity_id: String,
    pub target_entity_id: String,
    pub relationship_type: String,
    pub relationship_detail: String,
    pub strength: u32,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsiderStatus {
    pub entity_id: String,
    pub company_symbol: String,
    pub is_insider: bool,
    pub insider_type: String,
    pub designation: String,
    pub window_status: String,
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
pub struct TradingWindowStatus {
    pub company_symbol: String,
    pub window_status: String,
    pub closure_reason: String,
    pub closure_start: u64,
    pub expected_opening: u64,
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

struct DashboardWebserverClient {
    client: WeilContractClient,
}

impl DashboardWebserverClient {
    pub fn new(contract_id: ContractId, wallet: Wallet) -> Result<Self, anyhow::Error> {
        Ok(DashboardWebserverClient {
            client: WeilClient::new(wallet, None)?.to_contract_client(contract_id),
        })
    }

    pub async fn ping(&self) -> Result<String, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
        }

        let args = Args {  };

        let resp = self.client.execute("ping".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<String>(&result)?;

        Ok(result)
    }

    pub async fn push_alert(&self, alert: Alert) -> Result<String, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            alert: Alert,
        }

        let args = Args { alert };

        let resp = self.client.execute("push_alert".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<String>(&result)?;

        Ok(result)
    }

    pub async fn log_workflow_start(&self, workflow_id: String, workflow_type: String, trigger: String, total_steps: u32) -> Result<String, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            workflow_id: String,
            workflow_type: String,
            trigger: String,
            total_steps: u32,
        }

        let args = Args { workflow_id, workflow_type, trigger, total_steps };

        let resp = self.client.execute("log_workflow_start".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<String>(&result)?;

        Ok(result)
    }

    pub async fn update_workflow_progress(&self, workflow_id: String, steps_completed: u32, status: String, result_summary: String) -> Result<String, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            workflow_id: String,
            steps_completed: u32,
            status: String,
            result_summary: String,
        }

        let args = Args { workflow_id, steps_completed, status, result_summary };

        let resp = self.client.execute("update_workflow_progress".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<String>(&result)?;

        Ok(result)
    }

    pub async fn upsert_case(&self, case_record: CaseRecord) -> Result<String, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            case_record: CaseRecord,
        }

        let args = Args { case_record };

        let resp = self.client.execute("upsert_case".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<String>(&result)?;

        Ok(result)
    }

    pub async fn register_risk_entity(&self, entity: RiskEntity) -> Result<String, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            entity: RiskEntity,
        }

        let args = Args { entity };

        let resp = self.client.execute("register_risk_entity".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<String>(&result)?;

        Ok(result)
    }

    pub async fn get_live_alerts(&self, severity_filter: Option<String>, limit: Option<u32>) -> Result<Vec<Alert>, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            severity_filter: Option<String>,
            limit: Option<u32>,
        }

        let args = Args { severity_filter, limit };

        let resp = self.client.execute("get_live_alerts".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<Vec<Alert>>(&result)?;

        Ok(result)
    }

    pub async fn get_workflow_history(&self, workflow_type: Option<String>, limit: Option<u32>) -> Result<Vec<WorkflowExecution>, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            workflow_type: Option<String>,
            limit: Option<u32>,
        }

        let args = Args { workflow_type, limit };

        let resp = self.client.execute("get_workflow_history".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<Vec<WorkflowExecution>>(&result)?;

        Ok(result)
    }

    pub async fn get_cases_by_status(&self, status: Option<String>, limit: Option<u32>) -> Result<Vec<CaseRecord>, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            status: Option<String>,
            limit: Option<u32>,
        }

        let args = Args { status, limit };

        let resp = self.client.execute("get_cases_by_status".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<Vec<CaseRecord>>(&result)?;

        Ok(result)
    }

    pub async fn get_stats(&self) -> Result<SurveillanceStats, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
        }

        let args = Args {  };

        let resp = self.client.execute("get_stats".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<SurveillanceStats>(&result)?;

        Ok(result)
    }

    pub async fn get_high_risk_entities(&self, min_risk_score: Option<u32>, limit: Option<u32>) -> Result<Vec<RiskEntity>, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            min_risk_score: Option<u32>,
            limit: Option<u32>,
        }

        let args = Args { min_risk_score, limit };

        let resp = self.client.execute("get_high_risk_entities".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<Vec<RiskEntity>>(&result)?;

        Ok(result)
    }

    pub async fn get_case_details(&self, case_id: String) -> Result<CaseRecord, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            case_id: String,
        }

        let args = Args { case_id };

        let resp = self.client.execute("get_case_details".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<CaseRecord>(&result)?;

        Ok(result)
    }

    pub async fn get_entity_alerts(&self, entity_id: String, limit: Option<u32>) -> Result<Vec<Alert>, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            entity_id: String,
            limit: Option<u32>,
        }

        let args = Args { entity_id, limit };

        let resp = self.client.execute("get_entity_alerts".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<Vec<Alert>>(&result)?;

        Ok(result)
    }

    pub async fn get_trades_proxy(&self, symbol: String, limit: Option<u32>) -> Result<Vec<Trade>, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            symbol: String,
            limit: Option<u32>,
        }

        let args = Args { symbol, limit };

        let resp = self.client.execute("get_trades_proxy".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<Vec<Trade>>(&result)?;

        Ok(result)
    }

    pub async fn search_entities_proxy(&self, search_query: String) -> Result<Vec<Entity>, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            search_query: String,
        }

        let args = Args { search_query };

        let resp = self.client.execute("search_entities_proxy".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<Vec<Entity>>(&result)?;

        Ok(result)
    }

    pub async fn get_relationships_proxy(&self, entity_id: String) -> Result<Vec<Relationship>, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            entity_id: String,
        }

        let args = Args { entity_id };

        let resp = self.client.execute("get_relationships_proxy".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<Vec<Relationship>>(&result)?;

        Ok(result)
    }

    pub async fn check_insider_proxy(&self, entity_id: String, company_symbol: String) -> Result<InsiderStatus, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            entity_id: String,
            company_symbol: String,
        }

        let args = Args { entity_id, company_symbol };

        let resp = self.client.execute("check_insider_proxy".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<InsiderStatus>(&result)?;

        Ok(result)
    }

    pub async fn get_active_upsi_proxy(&self, company_symbol: String) -> Result<Vec<UPSIRecord>, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            company_symbol: String,
        }

        let args = Args { company_symbol };

        let resp = self.client.execute("get_active_upsi_proxy".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<Vec<UPSIRecord>>(&result)?;

        Ok(result)
    }

    pub async fn get_trading_window_proxy(&self, company_symbol: String) -> Result<TradingWindowStatus, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            company_symbol: String,
        }

        let args = Args { company_symbol };

        let resp = self.client.execute("get_trading_window_proxy".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<TradingWindowStatus>(&result)?;

        Ok(result)
    }

    pub async fn analyze_volume_proxy(&self, symbol: String) -> Result<TradeAnalysis, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            symbol: String,
        }

        let args = Args { symbol };

        let resp = self.client.execute("analyze_volume_proxy".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<TradeAnalysis>(&result)?;

        Ok(result)
    }

    pub async fn generate_report_proxy(&self, report_type: String, params: String) -> Result<ReportResult, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
            report_type: String,
            params: String,
        }

        let args = Args { report_type, params };

        let resp = self.client.execute("generate_report_proxy".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<ReportResult>(&result)?;

        Ok(result)
    }

    pub async fn get_tools(&self) -> Result<String, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
        }

        let args = Args {  };

        let resp = self.client.execute("get_tools".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<String>(&result)?;

        Ok(result)
    }

    pub async fn get_prompts(&self) -> Result<String, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
        }

        let args = Args {  };

        let resp = self.client.execute("get_prompts".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<String>(&result)?;

        Ok(result)
    }

}