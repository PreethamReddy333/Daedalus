import {
  WeilWallet,
  Schema,
  Contract,
  ContractFactory,
  parseSchema,
  parseExecutionResult,
  WeilWalletConnection,
  Option,
} from '@weilliptic/weil-sdk';


interface DashboardConfig {
  name: string;
}
const dashboardConfigSchema = 
  Schema.object({
    name: Schema.string,
  });



interface Alert {
  id: string;
  alert_type: string;
  severity: string;
  risk_score: number;
  entity_id: string;
  symbol: string;
  description: string;
  workflow_id: string;
  timestamp: bigint;
}
const alertSchema = 
  Schema.object({
    id: Schema.string,
    alert_type: Schema.string,
    severity: Schema.string,
    risk_score: Schema.u32,
    entity_id: Schema.string,
    symbol: Schema.string,
    description: Schema.string,
    workflow_id: Schema.string,
    timestamp: Schema.u64,
  });



interface WorkflowExecution {
  id: string;
  workflow_type: string;
  trigger: string;
  steps_completed: number;
  total_steps: number;
  status: string;
  started_at: bigint;
  completed_at: bigint;
  result_summary: string;
}
const workflowExecutionSchema = 
  Schema.object({
    id: Schema.string,
    workflow_type: Schema.string,
    trigger: Schema.string,
    steps_completed: Schema.u32,
    total_steps: Schema.u32,
    status: Schema.string,
    started_at: Schema.u64,
    completed_at: Schema.u64,
    result_summary: Schema.string,
  });



interface CaseRecord {
  case_id: string;
  case_type: string;
  status: string;
  priority: string;
  subject_entity: string;
  symbol: string;
  risk_score: number;
  assigned_to: string;
  created_at: bigint;
  updated_at: bigint;
  summary: string;
}
const caseRecordSchema = 
  Schema.object({
    case_id: Schema.string,
    case_type: Schema.string,
    status: Schema.string,
    priority: Schema.string,
    subject_entity: Schema.string,
    symbol: Schema.string,
    risk_score: Schema.u32,
    assigned_to: Schema.string,
    created_at: Schema.u64,
    updated_at: Schema.u64,
    summary: Schema.string,
  });



interface SurveillanceStats {
  total_alerts_today: number;
  total_workflows_today: number;
  open_cases: number;
  high_risk_entities: number;
  compliance_score: number;
}
const surveillanceStatsSchema = 
  Schema.object({
    total_alerts_today: Schema.u32,
    total_workflows_today: Schema.u32,
    open_cases: Schema.u32,
    high_risk_entities: Schema.u32,
    compliance_score: Schema.u32,
  });



interface RiskEntity {
  entity_id: string;
  entity_name: string;
  risk_score: number;
  alert_count: number;
  last_alert_at: bigint;
}
const riskEntitySchema = 
  Schema.object({
    entity_id: Schema.string,
    entity_name: Schema.string,
    risk_score: Schema.u32,
    alert_count: Schema.u32,
    last_alert_at: Schema.u64,
  });



interface Trade {
  trade_id: string;
  symbol: string;
  account_id: string;
  trade_type: string;
  quantity: bigint;
  price: string;
  value: string;
  exchange: string;
  segment: string;
  timestamp: bigint;
  order_id: string;
}
const tradeSchema = 
  Schema.object({
    trade_id: Schema.string,
    symbol: Schema.string,
    account_id: Schema.string,
    trade_type: Schema.string,
    quantity: Schema.u64,
    price: Schema.string,
    value: Schema.string,
    exchange: Schema.string,
    segment: Schema.string,
    timestamp: Schema.u64,
    order_id: Schema.string,
  });



interface Entity {
  entity_id: string;
  entity_type: string;
  name: string;
  pan_number: string;
  registration_id: string;
}
const entitySchema = 
  Schema.object({
    entity_id: Schema.string,
    entity_type: Schema.string,
    name: Schema.string,
    pan_number: Schema.string,
    registration_id: Schema.string,
  });



interface Relationship {
  source_entity_id: string;
  target_entity_id: string;
  relationship_type: string;
  relationship_detail: string;
  strength: number;
  verified: boolean;
}
const relationshipSchema = 
  Schema.object({
    source_entity_id: Schema.string,
    target_entity_id: Schema.string,
    relationship_type: Schema.string,
    relationship_detail: Schema.string,
    strength: Schema.u32,
    verified: Schema.bool,
  });



interface InsiderStatus {
  entity_id: string;
  company_symbol: string;
  is_insider: boolean;
  insider_type: string;
  designation: string;
  window_status: string;
}
const insiderStatusSchema = 
  Schema.object({
    entity_id: Schema.string,
    company_symbol: Schema.string,
    is_insider: Schema.bool,
    insider_type: Schema.string,
    designation: Schema.string,
    window_status: Schema.string,
  });



interface ReportResult {
  report_id: string;
  report_type: string;
  storage_path: string;
  download_url: string;
  expires_at: bigint;
  risk_score: number;
  success: boolean;
  error: string;
}
const reportResultSchema = 
  Schema.object({
    report_id: Schema.string,
    report_type: Schema.string,
    storage_path: Schema.string,
    download_url: Schema.string,
    expires_at: Schema.u64,
    risk_score: Schema.u32,
    success: Schema.bool,
    error: Schema.string,
  });



interface UPSIRecord {
  upsi_id: string;
  company_symbol: string;
  upsi_type: string;
  description: string;
  nature: string;
  created_date: bigint;
  public_date: bigint;
  is_public: boolean;
}
const upsiRecordSchema = 
  Schema.object({
    upsi_id: Schema.string,
    company_symbol: Schema.string,
    upsi_type: Schema.string,
    description: Schema.string,
    nature: Schema.string,
    created_date: Schema.u64,
    public_date: Schema.u64,
    is_public: Schema.bool,
  });



interface TradingWindowStatus {
  company_symbol: string;
  window_status: string;
  closure_reason: string;
  closure_start: bigint;
  expected_opening: bigint;
}
const tradingWindowStatusSchema = 
  Schema.object({
    company_symbol: Schema.string,
    window_status: Schema.string,
    closure_reason: Schema.string,
    closure_start: Schema.u64,
    expected_opening: Schema.u64,
  });



interface TradeAnalysis {
  symbol: string;
  total_volume: bigint;
  avg_price: string;
  high_price: string;
  low_price: string;
  buy_volume: bigint;
  sell_volume: bigint;
  trade_count: number;
  concentration_ratio: string;
}
const tradeAnalysisSchema = 
  Schema.object({
    symbol: Schema.string,
    total_volume: Schema.u64,
    avg_price: Schema.string,
    high_price: Schema.string,
    low_price: Schema.string,
    buy_volume: Schema.u64,
    sell_volume: Schema.u64,
    trade_count: Schema.u32,
    concentration_ratio: Schema.string,
  });



export const DashboardWebserver = ((wallet: WeilWallet | WeilWalletConnection, contractAddress: string) => ({
  ping: async () => {

    const args = {}
    const result = await wallet.contracts.execute(
      contractAddress,
      "ping",
      args,
    );

    return parseExecutionResult(result, Schema.string);
  },
  push_alert: async (alert: Alert) => {

    const args = parseSchema(
      Schema.args({
        alert: alertSchema,
      }),
      { alert }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "push_alert",
      args,
    );

    return parseExecutionResult(result, Schema.string);
  },
  log_workflow_start: async (workflow_id: string, workflow_type: string, trigger: string, total_steps: number) => {

    const args = parseSchema(
      Schema.args({
        workflow_id: Schema.string,
        workflow_type: Schema.string,
        trigger: Schema.string,
        total_steps: Schema.u32,
      }),
      { workflow_id, workflow_type, trigger, total_steps }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "log_workflow_start",
      args,
    );

    return parseExecutionResult(result, Schema.string);
  },
  update_workflow_progress: async (workflow_id: string, steps_completed: number, status: string, result_summary: string) => {

    const args = parseSchema(
      Schema.args({
        workflow_id: Schema.string,
        steps_completed: Schema.u32,
        status: Schema.string,
        result_summary: Schema.string,
      }),
      { workflow_id, steps_completed, status, result_summary }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "update_workflow_progress",
      args,
    );

    return parseExecutionResult(result, Schema.string);
  },
  upsert_case: async (case_record: CaseRecord) => {

    const args = parseSchema(
      Schema.args({
        case_record: caseRecordSchema,
      }),
      { case_record }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "upsert_case",
      args,
    );

    return parseExecutionResult(result, Schema.string);
  },
  register_risk_entity: async (entity: RiskEntity) => {

    const args = parseSchema(
      Schema.args({
        entity: riskEntitySchema,
      }),
      { entity }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "register_risk_entity",
      args,
    );

    return parseExecutionResult(result, Schema.string);
  },
  get_live_alerts: async (severity_filter: Option<string>, limit: Option<number>) => {

    const args = parseSchema(
      Schema.args({
        severity_filter: Schema.option(Schema.string),
        limit: Schema.option(Schema.u32),
      }),
      { severity_filter, limit }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "get_live_alerts",
      args,
    );

    return parseExecutionResult(result, Schema.array(alertSchema));
  },
  get_workflow_history: async (workflow_type: Option<string>, limit: Option<number>) => {

    const args = parseSchema(
      Schema.args({
        workflow_type: Schema.option(Schema.string),
        limit: Schema.option(Schema.u32),
      }),
      { workflow_type, limit }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "get_workflow_history",
      args,
    );

    return parseExecutionResult(result, Schema.array(workflowExecutionSchema));
  },
  get_cases_by_status: async (status: Option<string>, limit: Option<number>) => {

    const args = parseSchema(
      Schema.args({
        status: Schema.option(Schema.string),
        limit: Schema.option(Schema.u32),
      }),
      { status, limit }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "get_cases_by_status",
      args,
    );

    return parseExecutionResult(result, Schema.array(caseRecordSchema));
  },
  get_stats: async () => {

    const args = {}
    const result = await wallet.contracts.execute(
      contractAddress,
      "get_stats",
      args,
    );

    return parseExecutionResult(result, surveillanceStatsSchema);
  },
  get_high_risk_entities: async (min_risk_score: Option<number>, limit: Option<number>) => {

    const args = parseSchema(
      Schema.args({
        min_risk_score: Schema.option(Schema.u32),
        limit: Schema.option(Schema.u32),
      }),
      { min_risk_score, limit }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "get_high_risk_entities",
      args,
    );

    return parseExecutionResult(result, Schema.array(riskEntitySchema));
  },
  get_case_details: async (case_id: string) => {

    const args = parseSchema(
      Schema.args({
        case_id: Schema.string,
      }),
      { case_id }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "get_case_details",
      args,
    );

    return parseExecutionResult(result, caseRecordSchema);
  },
  get_entity_alerts: async (entity_id: string, limit: Option<number>) => {

    const args = parseSchema(
      Schema.args({
        entity_id: Schema.string,
        limit: Schema.option(Schema.u32),
      }),
      { entity_id, limit }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "get_entity_alerts",
      args,
    );

    return parseExecutionResult(result, Schema.array(alertSchema));
  },
  get_trades_proxy: async (symbol: string, limit: Option<number>) => {

    const args = parseSchema(
      Schema.args({
        symbol: Schema.string,
        limit: Schema.option(Schema.u32),
      }),
      { symbol, limit }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "get_trades_proxy",
      args,
    );

    return parseExecutionResult(result, Schema.array(tradeSchema));
  },
  search_entities_proxy: async (search_query: string) => {

    const args = parseSchema(
      Schema.args({
        search_query: Schema.string,
      }),
      { search_query }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "search_entities_proxy",
      args,
    );

    return parseExecutionResult(result, Schema.array(entitySchema));
  },
  get_relationships_proxy: async (entity_id: string) => {

    const args = parseSchema(
      Schema.args({
        entity_id: Schema.string,
      }),
      { entity_id }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "get_relationships_proxy",
      args,
    );

    return parseExecutionResult(result, Schema.array(relationshipSchema));
  },
  check_insider_proxy: async (entity_id: string, company_symbol: string) => {

    const args = parseSchema(
      Schema.args({
        entity_id: Schema.string,
        company_symbol: Schema.string,
      }),
      { entity_id, company_symbol }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "check_insider_proxy",
      args,
    );

    return parseExecutionResult(result, insiderStatusSchema);
  },
  get_active_upsi_proxy: async (company_symbol: string) => {

    const args = parseSchema(
      Schema.args({
        company_symbol: Schema.string,
      }),
      { company_symbol }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "get_active_upsi_proxy",
      args,
    );

    return parseExecutionResult(result, Schema.array(upsiRecordSchema));
  },
  get_trading_window_proxy: async (company_symbol: string) => {

    const args = parseSchema(
      Schema.args({
        company_symbol: Schema.string,
      }),
      { company_symbol }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "get_trading_window_proxy",
      args,
    );

    return parseExecutionResult(result, tradingWindowStatusSchema);
  },
  analyze_volume_proxy: async (symbol: string) => {

    const args = parseSchema(
      Schema.args({
        symbol: Schema.string,
      }),
      { symbol }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "analyze_volume_proxy",
      args,
    );

    return parseExecutionResult(result, tradeAnalysisSchema);
  },
  generate_report_proxy: async (report_type: string, params: string) => {

    const args = parseSchema(
      Schema.args({
        report_type: Schema.string,
        params: Schema.string,
      }),
      { report_type, params }
    );

    const result = await wallet.contracts.execute(
      contractAddress,
      "generate_report_proxy",
      args,
    );

    return parseExecutionResult(result, reportResultSchema);
  },
  get_tools: async () => {

    const args = {}
    const result = await wallet.contracts.execute(
      contractAddress,
      "get_tools",
      args,
    );

    return parseExecutionResult(result, Schema.string);
  },
  get_prompts: async () => {

    const args = {}
    const result = await wallet.contracts.execute(
      contractAddress,
      "get_prompts",
      args,
    );

    return parseExecutionResult(result, Schema.string);
  },

} satisfies Contract)) satisfies ContractFactory;

