1. Get UPSI Record by ID
"Get details of UPSI-0001"
"What is UPSI-0001 about?"
"Show me UPSI record UPSI-0003"
2. Get Active UPSI for Company
"What active UPSI exists for TCS?"
"Show non-public UPSI for INFY"
"Are there any unpublished price sensitive information for ICICI?"
3. Get UPSI Access Logs
"Who accessed UPSI-0001?"
"Show access logs for UPSI-0002"
"List all people who viewed UPSI-0003"
4. Get Access by Person
"What UPSI has Rajesh Kumar (ENT-0001) accessed?"
"Show all UPSI access by ENT-0005 in the last 30 days"
"Did Priya Sharma access any sensitive information recently?"
5. Check UPSI Access Before Date
"Did ENT-0001 access RELIANCE UPSI before January 15th?"
"Check if Rajesh Kumar had UPSI access for RELIANCE before the announcement"
"Was ENT-0003 aware of INFY results before they went public?"
6. Get Trading Window Status
"Is the trading window open for RELIANCE?"
"What is the trading window status for TCS?"
"When does HDFC trading window open?"
7. Check Window Violation
"Did ENT-0001 trade during closed window for RELIANCE?"
"Check if Rajesh Kumar violated trading window rules"
"Was there any trading by ENT-0002 during TCS closure period?"
Multi-Step Investigation Flow Example
1. "Show active UPSI for RELIANCE"
2. "Who accessed this UPSI?"  
3. "Did ENT-0001 trade during the closed window?"
4. "Create a Jira ticket for this investigation"
5. "Generate an STR report"

{
  "recent_queries": [
    {
      "method_name": "generate_entity_risk_report",
      "entity_id": "SUS-001",
      "company_symbol": "",
      "case_id": "",
      "report_id": "RISK-2026-0011",
      "timestamp": 4,
      "natural_language_prompt": "Generated risk report for SUS-001"
    },
    {
      "method_name": "generate_str",
      "entity_id": "SUS-001",
      "company_symbol": "",
      "case_id": "CASE-001",
      "report_id": "STR-2026-0012",
      "timestamp": 5,
      "natural_language_prompt": "Generated STR for SUS-001 in case CASE-001"
    },
    {
      "method_name": "generate_surveillance_report",
      "entity_id": "",
      "company_symbol": "",
      "case_id": "",
      "report_id": "SURV-2026-0013",
      "timestamp": 6,
      "natural_language_prompt": "Generated WEEKLY surveillance report"
    },
    {
      "method_name": "generate_entity_risk_report",
      "entity_id": "SUS-001",
      "company_symbol": "",
      "case_id": "",
      "report_id": "RISK-2026-0014",
      "timestamp": 7,
      "natural_language_prompt": "Generated risk report for SUS-001"
    },
    {
      "method_name": "generate_investigation_report",
      "entity_id": "",
      "company_symbol": "",
      "case_id": "CASE-001",
      "report_id": "INV-2026-0015",
      "timestamp": 8,
      "natural_language_prompt": "Generated investigation report for case CASE-001"
    },
    {
      "method_name": "generate_compliance_scorecard",
      "entity_id": "SUS-001",
      "company_symbol": "",
      "case_id": "",
      "report_id": "COMP-2026-0016",
      "timestamp": 9,
      "natural_language_prompt": "Generated compliance scorecard for SUS-001"
    },
    {
      "method_name": "generate_compliance_scorecard",
      "entity_id": "SUS-001",
      "company_symbol": "",
      "case_id": "",
      "report_id": "COMP-2026-0017",
      "timestamp": 10,
      "natural_language_prompt": "Generated compliance scorecard for SUS-001"
    },
    {
      "method_name": "generate_esm_report",
      "entity_id": "",
      "company_symbol": "",
      "case_id": "",
      "report_id": "ESM-2026-0018",
      "timestamp": 11,
      "natural_language_prompt": "Generated ESM report for 2026-01-14"
    },
    {
      "method_name": "get_report_url",
      "entity_id": "",
      "company_symbol": "",
      "case_id": "",
      "report_id": "RISK-2026-0001",
      "timestamp": 11,
      "natural_language_prompt": "Retrieved URL for RISK-2026-0001"
    },
    {
      "method_name": "generate_surveillance_report",
      "entity_id": "",
      "company_symbol": "",
      "case_id": "",
      "report_id": "SURV-2026-0019",
      "timestamp": 11,
      "natural_language_prompt": "Generated DAILY surveillance report"
    }
  ],
  "last_entity_id": "SUS-001",
  "last_company_symbol": "RELIANCE",
  "last_case_id": "CASE-001",
  "last_report_id": "SURV-2026-0019"
}
