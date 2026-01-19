# DAEDALUS - Market surveillance and complaince platform

A comprehensive market surveillance platform built on **VeilChain** using the **Model Context Protocol (MCP)** architecture. This system enables real-time detection of market manipulation, insider trading, and regulatory compliance monitoring.
  
## MCPs Overview

| MCP | Contract ID | External APIs |
|-----|-------------|---------------|
| Dashboard WebServer | `aaaaaazeabi6qpbwteomycouqutujabnnjjiy5alq3hf4gxxec4e7akjli` | None |
| Trade Data | `aaaaaa7ycnpwxv2xks67l4lkczhwgbyawt34u7esvsbfgbnq7putxnvmfm` | Alpha Vantage |
| Entity Relationship | `aaaaaa36wpz26hhxrmyvo2clmpqqkwqudoqkymfkgltfmtvoceftcfes5u` | Neo4j Aura |
| Anomaly Detection | `aaaaaa4s4a4xuzmfsxs5gbs3hcm7npikux3jah52oq6rvnty63jw7qpoam` | TAAPI.io |
| UPSI Database | `` | Supabase |
| Regulatory Reports | `aaaaaa77wlohisawkkbxkgc77cb24slezi4a5yx6obqiyjcmwofwx3njfm` | Supabase Storage |
| Jira MCP | `aaaaaaz5hqfunhcpxhwzf5nplsgtxaydfgupn5u2at47tkz2igw7ipaya4` | Atlassian |
| Slack Notifier | `aaaaaa4l5izydl4rfdxv5vmxrbdtkqka5htfq55d2d6pchqcplqv5gjbma` | Slack API |

Warning: These contract ID's would most probably be invalid, here just for reference sake.


## Detailed Setup Guide

### 1. Dashboard WebServer

The central hub that aggregates data from all MCPs and serves the surveillance dashboard.

#### Directory Structure
```
dashboard_webserver/
├── src/
│   ├── lib.rs              # Main contract logic
│   ├── trade_data.rs       # Trade data proxy bindings
│   ├── entity_relationship.rs
│   ├── upsi_database.rs
│   └── regulatory_reports.rs
├── dashboard_webserver.widl # Interface definition
├── config.yaml             # Configuration
└── Cargo.toml
```


---

### 2. Trade Data MCP

Fetches real-time and historical trade data from Alpha Vantage API.

#### config.yaml
```yaml
dashboard_contract_id: "aaaaaazeabi6qpbwteomycouqutujabnnjjiy5alq3hf4gxxec4e7akjli"

# Get free API key at: https://www.alphavantage.co/support/#api-key
api_key_1: "YOUR_ALPHA_VANTAGE_API_KEY"
api_key_2: "BACKUP_API_KEY"  # Optional, for rate limit handling
```

#### API Setup: Alpha Vantage
1. Go to [Alpha Vantage](https://www.alphavantage.co/support/#api-key)
2. Sign up for free (5 API calls/min, 500/day)
3. Copy API key to config.yaml

#### Key Methods
| Method | Description |
|--------|-------------|
| `get_trades` | Fetch trades for a symbol |
| `get_intraday_data` | Get minute-by-minute data |
| `analyze_volume` | Volume analysis |
| `get_trade_history` | Historical trades |

#### Test Command
```bash
execute -n aaaaaa7ycnpwxv2xks67l4lkczhwgbyawt34u7esvsbfgbnq7putxnvmfm \
        -m get_intraday_data \
        -i '{"symbol":"AAPL","interval":"5min"}'
```

---

### 3. Entity Relationship MCP

Maps relationships between traders, companies, and accounts using Neo4j graph database.

#### config.yaml
```yaml
dashboard_contract_id: "aaaaaazeabi6qpbwteomycouqutujabnnjjiy5alq3hf4gxxec4e7akjli"

# Get Neo4j Aura (free tier): https://console.neo4j.io
neo4j_uri: "neo4j+s://xxxxxxxx.databases.neo4j.io"
neo4j_username: "neo4j"
neo4j_password: "YOUR_PASSWORD"
```

#### API Setup: Neo4j Aura
1. Go to [Neo4j Aura](https://console.neo4j.io)
2. Create free account → New Instance (Free tier)
3. Download credentials file
4. Copy URI, username, password to config.yaml

#### Sample Data for testing, truncated from the actual gloabal consistent data(Cypher)
```cypher
// Create entities
CREATE (e1:Entity {entity_id: 'ENT-0001', name: 'Rajesh Kumar', type: 'INDIVIDUAL'})
CREATE (e2:Entity {entity_id: 'ENT-0002', name: 'Kumar Holdings', type: 'COMPANY'})
CREATE (c1:Company {symbol: 'RELIANCE', name: 'Reliance Industries'})

// Create relationships
CREATE (e1)-[:DIRECTOR_OF {since: '2020-01-01'}]->(e2)
CREATE (e1)-[:INSIDER_OF {designation: 'CFO'}]->(c1)
CREATE (e2)-[:SHAREHOLDER_OF {percentage: 5.2}]->(c1)
```

#### Key Methods
| Method | Description |
|--------|-------------|
| `get_entity` | Get entity details |
| `get_relationships` | Get entity connections |
| `check_insider_status` | Check if entity is insider |
| `search_entities` | Search by name/ID |

---

### 4. Anomaly Detection MCP

Detects suspicious trading patterns using technical analysis.

#### config.yaml
```yaml
dashboard_contract_id: "aaaaaazeabi6qpbwteomycouqutujabnnjjiy5alq3hf4gxxec4e7akjli"

# Get API key at: https://taapi.io
alpha_vantage_key: "YOUR_ALPHA_VANTAGE_KEY"
taapi_secret: "YOUR_TAAPI_SECRET"
```

#### API Setup: TAAPI.io
1. Go to [TAAPI.io](https://taapi.io)
2. Sign up for free tier (10 requests/min)
3. Copy secret key to config.yaml

#### Detection Types
| Type | Description | Risk Score |
|------|-------------|------------|
| `WASH_TRADE` | Circular trading patterns | 70-90 |
| `SPOOFING` | Order manipulation | 65-85 |
| `FRONT_RUNNING` | Trading ahead of clients | 75-95 |
| `PUMP_AND_DUMP` | Price manipulation | 80-100 |
| `INSIDER_TRADING` | Pre-announcement trades | 85-100 |

#### Key Methods
| Method | Description |
|--------|-------------|
| `detect_anomalies` | Run detection algorithms |
| `analyze_pattern` | Pattern analysis |
| `get_risk_score` | Calculate risk score |

---

### 5. UPSI Database MCP

Tracks Unpublished Price Sensitive Information (UPSI) access and trading windows.

#### config.yaml
```yaml
dashboard_contract_id: "aaaaaazeabi6qpbwteomycouqutujabnnjjiy5alq3hf4gxxec4e7akjli"

# Supabase (free tier): https://supabase.com
supabase_url: "https://xxxxx.supabase.co"
supabase_anon_key: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

#### API Setup: Supabase
1. Go to [Supabase](https://supabase.com)
2. Create new project (free tier: 500MB, 2GB bandwidth)
3. Go to Settings → API → Copy URL and anon key

#### Database Schema for testing.
```sql
-- UPSI Records
CREATE TABLE upsi_records (
    upsi_id TEXT PRIMARY KEY,
    company_symbol TEXT NOT NULL,
    upsi_type TEXT NOT NULL,
    description TEXT,
    nature TEXT,
    created_date TIMESTAMP NOT NULL,
    public_date TIMESTAMP,
    is_public BOOLEAN DEFAULT FALSE
);

-- Access Log
CREATE TABLE upsi_access_log (
    id SERIAL PRIMARY KEY,
    upsi_id TEXT REFERENCES upsi_records(upsi_id),
    entity_id TEXT NOT NULL,
    access_type TEXT,
    accessed_at TIMESTAMP DEFAULT NOW()
);

-- Trading Windows
CREATE TABLE trading_windows (
    id SERIAL PRIMARY KEY,
    company_symbol TEXT NOT NULL,
    window_status TEXT NOT NULL,
    closure_reason TEXT,
    closure_start TIMESTAMP,
    expected_opening TIMESTAMP
);
```

#### Sample Data
```sql
INSERT INTO upsi_records VALUES 
('UPSI-2026-001', 'RELIANCE', 'MERGER', 'Acquisition of XYZ Corp', 'Material', NOW(), NULL, FALSE),
('UPSI-2026-002', 'TCS', 'FINANCIAL', 'Q4 earnings preview', 'Financial', NOW(), NULL, FALSE);

INSERT INTO trading_windows VALUES 
(DEFAULT, 'RELIANCE', 'CLOSED', 'Merger announcement pending', NOW(), NOW() + INTERVAL '30 days');
```

---

### 6. Regulatory Reports MCP

Generates compliance reports and stores them in Supabase Storage.

#### config.yaml
```yaml
dashboard_contract_id: "aaaaaazeabi6qpbwteomycouqutujabnnjjiy5alq3hf4gxxec4e7akjli"
jira_contract_id: "aaaaaaz5hqfunhcpxhwzf5nplsgtxaydfgupn5u2at47tkz2igw7ipaya4"

# Supabase Storage
supabase_url: "https://xxxxx.supabase.co"
supabase_anon_key: "YOUR_ANON_KEY"
supabase_service_key: "YOUR_SERVICE_KEY"  # For storage uploads
```

#### Storage Setup
1. In Supabase Dashboard → Storage
2. Create bucket: `regulatory-reports`
3. Set policy: Allow authenticated uploads

#### Report Types
| Type | Description |
|------|-------------|
| `STR` | Suspicious Transaction Report |
| `CTR` | Cash Transaction Report |
| `SAR` | Suspicious Activity Report |
| `COMPLIANCE` | General compliance report |

---

### 7. Jira MCP

Creates investigation tickets in Atlassian Jira.

#### config.yaml
```yaml
# Atlassian Jira Cloud
jira_base_url: "https://your-domain.atlassian.net"
jira_email: "your-email@example.com"
jira_api_token: "YOUR_API_TOKEN"
jira_project_key: "SURV"
```

#### API Setup: Atlassian Jira
1. Go to [Atlassian](https://id.atlassian.com/manage-profile/security/api-tokens)
2. Create API Token
3. Create a project in Jira with key "SURV"

#### Key Methods
| Method | Description |
|--------|-------------|
| `create_ticket` | Create investigation ticket |
| `update_ticket` | Update ticket status |
| `get_ticket` | Get ticket details |
| `add_comment` | Add investigation notes |

---

### 8. Slack Notifier MCP

Sends real-time alerts to Slack channels.

#### config.yaml
```yaml
# Slack Bot
slack_bot_token: "xoxb-XXXXXXXXXX-XXXXXXXXXX-XXXXXXXXXXXXXXXX"
slack_channel_alerts: "#surveillance-alerts"
slack_channel_cases: "#investigation-cases"
```

#### API Setup: Slack
1. Go to [Slack API](https://api.slack.com/apps)
2. Create New App → From scratch
3. OAuth & Permissions → Add scopes:
   - `chat:write`
   - `channels:read`
4. Install to Workspace → Copy Bot Token

---

## External API Setup

### Summary of Required APIs

| Service | Free Tier | Used By |
|---------|-----------|---------|
| [Alpha Vantage](https://www.alphavantage.co) | 5/min, 500/day | Trade Data |
| [Neo4j Aura](https://console.neo4j.io) | 1 instance, 50K nodes | Entity Relationship |
| [TAAPI.io](https://taapi.io) | 10/min | Anomaly Detection |
| [Supabase](https://supabase.com) | 500MB, 2GB/mo | UPSI, Reports |
| [Atlassian](https://atlassian.com) | 10 users free | Jira MCP |
| [Slack](https://slack.com) | Free workspace | Slack Notifier |

---

## Dashboard UI

The surveillance dashboard is a static web application.

### Local Development
```bash
cd surveillance_dashboard/dashboard_ui
npm install
npm run dev
```

### Production Build
```bash
npm run build
# Deploy dist/ folder to VeilChain static assets or any CDN
```

### Features
- **Live Feed**: Real-time trade streaming via WebSocket (Binance, free)
- **Alerts Panel**: View and filter surveillance alerts
- **Cases Panel**: Manage investigation cases
- **Entity Search**: Look up trader relationships
- **Trade Analysis**: Volume and pattern analysis

### WebSocket Integration
The dashboard connects to free WebSocket feeds:
- **Binance** (default): Crypto trades, no API key needed
- **Finnhub**: Stock trades, free API key
- **Polygon**: Stock trades, paid

---

## Testing & Verification

### Push Sample Data
```bash
# Push an alert
execute -n aaaaaazeabi6qpbwteomycouqutujabnnjjiy5alq3hf4gxxec4e7akjli \
        -m push_alert \
        -i '{"alert":{"id":"TEST-001","alert_type":"INSIDER_TRADING","severity":"HIGH","risk_score":85,"entity_id":"ENT-0001","symbol":"RELIANCE","description":"Test alert","workflow_id":"WF-001","timestamp":1737195000000}}'

# Create a case
execute -n aaaaaazeabi6qpbwteomycouqutujabnnjjiy5alq3hf4gxxec4e7akjli \
        -m upsert_case \
        -i '{"case_record":{"case_id":"CASE-001","case_type":"INSIDER_TRADING","status":"OPEN","priority":"HIGH","subject_entity":"Test Entity","symbol":"RELIANCE","risk_score":85,"assigned_to":"Analyst","created_at":1737100000000,"updated_at":1737195000000,"summary":"Test case"}}'

# Get stats
execute -n aaaaaazeabi6qpbwteomycouqutujabnnjjiy5alq3hf4gxxec4e7akjli \
        -m get_stats -i '{}'
```

---

## Workflows

See https://docs.google.com/document/d/1SYKcsst-kxTPCBkscEZU4qBiz1wlCxSkrfTLbmvPCQ4/edit?usp=sharing
for more details
---


