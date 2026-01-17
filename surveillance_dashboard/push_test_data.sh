#!/bin/bash
# Push Test Data to Surveillance Dashboard
# ==========================================
# This script pushes sample alerts, cases, and workflows to the dashboard
# using the WeilChain CLI.

DASHBOARD_CONTRACT="aaaaaa3bk6th2b5sxpylnhak27b7mpa2pv6uqgj3uox7cshmw5kbry7sae"

echo "=== Pushing Test Data to Surveillance Dashboard ==="
echo "Contract: $DASHBOARD_CONTRACT"
echo ""

# Push Sample Alerts
echo "1. Pushing sample alerts..."

# Alert 1: Critical Insider Trading
execute -n $DASHBOARD_CONTRACT -m push_alert -i '{"alert":{"id":"ALERT-TEST-001","alert_type":"INSIDER","severity":"CRITICAL","risk_score":92,"entity_id":"ENT-0001","symbol":"RELIANCE","description":"Pre-announcement heavy buying detected - 3.5x normal volume","workflow_id":"WF-2026-001","timestamp":1736870400}}'

# Alert 2: High Wash Trade
execute -n $DASHBOARD_CONTRACT -m push_alert -i '{"alert":{"id":"ALERT-TEST-002","alert_type":"WASH_TRADE","severity":"HIGH","risk_score":78,"entity_id":"ENT-0002","symbol":"TCS","description":"Circular trading pattern detected between related accounts","workflow_id":"WF-2026-002","timestamp":1736870500}}'

# Alert 3: Medium Spoofing
execute -n $DASHBOARD_CONTRACT -m push_alert -i '{"alert":{"id":"ALERT-TEST-003","alert_type":"SPOOFING","severity":"MEDIUM","risk_score":65,"entity_id":"ENT-0003","symbol":"INFY","description":"Order cancellation pattern detected - 85% cancel rate","workflow_id":"","timestamp":1736870600}}'

echo ""
echo "2. Pushing sample cases..."

# Case 1: Investigating
execute -n $DASHBOARD_CONTRACT -m upsert_case -i '{"case_record":{"case_id":"CASE-2026-001","case_type":"INSIDER_TRADING","status":"INVESTIGATING","priority":"CRITICAL","subject_entity":"ENT-0001","symbol":"RELIANCE","risk_score":92,"assigned_to":"Anil Verma","created_at":1736870400,"updated_at":1736870700,"summary":"Insider trading investigation - Pre-Q3 results trading by connected entity"}}'

# Case 2: Open
execute -n $DASHBOARD_CONTRACT -m upsert_case -i '{"case_record":{"case_id":"CASE-2026-002","case_type":"WASH_TRADING","status":"OPEN","priority":"HIGH","subject_entity":"ENT-0002","symbol":"TCS","risk_score":78,"assigned_to":"","created_at":1736870500,"updated_at":1736870500,"summary":"Potential wash trading between family accounts"}}'

echo ""
echo "3. Pushing sample workflow..."

execute -n $DASHBOARD_CONTRACT -m log_workflow_start -i '{"workflow_id":"WF-2026-001","workflow_type":"INSIDER_DETECTION","trigger":"Volume anomaly on RELIANCE","total_steps":5}'

execute -n $DASHBOARD_CONTRACT -m update_workflow_progress -i '{"workflow_id":"WF-2026-001","steps_completed":5,"status":"COMPLETED","result_summary":"Identified ENT-0001 as connected entity with UPSI access. Risk score: 92"}'

echo ""
echo "4. Pushing sample high-risk entity..."

execute -n $DASHBOARD_CONTRACT -m register_risk_entity -i '{"entity":{"entity_id":"ENT-0001","entity_name":"Rajesh Kumar","risk_score":92,"alert_count":3,"last_alert_at":1736870700}}'

echo ""
echo "=== Test Data Push Complete ==="
echo "Now refresh your dashboard to see the data!"
