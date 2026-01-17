# Universal Demo Dataset Plan

## Objective
Create **universal demo data** where every entity/symbol works across **ALL 7 MCPs** for any flow combination.

---

## Core Entities (Must Exist in ALL MCPs)

| ID | Name | Type | Account | Symbols |
|----|------|------|---------|---------|
| **ENT-0001** | Rajesh Kumar | Individual | ACC-0001 | RELIANCE, TCS |
| **ENT-0002** | Priya Sharma | Individual | ACC-0002 | RELIANCE, INFY |
| **ENT-0003** | Amit Patel | Individual | ACC-0003 | HDFC, ICICI |
| **ENT-0004** | Reliance Industries | Company | - | RELIANCE |
| **ENT-0005** | Mukesh Ambani | Promoter | ACC-0005 | RELIANCE |

| Symbol | Company | Has UPSI | Has Trades | Has Anomalies |
|--------|---------|----------|------------|---------------|
| **RELIANCE** | Reliance Industries | ✓ | ✓ | ✓ |
| **TCS** | Tata Consultancy | ✓ | ✓ | ✓ |
| **INFY** | Infosys | ✓ | ✓ | ✓ |
| **HDFC** | HDFC Bank | ✓ | ✓ | ✓ |
| **ICICI** | ICICI Bank | ✓ | ✓ | ✓ |

---

## Cross-MCP Data Matrix

| Entity/Symbol | trade_data | anomaly | entity_rel | upsi | risk | jira | slack | reports |
|---------------|------------|---------|------------|------|------|------|-------|---------|
| ENT-0001 / RELIANCE | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| ENT-0002 / TCS | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| ENT-0003 / INFY | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| ENT-0004 / HDFC | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| ENT-0005 / ICICI | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

---

## Field Mappings Between MCPs

```
trade_data.account_id  ←→  entity_relationship.entity_id  (via ACC→ENT mapping)
trade_data.symbol      ←→  upsi_database.company_symbol
anomaly.entity_id      ←→  risk_scoring.entity_id
anomaly.symbol         ←→  jira.case_summary (extracted)
jira.case_id           ←→  regulatory_reports.case_id
jira.ticket_url        ←→  slack.message (embedded)
reports.storage_path   ←→  slack.message (embedded)
```

---

## Data Per MCP

### 1. Entity Relationship (Neo4j/Mock)
- 15 entities with rich relationships
- Insider status for all 5 symbols
- Family/employer/director/shareholder connections

---

## Neo4j Cypher Scripts (Run Each Block Separately)

> **Important:** Run these as 3 SEPARATE queries in Neo4j Aura Console. Don't run all at once.

### Step 1: Clear and Create Entities
```cypher
// Clear existing data first
MATCH (n) DETACH DELETE n;
```

### Step 2: Create All Entities
```cypher
// Individuals
CREATE (:Entity {entity_id: 'ENT-0001', entity_type: 'INDIVIDUAL', name: 'Rajesh Kumar', pan_number: 'ABCPK1234A', registration_id: ''})
CREATE (:Entity {entity_id: 'ENT-0002', entity_type: 'INDIVIDUAL', name: 'Priya Sharma', pan_number: 'DEFPS5678B', registration_id: ''})
CREATE (:Entity {entity_id: 'ENT-0003', entity_type: 'INDIVIDUAL', name: 'Amit Patel', pan_number: 'GHIAP9012C', registration_id: ''})
CREATE (:Entity {entity_id: 'ENT-0005', entity_type: 'INDIVIDUAL', name: 'Mukesh Ambani', pan_number: 'AABPM1234M', registration_id: ''})
CREATE (:Entity {entity_id: 'ENT-0006', entity_type: 'INDIVIDUAL', name: 'Meera Kumar', pan_number: 'JKLMK3456D', registration_id: ''})
CREATE (:Entity {entity_id: 'ENT-0008', entity_type: 'INDIVIDUAL', name: 'Vikram Sharma', pan_number: 'MNVPS7890E', registration_id: ''})
CREATE (:Entity {entity_id: 'ENT-0009', entity_type: 'INDIVIDUAL', name: 'Anita Patel', pan_number: 'PQRAP1234F', registration_id: ''})
CREATE (:Entity {entity_id: 'ENT-0010', entity_type: 'INDIVIDUAL', name: 'Suresh Mehta', pan_number: 'STUSM5678G', registration_id: ''})
CREATE (:Entity {entity_id: 'ENT-0011', entity_type: 'INDIVIDUAL', name: 'Nita Ambani', pan_number: 'UVWNA9012H', registration_id: ''})
// Companies
CREATE (:Entity {entity_id: 'ENT-0004', entity_type: 'COMPANY', name: 'Reliance Industries', pan_number: 'AAACR5678R', registration_id: 'INE002A01018'})
CREATE (:Entity {entity_id: 'ENT-0012', entity_type: 'COMPANY', name: 'Tata Consultancy Services', pan_number: 'AAACT1234T', registration_id: 'INE467B01029'})
CREATE (:Entity {entity_id: 'ENT-0013', entity_type: 'COMPANY', name: 'Infosys Limited', pan_number: 'AAACI5678I', registration_id: 'INE009A01021'})
CREATE (:Entity {entity_id: 'ENT-0014', entity_type: 'COMPANY', name: 'HDFC Bank', pan_number: 'AAACH9012H', registration_id: 'INE040A01034'})
CREATE (:Entity {entity_id: 'ENT-0015', entity_type: 'COMPANY', name: 'ICICI Bank', pan_number: 'AAACI3456I', registration_id: 'INE090A01021'})
// Funds & Brokers
CREATE (:Entity {entity_id: 'ENT-0007', entity_type: 'FUND', name: 'Blackrock India Fund', pan_number: 'AAABR7890B', registration_id: 'MF-INV-123'})
CREATE (:Entity {entity_id: 'ENT-0016', entity_type: 'BROKER', name: 'Zerodha Securities', pan_number: 'AAAZS1234Z', registration_id: 'INZ000031633'})
// Insider Status
CREATE (:InsiderStatus {entity_id: 'ENT-0005', company_symbol: 'RELIANCE', is_insider: true, insider_type: 'PROMOTER', designation: 'Chairman & MD', window_status: 'CLOSED'})
CREATE (:InsiderStatus {entity_id: 'ENT-0011', company_symbol: 'RELIANCE', is_insider: true, insider_type: 'PROMOTER', designation: 'Director (Promoter Family)', window_status: 'CLOSED'})
CREATE (:InsiderStatus {entity_id: 'ENT-0006', company_symbol: 'RELIANCE', is_insider: true, insider_type: 'DESIGNATED_PERSON', designation: 'Promoter Family Member', window_status: 'CLOSED'})
CREATE (:InsiderStatus {entity_id: 'ENT-0001', company_symbol: 'RELIANCE', is_insider: true, insider_type: 'DESIGNATED_PERSON', designation: 'Connected Person', window_status: 'CLOSED'})
CREATE (:InsiderStatus {entity_id: 'ENT-0010', company_symbol: 'TCS', is_insider: true, insider_type: 'DIRECTOR', designation: 'Independent Director', window_status: 'CLOSED'})
CREATE (:InsiderStatus {entity_id: 'ENT-0002', company_symbol: 'TCS', is_insider: true, insider_type: 'KMP', designation: 'Senior Engineer', window_status: 'CLOSED'})
CREATE (:InsiderStatus {entity_id: 'ENT-0008', company_symbol: 'INFY', is_insider: true, insider_type: 'KMP', designation: 'CFO', window_status: 'CLOSED'})
CREATE (:InsiderStatus {entity_id: 'ENT-0003', company_symbol: 'INFY', is_insider: true, insider_type: 'DESIGNATED_PERSON', designation: 'Consultant', window_status: 'CLOSED'})
CREATE (:InsiderStatus {entity_id: 'ENT-0009', company_symbol: 'HDFC', is_insider: true, insider_type: 'DIRECTOR', designation: 'Independent Director', window_status: 'OPEN'})
CREATE (:InsiderStatus {entity_id: 'ENT-0010', company_symbol: 'ICICI', is_insider: true, insider_type: 'SHAREHOLDER', designation: 'Major Shareholder', window_status: 'CLOSED'});
```

### Step 3: Create All Relationships
```cypher
// FAMILY
MATCH (a:Entity {entity_id: 'ENT-0001'}), (b:Entity {entity_id: 'ENT-0006'}) CREATE (a)-[:FAMILY {detail: 'Spouse', strength: 10}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0006'}), (b:Entity {entity_id: 'ENT-0005'}) CREATE (a)-[:FAMILY {detail: 'Niece of Promoter', strength: 8}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0005'}), (b:Entity {entity_id: 'ENT-0011'}) CREATE (a)-[:FAMILY {detail: 'Spouse', strength: 10}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0002'}), (b:Entity {entity_id: 'ENT-0008'}) CREATE (a)-[:FAMILY {detail: 'Sibling', strength: 9}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0003'}), (b:Entity {entity_id: 'ENT-0009'}) CREATE (a)-[:FAMILY {detail: 'Spouse', strength: 10}]->(b);
// DIRECTOR
MATCH (a:Entity {entity_id: 'ENT-0005'}), (b:Entity {entity_id: 'ENT-0004'}) CREATE (a)-[:DIRECTOR {detail: 'Chairman & MD', strength: 10}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0011'}), (b:Entity {entity_id: 'ENT-0004'}) CREATE (a)-[:DIRECTOR {detail: 'Non-Executive Director', strength: 8}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0010'}), (b:Entity {entity_id: 'ENT-0012'}) CREATE (a)-[:DIRECTOR {detail: 'Independent Director', strength: 7}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0008'}), (b:Entity {entity_id: 'ENT-0013'}) CREATE (a)-[:DIRECTOR {detail: 'CFO', strength: 9}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0009'}), (b:Entity {entity_id: 'ENT-0014'}) CREATE (a)-[:DIRECTOR {detail: 'Independent Director', strength: 6}]->(b);
// EMPLOYER
MATCH (a:Entity {entity_id: 'ENT-0002'}), (b:Entity {entity_id: 'ENT-0012'}) CREATE (a)-[:EMPLOYER {detail: 'Senior Engineer at TCS', strength: 7}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0003'}), (b:Entity {entity_id: 'ENT-0013'}) CREATE (a)-[:EMPLOYER {detail: 'Consultant at Infosys', strength: 5}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0001'}), (b:Entity {entity_id: 'ENT-0004'}) CREATE (a)-[:EMPLOYER {detail: 'Former employee', strength: 4}]->(b);
// SHAREHOLDER
MATCH (a:Entity {entity_id: 'ENT-0001'}), (b:Entity {entity_id: 'ENT-0004'}) CREATE (a)-[:SHAREHOLDER {detail: '0.01% stake', strength: 3}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0005'}), (b:Entity {entity_id: 'ENT-0004'}) CREATE (a)-[:SHAREHOLDER {detail: '40.12% Promoter', strength: 10}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0007'}), (b:Entity {entity_id: 'ENT-0004'}) CREATE (a)-[:SHAREHOLDER {detail: '2.5% FII', strength: 6}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0007'}), (b:Entity {entity_id: 'ENT-0012'}) CREATE (a)-[:SHAREHOLDER {detail: '1.8% FII', strength: 5}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0002'}), (b:Entity {entity_id: 'ENT-0012'}) CREATE (a)-[:SHAREHOLDER {detail: '0.001% ESOP', strength: 2}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0010'}), (b:Entity {entity_id: 'ENT-0014'}) CREATE (a)-[:SHAREHOLDER {detail: '0.5% stake', strength: 4}]->(b);
// BENEFICIAL_OWNER
MATCH (a:Entity {entity_id: 'ENT-0001'}), (b:Entity {entity_id: 'ENT-0007'}) CREATE (a)-[:BENEFICIAL_OWNER {detail: 'Hidden UBO', strength: 8}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0003'}), (b:Entity {entity_id: 'ENT-0016'}) CREATE (a)-[:BENEFICIAL_OWNER {detail: 'Partner in brokerage', strength: 7}]->(b);
// BUSINESS_PARTNER
MATCH (a:Entity {entity_id: 'ENT-0001'}), (b:Entity {entity_id: 'ENT-0002'}) CREATE (a)-[:BUSINESS_PARTNER {detail: 'Joint investment', strength: 6}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0002'}), (b:Entity {entity_id: 'ENT-0003'}) CREATE (a)-[:BUSINESS_PARTNER {detail: 'Co-founders', strength: 7}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0008'}), (b:Entity {entity_id: 'ENT-0010'}) CREATE (a)-[:BUSINESS_PARTNER {detail: 'Advisory board', strength: 5}]->(b);
// CLIENT (Broker)
MATCH (a:Entity {entity_id: 'ENT-0001'}), (b:Entity {entity_id: 'ENT-0016'}) CREATE (a)-[:CLIENT {detail: 'Since 2020', strength: 4}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0002'}), (b:Entity {entity_id: 'ENT-0016'}) CREATE (a)-[:CLIENT {detail: 'Since 2021', strength: 4}]->(b);
MATCH (a:Entity {entity_id: 'ENT-0003'}), (b:Entity {entity_id: 'ENT-0016'}) CREATE (a)-[:CLIENT {detail: 'Since 2019', strength: 4}]->(b);
```

### Step 4: Verify Counts
```cypher
MATCH (e:Entity) RETURN count(e) as entities;
// Should return 15
```

```cypher
MATCH ()-[r]->() RETURN count(r) as relationships;
// Should return ~27
```

---



## Execute Commands Script

```bash
#!/bin/bash
# Populate Surveillance Dashboard with demo data

DASHBOARD_ID="aaaaaa425yj6nyqxta4t7rctn6av6mm7hjvmf2vm2sxemiomdw4e4ggaga"

# === ALERTS ===
execute --name $DASHBOARD_ID --method push_alert --method-args '{"alert":{"id":"ALERT-0001","alert_type":"INSIDER","severity":"CRITICAL","risk_score":92,"entity_id":"ENT-0001","symbol":"RELIANCE","description":"Insider trading - heavy pre-announcement buying","workflow_id":"WF-0001","timestamp":1737225600}}'

execute --name $DASHBOARD_ID --method push_alert --method-args '{"alert":{"id":"ALERT-0002","alert_type":"VOLUME_SPIKE","severity":"HIGH","risk_score":85,"entity_id":"ENT-0001","symbol":"RELIANCE","description":"5x volume anomaly detected","workflow_id":"WF-0001","timestamp":1737225700}}'

execute --name $DASHBOARD_ID --method push_alert --method-args '{"alert":{"id":"ALERT-0003","alert_type":"WASH_TRADE","severity":"HIGH","risk_score":78,"entity_id":"ENT-0002","symbol":"TCS","description":"Circular trading pattern","workflow_id":"WF-0002","timestamp":1737225800}}'

execute --name $DASHBOARD_ID --method push_alert --method-args '{"alert":{"id":"ALERT-0004","alert_type":"SPOOFING","severity":"MEDIUM","risk_score":65,"entity_id":"ENT-0003","symbol":"INFY","description":"Large orders cancelled rapidly","workflow_id":"WF-0003","timestamp":1737225900}}'

execute --name $DASHBOARD_ID --method push_alert --method-args '{"alert":{"id":"ALERT-0005","alert_type":"PUMP_DUMP","severity":"HIGH","risk_score":80,"entity_id":"ENT-0004","symbol":"HDFC","description":"Price manipulation suspected","workflow_id":"WF-0004","timestamp":1737226000}}'

# === CASES ===
execute --name $DASHBOARD_ID --method upsert_case --method-args '{"case_record":{"case_id":"CASE-2026-0001","case_type":"INSIDER_TRADING","status":"INVESTIGATING","priority":"CRITICAL","subject_entity":"ENT-0001","symbol":"RELIANCE","risk_score":92,"assigned_to":"Anil Verma","created_at":1737225600,"updated_at":1737225600,"summary":"Rajesh Kumar insider trading investigation"}}'

execute --name $DASHBOARD_ID --method upsert_case --method-args '{"case_record":{"case_id":"CASE-2026-0002","case_type":"WASH_TRADING","status":"OPEN","priority":"HIGH","subject_entity":"ENT-0002","symbol":"TCS","risk_score":78,"assigned_to":"Priya Investigator","created_at":1737225800,"updated_at":1737225800,"summary":"Priya Sharma wash trading investigation"}}'

execute --name $DASHBOARD_ID --method upsert_case --method-args '{"case_record":{"case_id":"CASE-2026-0003","case_type":"SPOOFING","status":"OPEN","priority":"MEDIUM","subject_entity":"ENT-0003","symbol":"INFY","risk_score":65,"assigned_to":"Amit Analyst","created_at":1737225900,"updated_at":1737225900,"summary":"Amit Patel spoofing investigation"}}'

# === RISK ENTITIES ===
execute --name $DASHBOARD_ID --method register_risk_entity --method-args '{"entity":{"entity_id":"ENT-0001","entity_name":"Rajesh Kumar","risk_score":92,"alert_count":5,"last_alert_at":1737225700}}'

execute --name $DASHBOARD_ID --method register_risk_entity --method-args '{"entity":{"entity_id":"ENT-0002","entity_name":"Priya Sharma","risk_score":78,"alert_count":3,"last_alert_at":1737225800}}'

execute --name $DASHBOARD_ID --method register_risk_entity --method-args '{"entity":{"entity_id":"ENT-0003","entity_name":"Amit Patel","risk_score":65,"alert_count":2,"last_alert_at":1737225900}}'

# === WORKFLOWS ===
execute --name $DASHBOARD_ID --method log_workflow_start --method-args '{"workflow_id":"WF-0001","workflow_type":"INSIDER_DETECTION","trigger":"UPSI access + large trade","total_steps":5}'

execute --name $DASHBOARD_ID --method update_workflow_progress --method-args '{"workflow_id":"WF-0001","steps_completed":5,"status":"COMPLETED","result_summary":"Insider trading confirmed, case escalated"}'

execute --name $DASHBOARD_ID --method log_workflow_start --method-args '{"workflow_id":"WF-0002","workflow_type":"MANIPULATION_CHECK","trigger":"Volume anomaly","total_steps":6}'

execute --name $DASHBOARD_ID --method update_workflow_progress --method-args '{"workflow_id":"WF-0002","steps_completed":3,"status":"RUNNING","result_summary":"Analyzing trade patterns"}'

echo "✓ Demo data populated successfully!"
```

---

## Supabase SQL (for UPSI Database)

```sql
-- UPSI Records
INSERT INTO upsi_records (upsi_id, company_symbol, upsi_type, description, nature, created_date, public_date, is_public) VALUES
('UPSI-0001', 'RELIANCE', 'FINANCIAL_RESULTS', 'Q3 FY2026 Results - 45% profit growth', 'POSITIVE', 1736899200, 1737936000, true),
('UPSI-0002', 'TCS', 'DIVIDEND', 'Special dividend Rs67/share', 'POSITIVE', 1736985600, 0, false),
('UPSI-0003', 'INFY', 'FINANCIAL_RESULTS', 'Q3 FY2026 - Revenue miss', 'NEGATIVE', 1736726400, 0, false),
('UPSI-0004', 'HDFC', 'MERGER', 'HDFC-HDFC Bank merger ratio revised', 'POSITIVE', 1736812800, 1737849600, true),
('UPSI-0005', 'ICICI', 'RIGHTS_ISSUE', 'Rights issue 1:5 at Rs800', 'NEUTRAL', 1736640000, 0, false);

-- UPSI Access Logs
INSERT INTO upsi_access_logs (access_id, upsi_id, accessor_entity_id, accessor_name, accessor_designation, access_timestamp, access_reason, access_mode) VALUES
('ACCESS-0001', 'UPSI-0001', 'ENT-0001', 'Rajesh Kumar', 'Connected Person', 1737072000, 'Spouse shared', 'VIEW'),
('ACCESS-0002', 'UPSI-0001', 'ENT-0005', 'Mukesh Ambani', 'Chairman', 1736899200, 'Board Meeting', 'VIEW'),
('ACCESS-0003', 'UPSI-0002', 'ENT-0002', 'Priya Sharma', 'Employee', 1736985700, 'Project work', 'VIEW'),
('ACCESS-0004', 'UPSI-0003', 'ENT-0003', 'Amit Patel', 'Consultant', 1736726500, 'Due diligence', 'VIEW'),
('ACCESS-0005', 'UPSI-0004', 'ENT-0001', 'Rajesh Kumar', 'Investor', 1736812900, 'Investment decision', 'VIEW');

-- Trading Windows
INSERT INTO trading_windows (company_symbol, window_status, closure_reason, closure_start, expected_opening) VALUES
('RELIANCE', 'CLOSED', 'Q3 Results', 1737504000, 1737936000),
('TCS', 'CLOSED', 'Dividend', 1737590400, 1738022400),
('INFY', 'CLOSED', 'Q3 Results', 1737417600, 1737849600),
('HDFC', 'OPEN', '', 0, 0),
('ICICI', 'CLOSED', 'Rights Issue', 1737331200, 1737763200);
```

---

## Next Steps
1. Run the **Execute Commands Script** against your deployed Dashboard.
2. Run the **Supabase SQL** in your Supabase SQL Editor.
3. Run the **Cache Population Commands** below.

---

## Cache Population Commands (Execute These)

### Contract Addresses
| MCP | Contract Address |
|-----|-----------------|
| anomaly_detection | `aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe` |
| entity_relationship | `aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm` |
| jira | `aaaaaa22qx66rbpoh6ak2mmjfb5kc2ryjnoqzp33ujzi73u3aze5wkij7e` |
| trade_data | `aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy` |
| surveillance_dashboard | `aaaaaa425yj6nyqxta4t7rctn6av6mm7hjvmf2vm2sxemiomdw4e4ggaga` |
| slack_notifier | `aaaaaa32j2c5cu5yn5qnxwjvmfkr7jdu7kxackaxyrj7742sombuims4eq` |
| regulatory_reports | `aaaaaa7pnhwr7c7mh4poqhyw7cwqy436lffiao3be64djpa67ieuyglmru` |

---

### 1. Trade Data MCP (Populate Cache)
```bash
# Get trades for RELIANCE (populates symbol cache)
execute --name aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy --method get_trades_by_symbol --method-args '{"symbol":"RELIANCE","limit":10}'

# Get trades by account (populates account cache)
execute --name aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy --method get_trades_by_account --method-args '{"account_id":"ACC-0001","limit":10}'

# Analyze volume for TCS
execute --name aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy --method analyze_volume --method-args '{"symbol":"TCS"}'

# Get context (check cache)
execute --name aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy --method get_context --method-args '{}'
```

---

### 2. Anomaly Detection MCP (Populate Cache)
```bash
# Detect spoofing for ENT-0001 on RELIANCE
execute --name aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe --method detect_spoofing --method-args '{"order_id":"ORD-0001","entity_id":"ENT-0001","symbol":"RELIANCE","order_details":"Large order cancelled"}'

# Detect pump & dump for TCS
execute --name aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe --method detect_pump_dump --method-args '{"symbol":"TCS","time_window_minutes":60}'

# Check RSI for INFY
execute --name aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe --method check_rsi_levels --method-args '{"symbol":"INFY"}'

# Analyze volume anomaly for HDFC
execute --name aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe --method analyze_volume_anomaly --method-args '{"symbol":"HDFC","interval":"15min"}'

# Get context
execute --name aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe --method get_context --method-args '{}'
```

---

### 3. Entity Relationship MCP (Populate Cache)
```bash
# Get entity details for ENT-0001
execute --name aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm --method get_entity --method-args '{"entity_id":"ENT-0001"}'

# Get relationships for ENT-0001
execute --name aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm --method get_relationships --method-args '{"entity_id":"ENT-0001"}'

# Check insider status
execute --name aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm --method check_insider_status --method-args '{"entity_id":"ENT-0001","company_symbol":"RELIANCE"}'

# Get company insiders for TCS
execute --name aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm --method get_company_insiders --method-args '{"company_symbol":"TCS"}'

# Get connected entities
execute --name aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm --method get_connected_entities --method-args '{"entity_id":"ENT-0005","max_hops":3}'

# Get context
execute --name aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm --method get_context --method-args '{}'
```

---

### 4. Jira MCP (Populate Cache)
```bash
# Create case ticket for ENT-0001
execute --name aaaaaa22qx66rbpoh6ak2mmjfb5kc2ryjnoqzp33ujzi73u3aze5wkij7e --method create_case_ticket --method-args '{"case_id":"CASE-2026-0001","subject_entity":"ENT-0001","case_summary":"Insider trading investigation - Rajesh Kumar RELIANCE","priority":"High"}'

# Create case ticket for ENT-0002
execute --name aaaaaa22qx66rbpoh6ak2mmjfb5kc2ryjnoqzp33ujzi73u3aze5wkij7e --method create_case_ticket --method-args '{"case_id":"CASE-2026-0002","subject_entity":"ENT-0002","case_summary":"Wash trading investigation - Priya Sharma TCS","priority":"Medium"}'

# Create case ticket for ENT-0003
execute --name aaaaaa22qx66rbpoh6ak2mmjfb5kc2ryjnoqzp33ujzi73u3aze5wkij7e --method create_case_ticket --method-args '{"case_id":"CASE-2026-0003","subject_entity":"ENT-0003","case_summary":"Spoofing investigation - Amit Patel INFY","priority":"Medium"}'
```

---

### 5. Surveillance Dashboard (Populate Cache)
```bash
# Push alerts
execute --name aaaaaa425yj6nyqxta4t7rctn6av6mm7hjvmf2vm2sxemiomdw4e4ggaga --method push_alert --method-args '{"alert":{"id":"ALERT-0001","alert_type":"INSIDER","severity":"CRITICAL","risk_score":92,"entity_id":"ENT-0001","symbol":"RELIANCE","description":"Insider trading suspected","workflow_id":"WF-0001","timestamp":1737225600}}'

execute --name aaaaaa425yj6nyqxta4t7rctn6av6mm7hjvmf2vm2sxemiomdw4e4ggaga --method push_alert --method-args '{"alert":{"id":"ALERT-0002","alert_type":"WASH_TRADE","severity":"HIGH","risk_score":78,"entity_id":"ENT-0002","symbol":"TCS","description":"Circular trading pattern","workflow_id":"WF-0002","timestamp":1737225800}}'

execute --name aaaaaa425yj6nyqxta4t7rctn6av6mm7hjvmf2vm2sxemiomdw4e4ggaga --method push_alert --method-args '{"alert":{"id":"ALERT-0003","alert_type":"SPOOFING","severity":"MEDIUM","risk_score":65,"entity_id":"ENT-0003","symbol":"INFY","description":"Order cancellation pattern","workflow_id":"WF-0003","timestamp":1737225900}}'

# Register risk entities
execute --name aaaaaa425yj6nyqxta4t7rctn6av6mm7hjvmf2vm2sxemiomdw4e4ggaga --method register_risk_entity --method-args '{"entity":{"entity_id":"ENT-0001","entity_name":"Rajesh Kumar","risk_score":92,"alert_count":5,"last_alert_at":1737225700}}'

execute --name aaaaaa425yj6nyqxta4t7rctn6av6mm7hjvmf2vm2sxemiomdw4e4ggaga --method register_risk_entity --method-args '{"entity":{"entity_id":"ENT-0002","entity_name":"Priya Sharma","risk_score":78,"alert_count":3,"last_alert_at":1737225800}}'

# Log workflows
execute --name aaaaaa425yj6nyqxta4t7rctn6av6mm7hjvmf2vm2sxemiomdw4e4ggaga --method log_workflow_start --method-args '{"workflow_id":"WF-0001","workflow_type":"INSIDER_DETECTION","trigger":"UPSI access","total_steps":5}'

execute --name aaaaaa425yj6nyqxta4t7rctn6av6mm7hjvmf2vm2sxemiomdw4e4ggaga --method update_workflow_progress --method-args '{"workflow_id":"WF-0001","steps_completed":5,"status":"COMPLETED","result_summary":"Confirmed insider trading"}'
```

---

### 6. Regulatory Reports MCP (Populate Cache)
```bash
# Get context first
execute --name aaaaaa7pnhwr7c7mh4poqhyw7cwqy436lffiao3be64djpa67ieuyglmru --method get_context --method-args '{}'

# Generate surveillance report
execute --name aaaaaa7pnhwr7c7mh4poqhyw7cwqy436lffiao3be64djpa67ieuyglmru --method generate_surveillance_report --method-args '{"from_date":"2026-01-10","to_date":"2026-01-14","report_type":"DAILY"}'

# Generate STR for ENT-0001
execute --name aaaaaa7pnhwr7c7mh4poqhyw7cwqy436lffiao3be64djpa67ieuyglmru --method generate_str --method-args '{"case_id":"CASE-2026-0001","entity_id":"ENT-0001","suspicious_activity_type":"INSIDER_TRADING","suspicion_reason":"Pre-announcement trading with UPSI access"}'
```

---

### 7. Slack Notifier (Send Test Notifications)
```bash
# Send alert notification
execute --name aaaaaa32j2c5cu5yn5qnxwjvmfkr7jdu7kxackaxyrj7742sombuims4eq --method send_alert --method-args '{"alert_type":"INSIDER","severity":"CRITICAL","symbol":"RELIANCE","entity_id":"ENT-0001","description":"Insider trading detected","risk_score":92}'

# Send case update
execute --name aaaaaa32j2c5cu5yn5qnxwjvmfkr7jdu7kxackaxyrj7742sombuims4eq --method send_case_update --method-args '{"case_id":"CASE-2026-0001","status":"INVESTIGATING","update_message":"Evidence collected, escalating to SEBI","assigned_to":"Anil Verma"}'

# Send workflow complete
execute --name aaaaaa32j2c5cu5yn5qnxwjvmfkr7jdu7kxackaxyrj7742sombuims4eq --method send_workflow_complete --method-args '{"workflow_id":"WF-0001","workflow_type":"INSIDER_DETECTION","result_summary":"Confirmed insider trading by ENT-0001","alert_count":3}'
```


