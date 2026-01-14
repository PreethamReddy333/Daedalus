#!/usr/bin/env python3
"""
Test script to verify Supabase REST API connection.
Run: python3 test_connection.py
"""

import urllib.request
import urllib.error
import json
import ssl

# Connection details from config.yaml
SUPABASE_URL = "https://gmjhagtqxczsfitnfztg.supabase.co"
SUPABASE_ANON_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImdtamhhZ3RxeGN6c2ZpdG5menRnIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjgxNDg5NDUsImV4cCI6MjA4MzcyNDk0NX0.XXDM-Y1824IwAxfeysP9CxCIRWA88V1GkYgKBN4ODwg"

def test_connection():
    print("=" * 50)
    print("Testing Supabase REST API Connection")
    print("=" * 50)
    print(f"URL: {SUPABASE_URL}")
    print("-" * 50)
    
    # Create SSL context
    ctx = ssl.create_default_context()
    
    # Test 1: Check if we can reach the API
    print("\n1. Testing API connectivity...")
    try:
        url = f"{SUPABASE_URL}/rest/v1/"
        req = urllib.request.Request(url)
        req.add_header("apikey", SUPABASE_ANON_KEY)
        req.add_header("Authorization", f"Bearer {SUPABASE_ANON_KEY}")
        
        with urllib.request.urlopen(req, context=ctx, timeout=10) as response:
            print("   ✅ API is reachable")
    except urllib.error.HTTPError as e:
        if e.code == 404:
            print("   ✅ API is reachable (404 is expected for root)")
        else:
            print(f"   ❌ API error: {e.code} {e.reason}")
            return False
    except Exception as e:
        print(f"   ❌ Connection failed: {e}")
        return False
    
    # Test 2: Query upsi_records table
    print("\n2. Querying upsi_records table...")
    try:
        url = f"{SUPABASE_URL}/rest/v1/upsi_records?select=*&limit=5"
        req = urllib.request.Request(url)
        req.add_header("apikey", SUPABASE_ANON_KEY)
        req.add_header("Authorization", f"Bearer {SUPABASE_ANON_KEY}")
        req.add_header("Content-Type", "application/json")
        
        with urllib.request.urlopen(req, context=ctx, timeout=10) as response:
            data = json.loads(response.read().decode())
            print(f"   ✅ Table accessible! Found {len(data)} records")
            if data:
                print(f"   📋 Sample record keys: {list(data[0].keys())}")
    except urllib.error.HTTPError as e:
        body = e.read().decode() if e.fp else ""
        if "does not exist" in body or e.code == 404:
            print("   ⚠️  Table 'upsi_records' does not exist yet")
            print("   💡 Run the CREATE TABLE SQL from config.yaml")
        else:
            print(f"   ❌ Query failed: {e.code} - {body}")
    except Exception as e:
        print(f"   ❌ Error: {e}")
    
    # Test 3: Query upsi_access_log table
    print("\n3. Querying upsi_access_log table...")
    try:
        url = f"{SUPABASE_URL}/rest/v1/upsi_access_log?select=*&limit=5"
        req = urllib.request.Request(url)
        req.add_header("apikey", SUPABASE_ANON_KEY)
        req.add_header("Authorization", f"Bearer {SUPABASE_ANON_KEY}")
        
        with urllib.request.urlopen(req, context=ctx, timeout=10) as response:
            data = json.loads(response.read().decode())
            print(f"   ✅ Table accessible! Found {len(data)} records")
    except urllib.error.HTTPError as e:
        body = e.read().decode() if e.fp else ""
        if "does not exist" in body or e.code == 404:
            print("   ⚠️  Table 'upsi_access_log' does not exist yet")
        else:
            print(f"   ❌ Query failed: {e.code}")
    except Exception as e:
        print(f"   ❌ Error: {e}")
    
    # Test 4: Query trading_windows table
    print("\n4. Querying trading_windows table...")
    try:
        url = f"{SUPABASE_URL}/rest/v1/trading_windows?select=*&limit=5"
        req = urllib.request.Request(url)
        req.add_header("apikey", SUPABASE_ANON_KEY)
        req.add_header("Authorization", f"Bearer {SUPABASE_ANON_KEY}")
        
        with urllib.request.urlopen(req, context=ctx, timeout=10) as response:
            data = json.loads(response.read().decode())
            print(f"   ✅ Table accessible! Found {len(data)} records")
    except urllib.error.HTTPError as e:
        body = e.read().decode() if e.fp else ""
        if "does not exist" in body or e.code == 404:
            print("   ⚠️  Table 'trading_windows' does not exist yet")
        else:
            print(f"   ❌ Query failed: {e.code}")
    except Exception as e:
        print(f"   ❌ Error: {e}")
    
    print("\n" + "=" * 50)
    print("✅ REST API connection test complete!")
    print("=" * 50)
    return True

if __name__ == "__main__":
    test_connection()
