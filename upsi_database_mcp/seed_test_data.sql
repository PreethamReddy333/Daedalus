-- =====================================================
-- UPSI Database Test Data for Workflow 1
-- Run this in Supabase SQL Editor: https://supabase.com/dashboard/project/kyzzcblbucnpbtgumaca/sql
-- =====================================================

-- Clear existing test data (optional)
-- DELETE FROM upsi_access_log;
-- DELETE FROM upsi_records;
-- DELETE FROM trading_windows;

-- =====================================================
-- 1. UPSI Records - Active UPSI for RELIANCE, TCS, INFY, HDFC
-- =====================================================

INSERT INTO upsi_records (upsi_id, company_symbol, upsi_type, description, nature, created_date, public_date, is_public)
VALUES 
  -- RELIANCE UPSI (Active - not yet public)
  ('UPSI-REL-001', 'RELIANCE', 'FINANCIAL_RESULTS', 'Q4 2025 quarterly results - 25% profit increase expected', 'POSITIVE', 1737100800000, 0, false),
  ('UPSI-REL-002', 'RELIANCE', 'MERGER', 'Acquisition of renewable energy company - $5B deal', 'POSITIVE', 1737014400000, 0, false),
  ('UPSI-REL-003', 'RELIANCE', 'DIVIDEND', 'Special dividend announcement - Rs 50 per share', 'POSITIVE', 1736928000000, 0, false),
  
  -- TCS UPSI
  ('UPSI-TCS-001', 'TCS', 'FINANCIAL_RESULTS', 'Q4 2025 results - revenue growth 15%', 'POSITIVE', 1737100800000, 0, false),
  ('UPSI-TCS-002', 'TCS', 'CONTRACT', 'Major government contract worth Rs 10,000 Cr', 'POSITIVE', 1737014400000, 0, false),
  
  -- INFY UPSI
  ('UPSI-INFY-001', 'INFY', 'FINANCIAL_RESULTS', 'Q4 2025 results - margin improvement', 'POSITIVE', 1737100800000, 0, false),
  ('UPSI-INFY-002', 'INFY', 'BUYBACK', 'Share buyback program - Rs 5,000 Cr', 'POSITIVE', 1736928000000, 0, false),
  
  -- HDFC UPSI
  ('UPSI-HDFC-001', 'HDFC', 'MERGER', 'HDFC Life stake increase to 75%', 'POSITIVE', 1737100800000, 0, false),
  
  -- ICICI UPSI
  ('UPSI-ICICI-001', 'ICICI', 'FINANCIAL_RESULTS', 'NPA reduction - asset quality improvement', 'POSITIVE', 1737100800000, 0, false),
  
  -- Generic UPSI for testing
  ('UPSI-MERGER-001', 'RELIANCE', 'MERGER', 'Jio-Airtel spectrum sharing agreement', 'NEUTRAL', 1736841600000, 0, false),
  ('UPSI-FIN-001', 'RELIANCE', 'FINANCIAL_RESULTS', 'Petrochemicals division spin-off plan', 'POSITIVE', 1736755200000, 0, false)
ON CONFLICT (upsi_id) DO UPDATE SET
  company_symbol = EXCLUDED.company_symbol,
  upsi_type = EXCLUDED.upsi_type,
  description = EXCLUDED.description,
  nature = EXCLUDED.nature,
  created_date = EXCLUDED.created_date,
  public_date = EXCLUDED.public_date,
  is_public = EXCLUDED.is_public;

-- =====================================================
-- 2. UPSI Access Logs - Who accessed each UPSI
-- =====================================================

INSERT INTO upsi_access_log (access_id, upsi_id, accessor_entity_id, accessor_name, accessor_designation, access_timestamp, access_reason, access_mode)
VALUES 
  -- ENT-0001 accessing RELIANCE UPSI (for Workflow 1)
  ('ACC-001', 'UPSI-REL-001', 'ENT-0001', 'Rajesh Kumar', 'Senior Analyst', 1737187200000, 'Financial analysis for Q4 report', 'VIEW'),
  ('ACC-002', 'UPSI-REL-002', 'ENT-0001', 'Rajesh Kumar', 'Senior Analyst', 1737100800000, 'M&A due diligence review', 'VIEW'),
  ('ACC-003', 'UPSI-REL-003', 'ENT-0001', 'Rajesh Kumar', 'Senior Analyst', 1737014400000, 'Dividend policy review', 'VIEW'),
  
  -- ENT-0002 accessing RELIANCE UPSI
  ('ACC-004', 'UPSI-REL-001', 'ENT-0002', 'Priya Sharma', 'Compliance Officer', 1737180000000, 'Compliance audit', 'VIEW'),
  ('ACC-005', 'UPSI-MERGER-001', 'ENT-0002', 'Priya Sharma', 'Compliance Officer', 1737093600000, 'Regulatory filing review', 'VIEW'),
  
  -- Mukesh Ambani (Promoter) accessing UPSI
  ('ACC-006', 'UPSI-REL-001', 'Mukesh Ambani', 'Mukesh Ambani', 'Chairman', 1737172800000, 'Board presentation preparation', 'VIEW'),
  ('ACC-007', 'UPSI-REL-002', 'Mukesh Ambani', 'Mukesh Ambani', 'Chairman', 1737079200000, 'Deal approval', 'VIEW'),
  
  -- Neeta Ambani accessing
  ('ACC-008', 'UPSI-REL-003', 'Neeta Ambani', 'Neeta Ambani', 'Board Member', 1737165600000, 'Board meeting agenda review', 'VIEW'),
  
  -- Amit Patel accessing INFY UPSI
  ('ACC-009', 'UPSI-INFY-001', 'Amit Patel', 'Amit Patel', 'Portfolio Manager', 1737158400000, 'Investment research', 'VIEW'),
  ('ACC-010', 'UPSI-INFY-002', 'Amit Patel', 'Amit Patel', 'Portfolio Manager', 1737072000000, 'Buyback impact analysis', 'VIEW'),
  
  -- TCS access logs
  ('ACC-011', 'UPSI-TCS-001', 'ENT-0003', 'Suresh Reddy', 'Fund Manager', 1737151200000, 'Quarterly review', 'VIEW'),
  ('ACC-012', 'UPSI-TCS-002', 'ENT-0003', 'Suresh Reddy', 'Fund Manager', 1737064800000, 'Contract analysis', 'VIEW'),
  
  -- HDFC access logs
  ('ACC-013', 'UPSI-HDFC-001', 'BROKER-X', 'BROKER-X', 'Trading Desk', 1737144000000, 'Client advisory', 'VIEW'),
  
  -- ICICI access logs
  ('ACC-014', 'UPSI-ICICI-001', 'BROKER-Y', 'BROKER-Y', 'Research Analyst', 1737136800000, 'Sector report', 'VIEW'),
  
  -- Additional access for UPSI-FIN-001
  ('ACC-015', 'UPSI-FIN-001', 'ENT-0001', 'Rajesh Kumar', 'Senior Analyst', 1736928000000, 'Strategic review', 'VIEW'),
  ('ACC-016', 'UPSI-FIN-001', 'ENT-0002', 'Priya Sharma', 'Compliance Officer', 1736841600000, 'Risk assessment', 'VIEW'),
  ('ACC-017', 'UPSI-FIN-001', 'Mukesh Ambani', 'Mukesh Ambani', 'Chairman', 1736755200000, 'Approval', 'VIEW')
ON CONFLICT (access_id) DO UPDATE SET
  upsi_id = EXCLUDED.upsi_id,
  accessor_entity_id = EXCLUDED.accessor_entity_id,
  accessor_name = EXCLUDED.accessor_name,
  accessor_designation = EXCLUDED.accessor_designation,
  access_timestamp = EXCLUDED.access_timestamp,
  access_reason = EXCLUDED.access_reason,
  access_mode = EXCLUDED.access_mode;

-- =====================================================
-- 3. Trading Windows - Current status for each company
-- =====================================================

INSERT INTO trading_windows (company_symbol, window_status, closure_reason, closure_start, expected_opening)
VALUES 
  -- RELIANCE - CLOSED (for testing window violation)
  ('RELIANCE', 'CLOSED', 'Q4 2025 financial results announcement', 1737100800000, 1737446400000),
  
  -- TCS - CLOSED
  ('TCS', 'CLOSED', 'Q4 2025 financial results announcement', 1737100800000, 1737360000000),
  
  -- INFY - OPEN
  ('INFY', 'OPEN', '', 0, 0),
  
  -- HDFC - CLOSED
  ('HDFC', 'CLOSED', 'Merger announcement pending', 1736928000000, 1737532800000),
  
  -- ICICI - OPEN
  ('ICICI', 'OPEN', '', 0, 0),
  
  -- SBIN - OPEN
  ('SBIN', 'OPEN', '', 0, 0),
  
  -- KOTAKBANK - CLOSED
  ('KOTAKBANK', 'CLOSED', 'Annual results announcement', 1737014400000, 1737273600000)
ON CONFLICT (company_symbol) DO UPDATE SET
  window_status = EXCLUDED.window_status,
  closure_reason = EXCLUDED.closure_reason,
  closure_start = EXCLUDED.closure_start,
  expected_opening = EXCLUDED.expected_opening;

-- =====================================================
-- Verification Queries
-- =====================================================

-- Check UPSI records for RELIANCE
SELECT * FROM upsi_records WHERE company_symbol = 'RELIANCE' AND is_public = false;

-- Check accessors for RELIANCE UPSI
SELECT * FROM upsi_access_log WHERE upsi_id LIKE 'UPSI-REL%' ORDER BY access_timestamp DESC;

-- Check trading window status
SELECT * FROM trading_windows;

-- Check ENT-0001 access history
SELECT * FROM upsi_access_log WHERE accessor_entity_id = 'ENT-0001';
