#!/usr/bin/env python3
"""
Test script that mirrors the UPSI Database MCP Rust logic.
Tests each function with real API calls to Supabase.
"""

import urllib.request
import urllib.error
import json
import ssl

# Config from config.yaml
SUPABASE_URL = "https://gmjhagtqxczsfitnfztg.supabase.co"
SUPABASE_ANON_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImdtamhhZ3RxeGN6c2ZpdG5menRnIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjgxNDg5NDUsImV4cCI6MjA4MzcyNDk0NX0.XXDM-Y1824IwAxfeysP9CxCIRWA88V1GkYgKBN4ODwg"

ctx = ssl.create_default_context()

def supabase_request(endpoint):
    """Mirror of Rust supabase_request function"""
    url = f"{SUPABASE_URL}/rest/v1/{endpoint}"
    req = urllib.request.Request(url)
    req.add_header("apikey", SUPABASE_ANON_KEY)
    req.add_header("Authorization", f"Bearer {SUPABASE_ANON_KEY}")
    req.add_header("Content-Type", "application/json")
    req.add_header("Prefer", "return=representation")
    
    try:
        with urllib.request.urlopen(req, context=ctx, timeout=10) as response:
            return json.loads(response.read().decode())
    except urllib.error.HTTPError as e:
        body = e.read().decode() if e.fp else ""
        raise Exception(f"HTTP {e.code}: {body}")

def test_get_active_upsi(company_symbol):
    """Test: Show me all active UPSI for {company}"""
    print(f"\n{'='*60}")
    print(f"TEST: get_active_upsi('{company_symbol}')")
    print(f"{'='*60}")
    
    # Exact endpoint from Rust code (line 142)
    endpoint = f"upsi_records?company_symbol=eq.{company_symbol}&is_public=eq.false&select=*"
    print(f"Endpoint: {endpoint}")
    
    try:
        records = supabase_request(endpoint)
        print(f"✅ SUCCESS: Found {len(records)} active UPSI records")
        for r in records[:3]:  # Show first 3
            print(f"   - {r.get('upsi_id')}: {r.get('upsi_type')} - {r.get('description', '')[:50]}")
        return records
    except Exception as e:
        print(f"❌ FAILED: {e}")
        return None

def test_get_upsi_accessors(upsi_id):
    """Test: Who accessed {upsi_id}"""
    print(f"\n{'='*60}")
    print(f"TEST: get_upsi_accessors('{upsi_id}')")
    print(f"{'='*60}")
    
    # Exact endpoint from Rust code (line 233)
    endpoint = f"upsi_access_log?upsi_id=eq.{upsi_id}&select=*"
    print(f"Endpoint: {endpoint}")
    
    try:
        logs = supabase_request(endpoint)
        print(f"✅ SUCCESS: Found {len(logs)} access records")
        for log in logs[:3]:
            print(f"   - {log.get('accessor_name')} ({log.get('accessor_designation')}) - {log.get('access_reason', '')[:40]}")
        return logs
    except Exception as e:
        print(f"❌ FAILED: {e}")
        return None

def test_get_trading_window(company_symbol):
    """Test: What is the trading window status for {company}"""
    print(f"\n{'='*60}")
    print(f"TEST: get_trading_window('{company_symbol}')")
    print(f"{'='*60}")
    
    # Exact endpoint from Rust code (line 205)
    endpoint = f"trading_windows?company_symbol=eq.{company_symbol}&select=*"
    print(f"Endpoint: {endpoint}")
    
    try:
        windows = supabase_request(endpoint)
        if windows:
            w = windows[0]
            print(f"✅ SUCCESS: Trading window found")
            print(f"   Status: {w.get('window_status')}")
            print(f"   Reason: {w.get('closure_reason', 'N/A')}")
        else:
            print(f"⚠️  No trading window found for {company_symbol}")
        return windows
    except Exception as e:
        print(f"❌ FAILED: {e}")
        return None

def test_get_upsi(upsi_id):
    """Test: Get UPSI by ID"""
    print(f"\n{'='*60}")
    print(f"TEST: get_upsi('{upsi_id}')")
    print(f"{'='*60}")
    
    # Exact endpoint from Rust code (line 131)
    endpoint = f"upsi_records?upsi_id=eq.{upsi_id}&select=*"
    print(f"Endpoint: {endpoint}")
    
    try:
        records = supabase_request(endpoint)
        if records:
            r = records[0]
            print(f"✅ SUCCESS: UPSI record found")
            print(f"   Type: {r.get('upsi_type')}")
            print(f"   Company: {r.get('company_symbol')}")
            print(f"   Description: {r.get('description', '')[:60]}")
        else:
            print(f"⚠️  UPSI record {upsi_id} not found")
        return records[0] if records else None
    except Exception as e:
        print(f"❌ FAILED: {e}")
        return None

if __name__ == "__main__":
    print("\n" + "="*60)
    print("UPSI DATABASE MCP - LOGIC TEST")
    print("Testing the exact same REST API calls as the Rust code")
    print("="*60)
    
    # Test 1: Active UPSI for RELIANCE
    test_get_active_upsi("RELIANCE")
    
    # Test 2: Who accessed UPSI-001
    test_get_upsi_accessors("UPSI-001")
    
    # Test 3: Trading window for RELIANCE
    test_get_trading_window("RELIANCE")
    
    # Test 4: Get specific UPSI
    test_get_upsi("UPSI-001")
    
    print("\n" + "="*60)
    print("ALL TESTS COMPLETE")
    print("="*60)
