# Trade Data MCP - Testing Commands

## Deployment
First deploy the contract:
```bash
cd /Users/preetham/Documents/demoo/trade_data_mcp
deploy -e -f target/wasm32-unknown-unknown/release/trade_data_mcp.wasm \
        -p trade_data.widl \
        -c config.yaml
```

Save the contract_address from the deployment output and use it below.

---

## Test Commands for All Methods

Replace `<CONTRACT_ID>` with the deployed contract address.

### 1. Test get_trades_by_symbol
Fetch trades for IBM stock:
```bash
icarus call <CONTRACT_ID> get_trades_by_symbol \
  '{"symbol": "IBM", "from_timestamp": 1704067200000, "to_timestamp": 1737225600000, "limit": 5}'
```

### 2. Test analyze_volume  
Get volume statistics for AAPL:
```bash
icarus call <CONTRACT_ID> analyze_volume \
  '{"symbol": "AAPL", "from_timestamp": 1704067200000, "to_timestamp": 1737225600000}'
```

### 3. Test detect_volume_anomaly
Check for volume spikes in MSFT:
```bash
icarus call <CONTRACT_ID> detect_volume_anomaly \
  '{"symbol": "MSFT", "date_timestamp": 1737225600000}'
```

### 4. Test get_top_traders
Find top 5 traders for GOOGL:
```bash
icarus call <CONTRACT_ID> get_top_traders \
  '{"symbol": "GOOGL", "from_timestamp": 1704067200000, "to_timestamp": 1737225600000, "limit": 5}'
```

### 5. Test get_large_orders
Find institutional orders over $1M:
```bash
icarus call <CONTRACT_ID> get_large_orders \
  '{"min_value": 1000000, "from_timestamp": 1704067200000, "to_timestamp": 1737225600000}'
```

### 6. Test get_account_profile
Get trading profile for ACC001:
```bash
icarus call <CONTRACT_ID> get_account_profile \
  '{"account_id": "ACC001", "days_back": 30}'
```

### 7. Test get_trades_by_account
Get all trades for account ACC050:
```bash
icarus call <CONTRACT_ID> get_trades_by_account \
  '{"account_id": "ACC050", "from_timestamp": 1704067200000, "to_timestamp": 1737225600000, "limit": 10}'
```

###  8. Test get_trades_by_accounts
Get trades for multiple accounts on TSLA:
```bash
icarus call <CONTRACT_ID> get_trades_by_accounts \
  '{"account_ids": "ACC001,ACC002,ACC003", "symbol": "TSLA", "from_timestamp": 1704067200000, "to_timestamp": 1737225600000}'
```

### 9. Test get_trade
Fetch a specific trade by ID (you'll need to get a trade_id from method 1 first):
```bash
# First run get_trades_by_symbol to get a trade_id, then:
icarus call <CONTRACT_ID> get_trade \
  '{"trade_id": "IBM_1704067200000_ACC001_0"}'
```

### 10. Test tools() - Verify MCP Schema
Check that all tools are properly exposed:
```bash
icarus call <CONTRACT_ID> tools '{}'
```

---

## Quick Test Script

Run all tests in sequence (replace <CONTRACT_ID> first):

```bash
#!/bin/bash
CONTRACT="<CONTRACT_ID>"

echo "=== Testing Trade Data MCP ==="

echo "\n1. Testing get_trades_by_symbol..."
icarus call $CONTRACT get_trades_by_symbol \
  '{"symbol": "IBM", "from_timestamp": 1704067200000, "to_timestamp": 1737225600000, "limit": 5}'

echo "\n2. Testing analyze_volume..."
icarus call $CONTRACT analyze_volume \
  '{"symbol": "AAPL", "from_timestamp": 1704067200000, "to_timestamp": 1737225600000}'

echo "\n3. Testing detect_volume_anomaly..."
icarus call $CONTRACT detect_volume_anomaly \
  '{"symbol": "MSFT", "date_timestamp": 1737225600000}'

echo "\n4. Testing get_top_traders..."
icarus call $CONTRACT get_top_traders \
  '{"symbol": "GOOGL", "from_timestamp": 1704067200000, "to_timestamp": 1737225600000, "limit": 5}'

echo "\n5. Testing get_large_orders..."
icarus call $CONTRACT get_large_orders \
  '{"min_value": 1000000, "from_timestamp": 1704067200000, "to_timestamp": 1737225600000}'

echo "\n6. Testing tools schema..."
icarus call $CONTRACT tools '{}'

echo "\n=== All Tests Complete ==="
```

---

## Expected Results

**get_trades_by_symbol**: Array of Trade objects with real IBM price data  
**analyze_volume**: TradeAnalysis with volume stats, price ranges, concentration ratio  
**detect_volume_anomaly**: VolumeAnomaly showing if current volume is abnormal
**get_top_traders**: Top 5 AccountActivity records sorted by volume
**get_large_orders**: Trades with value >= $1M sorted by value
**get_account_profile**: AccountActivity across multiple symbols for one account
**tools**: JSON array with all 9 function definitions

---

## Troubleshooting

If you get **"Alpha Vantage API rate limit reached"**:
- You've exceeded 25 requests/day on free tier
- Use api_key_2 or api_key_3 in config
- Or wait 24 hours for reset
- Or upgrade to premium tier (75 req/min)

If you get **"No time series data found"**:
- Alpha Vantage might not have data for that symbol
- Try well-known symbols: IBM, AAPL, MSFT, GOOGL, TSLA
- Check if symbol is valid on Alpha Vantage

If methods return empty arrays:
- Timestamp might be outside available data range
- Try recent timestamps (last 30 days)
- Alpha Vantage intraday data has limited history
