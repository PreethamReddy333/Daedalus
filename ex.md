# MCP Test Commands Reference

This document contains execute commands and natural language prompts for all MCP methods.

---

## Contract IDs Quick Reference

| MCP | Contract ID |
|-----|-------------|
| Anomaly Detection | `aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe` |
| Entity Relationship | `aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm` |
| Trade Data | `aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy` |
| UPSI Database | `aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4` |
| Jira MCP | `aaaaaa22qx66rbpoh6ak2mmjfb5kc2ryjnoqzp33ujzi73u3aze5wkij7e` |

---

## 1. Anomaly Detection MCP

**Contract ID:** `aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe`

### get_context
**Natural Language:** "Show me what entities and symbols were recently queried"
```bash
execute -n aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe -m get_context
```

### detect_spoofing
**Natural Language:** "Check if order ORD123 from trader ENT-REL-001 on RELIANCE is spoofing"
```bash
execute -n aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe -m detect_spoofing -i '{"order_id":"ORD123","entity_id":"ENT-REL-001","symbol":"RELIANCE","order_details":"Large order placed and cancelled"}'
```

### detect_wash_trading
**Natural Language:** "Check if ENT-REL-001 and ACC017 are doing wash trades on IBM"
```bash
execute -n aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe -m detect_wash_trading -i '{"entity_id":"ENT-REL-001","counterparty_id":"ACC017","symbol":"IBM","trade_timestamp":1737225600000}'
```

### detect_pump_dump
**Natural Language:** "Is there a pump and dump happening on AAPL in the last 60 minutes?"
```bash
execute -n aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe -m detect_pump_dump -i '{"symbol":"AAPL","time_window_minutes":60}'
```

### detect_front_running
**Natural Language:** "Check if broker BRK-001 front-ran a client trade on MSFT"
```bash
execute -n aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe -m detect_front_running -i '{"entity_id":"BRK-001","symbol":"MSFT","client_trade_timestamp":1737225600000,"prop_trade_timestamp":1737225590000}'
```

### analyze_volume_anomaly
**Natural Language:** "Analyze volume anomalies for IBM on 15min intervals"
```bash
execute -n aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe -m analyze_volume_anomaly -i '{"symbol":"IBM","interval":"15min"}'
```

### check_rsi_levels
**Natural Language:** "What are the RSI levels for GOOGL?"
```bash
execute -n aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe -m check_rsi_levels -i '{"symbol":"GOOGL"}'
```

### scan_entity_anomalies
**Natural Language:** "Run a full anomaly scan on suspect SUS-001"
```bash
execute -n aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe -m scan_entity_anomalies -i '{"entity_id":"SUS-001"}'
```

### tools
**Natural Language:** "What methods are available in anomaly detection?"
```bash
execute -n aaaaaay3luxuojcss2b7lptxckmhydqkatcinoiogzuep66itps6ntv7pe -m tools
```

---

## 2. Entity Relationship MCP

**Contract ID:** `aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm`

### get_context
**Natural Language:** "Show me the recently queried entities and companies"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m get_context
```

### get_entity
**Natural Language:** "Get details for Mukesh Ambani (ENT-REL-001)"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m get_entity -i '{"entity_id":"ENT-REL-001"}'
```

**Natural Language (Fuzzy):** "Get details for entity REL-001"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m get_entity -i '{"entity_id":"REL-001"}'
```

### search_entities
**Natural Language:** "Search for entities with name containing Ambani"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m search_entities -i '{"search_query":"Ambani","limit":5}'
```

**Natural Language:** "Find entities by PAN AABPA1234A"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m search_entities -i '{"search_query":"AABPA1234A","limit":5}'
```

### get_relationships
**Natural Language:** "What are Mukesh Ambani's relationships?"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m get_relationships -i '{"entity_id":"ENT-REL-001"}'
```

### get_connected_entities
**Natural Language:** "Find all entities within 2 hops of Reliance CFO"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m get_connected_entities -i '{"entity_id":"ENT-REL-006","max_hops":2}'
```

### check_insider_status
**Natural Language:** "Is Mukesh Ambani an insider of RELIANCE?"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m check_insider_status -i '{"entity_id":"ENT-REL-001","company_symbol":"RELIANCE"}'
```

**Natural Language:** "Is suspect SUS-001 connected to RELIANCE?"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m check_insider_status -i '{"entity_id":"SUS-001","company_symbol":"RELIANCE"}'
```

### get_company_insiders
**Natural Language:** "List all insiders of RELIANCE"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m get_company_insiders -i '{"company_symbol":"RELIANCE"}'
```

**Natural Language (Fuzzy):** "Get INFY insiders"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m get_company_insiders -i '{"company_symbol":"INF"}'
```

### are_entities_connected
**Natural Language:** "Is suspect SUS-001 connected to Reliance CFO?"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m are_entities_connected -i '{"entity_id_1":"SUS-001","entity_id_2":"ENT-REL-006","max_hops":3}'
```

### get_family_members
**Natural Language:** "Who are Mukesh Ambani's family members?"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m get_family_members -i '{"entity_id":"ENT-REL-001"}'
```

### tools
**Natural Language:** "What methods are available in entity relationship?"
```bash
execute -n aaaaaa5nb5t5cgg7nrintzf56dpmb3otezf3hycr47mbwocei5zwm5h4gm -m tools
```

---

## 3. Trade Data MCP

**Contract ID:** `aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy`

### get_context
**Natural Language:** "Show me recently queried symbols and accounts"
```bash
execute -n aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy -m get_context
```

### get_trade
**Natural Language:** "Get trade details for trade ID IBM_1737225600000_ACC017"
```bash
execute -n aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy -m get_trade -i '{"trade_id":"IBM_1737225600000_ACC017"}'
```

### get_trades_by_symbol
**Natural Language:** "Get the last 10 trades for IBM"
```bash
execute -n aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy -m get_trades_by_symbol -i '{"symbol":"IBM","limit":10}'
```

**Natural Language (Fuzzy):** "Show me Apple trades"
```bash
execute -n aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy -m get_trades_by_symbol -i '{"symbol":"AAP","limit":5}'
```

### get_trades_by_account
**Natural Language:** "Get all trades by account ACC017"
```bash
execute -n aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy -m get_trades_by_account -i '{"account_id":"ACC017","limit":10}'
```

### get_trades_by_accounts
**Natural Language:** "Get IBM trades for accounts ACC001, ACC002, ACC003"
```bash
execute -n aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy -m get_trades_by_accounts -i '{"account_ids":"ACC001,ACC002,ACC003","symbol":"IBM"}'
```

### analyze_volume
**Natural Language:** "Analyze trading volume for AAPL"
```bash
execute -n aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy -m analyze_volume -i '{"symbol":"AAPL"}'
```

### detect_volume_anomaly
**Natural Language:** "Are there any volume anomalies on MSFT?"
```bash
execute -n aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy -m detect_volume_anomaly -i '{"symbol":"MSFT"}'
```

### get_top_traders
**Natural Language:** "Who are the top 5 traders on GOOGL?"
```bash
execute -n aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy -m get_top_traders -i '{"symbol":"GOOGL","limit":5}'
```

### get_large_orders
**Natural Language:** "Find large orders over $1,000,000"
```bash
execute -n aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy -m get_large_orders -i '{"min_value":1000000}'
```

### get_account_profile
**Natural Language:** "Show me the trading profile for account ACC042"
```bash
execute -n aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy -m get_account_profile -i '{"account_id":"ACC042"}'
```

### tools
**Natural Language:** "What methods are available in trade data?"
```bash
execute -n aaaaaa7bmmtxibdkvxut2rpyuplk57gadst7qlz5hkl7ugalw6co4lsryy -m tools
```

---

## 4. UPSI Database MCP

**Contract ID:** `aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4`

### get_context
**Natural Language:** "Show me recent UPSI queries"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m get_context
```

### get_upsi
**Natural Language:** "Get UPSI record UPSI-001"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m get_upsi -i '{"upsi_id":"UPSI-001"}'
```

**Natural Language (Fuzzy):** "Get UPSI for 001"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m get_upsi -i '{"upsi_id":"001"}'
```

### get_active_upsi
**Natural Language:** "What active UPSI exists for RELIANCE?"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m get_active_upsi -i '{"company_symbol":"RELIANCE"}'
```

**Natural Language:** "Show active UPSI for Infosys"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m get_active_upsi -i '{"company_symbol":"INFY"}'
```

### get_upsi_access_log
**Natural Language:** "Who accessed UPSI-001 in the last month?"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m get_upsi_access_log -i '{"upsi_id":"UPSI-001","from_timestamp":1704067200000,"to_timestamp":1737225600000}'
```

### get_access_by_person
**Natural Language:** "What UPSI did Mukesh Ambani access in the last 30 days?"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m get_access_by_person -i '{"accessor_entity_id":"ENT-REL-001","days_back":30}'
```

**Natural Language:** "What UPSI did Reliance CFO access?"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m get_access_by_person -i '{"accessor_entity_id":"ENT-REL-006","days_back":30}'
```

### check_upsi_access_before
**Natural Language:** "Did suspect SUS-001 access RELIANCE UPSI before trading?"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m check_upsi_access_before -i '{"entity_id":"SUS-001","company_symbol":"RELIANCE","before_timestamp":1737225600000}'
```

### get_trading_window
**Natural Language:** "Is RELIANCE trading window open or closed?"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m get_trading_window -i '{"company_symbol":"RELIANCE"}'
```

**Natural Language:** "Check TCS trading window status"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m get_trading_window -i '{"company_symbol":"TCS"}'
```

### check_window_violation
**Natural Language:** "Did Mukesh Ambani trade RELIANCE during a closed window?"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m check_window_violation -i '{"entity_id":"ENT-REL-001","company_symbol":"RELIANCE","trade_timestamp":1736500000000}'
```

### get_upsi_accessors
**Natural Language:** "Who has accessed UPSI-001?"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m get_upsi_accessors -i '{"upsi_id":"UPSI-001"}'
```

### tools
**Natural Language:** "What methods are available in UPSI database?"
```bash
execute -n aaaaaaytejhz4sy6rhqobwg2sqvloy5276ouvo6ejod2fzt4rtobggxtf4 -m tools
```

---

## 5. Jira MCP

**Contract ID:** `aaaaaa22qx66rbpoh6ak2mmjfb5kc2ryjnoqzp33ujzi73u3aze5wkij7e`

### create_ticket
**Natural Language:** "Create a high priority Jira ticket for investigating suspicious trading"
```bash
execute -n aaaaaa22qx66rbpoh6ak2mmjfb5kc2ryjnoqzp33ujzi73u3aze5wkij7e -m create_ticket -i '{"summary":"Suspicious trading detected for ENT-REL-001","description":"Volume anomaly detected on RELIANCE. Requires investigation.","priority":"High","issue_type":"Task"}'
```

### create_case_ticket
**Natural Language:** "Create a surveillance case ticket for case CASE-001 investigating Mukesh Ambani"
```bash
execute -n aaaaaa22qx66rbpoh6ak2mmjfb5kc2ryjnoqzp33ujzi73u3aze5wkij7e -m create_case_ticket -i '{"case_id":"CASE-001","subject_entity":"ENT-REL-001","case_summary":"Potential insider trading on RELIANCE stock","priority":"High"}'
```

### close_ticket
**Natural Language:** "Close ticket SURV-123 as Done"
```bash
execute -n aaaaaa22qx66rbpoh6ak2mmjfb5kc2ryjnoqzp33ujzi73u3aze5wkij7e -m close_ticket -i '{"ticket_key":"SURV-123","resolution":"Done"}'
```

### get_ticket
**Natural Language:** "Get details for Jira ticket SURV-123"
```bash
execute -n aaaaaa22qx66rbpoh6ak2mmjfb5kc2ryjnoqzp33ujzi73u3aze5wkij7e -m get_ticket -i '{"ticket_key":"SURV-123"}'
```

### add_comment
**Natural Language:** "Add a comment to ticket SURV-123 about the investigation progress"
```bash
execute -n aaaaaa22qx66rbpoh6ak2mmjfb5kc2ryjnoqzp33ujzi73u3aze5wkij7e -m add_comment -i '{"ticket_key":"SURV-123","comment":"Investigation in progress. Confirming entity relationships."}'
```

### update_ticket_status
**Natural Language:** "Move ticket SURV-123 to In Progress"
```bash
execute -n aaaaaa22qx66rbpoh6ak2mmjfb5kc2ryjnoqzp33ujzi73u3aze5wkij7e -m update_ticket_status -i '{"ticket_key":"SURV-123","new_status":"In Progress"}'
```

### tools
**Natural Language:** "What methods are available in Jira?"
```bash
execute -n aaaaaa22qx66rbpoh6ak2mmjfb5kc2ryjnoqzp33ujzi73u3aze5wkij7e -m tools
```

---

## Key Entity IDs Reference

| Entity ID | Name | Type |
|-----------|------|------|
| ENT-REL-001 | Mukesh Ambani | Reliance CMD |
| ENT-REL-006 | Srikanth Venkatachari | Reliance CFO |
| ENT-INF-001 | Salil Parekh | Infosys CEO |
| ENT-TCS-001 | K. Krithivasan | TCS CEO |
| SUS-001 | Vikram Shell Holdings | Suspect |
| SUS-003 | Deepak Malhotra | Suspect |
| BRK-001 | Alpha Securities | Broker |
| ACC017 | Trading Account 17 | Account |
| ACC042 | Trading Account 42 | Account |

## Company Symbols

- `RELIANCE` - Reliance Industries
- `INFY` - Infosys
- `TCS` - Tata Consultancy Services
- `HDFCBANK` - HDFC Bank
- `IBM` - IBM (US Stock)
- `AAPL` - Apple (US Stock)
- `MSFT` - Microsoft (US Stock)
- `GOOGL` - Google (US Stock)
- `TSLA` - Tesla (US Stock)
