// =====================================================================
// Neo4j Test Data for Entity Relationship MCP
// Market Surveillance - Indian Capital Markets
// =====================================================================
// Run this in Neo4j Browser or Neo4j Aura to populate the database
// =====================================================================

// ===== CLEAR EXISTING DATA (OPTIONAL - UNCOMMENT IF NEEDED) =====
// MATCH (n) DETACH DELETE n;

// =====================================================================
// PART 1: CREATE COMPANIES (Listed on NSE/BSE)
// =====================================================================

CREATE (reliance:Company {
  symbol: 'RELIANCE',
  name: 'Reliance Industries Limited',
  sector: 'Oil & Gas',
  market_cap: 'Large Cap',
  isin: 'INE002A01018'
});

CREATE (infy:Company {
  symbol: 'INFY',
  name: 'Infosys Limited',
  sector: 'IT Services',
  market_cap: 'Large Cap',
  isin: 'INE009A01021'
});

CREATE (tcs:Company {
  symbol: 'TCS',
  name: 'Tata Consultancy Services Limited',
  sector: 'IT Services',
  market_cap: 'Large Cap',
  isin: 'INE467B01029'
});

CREATE (hdfc:Company {
  symbol: 'HDFCBANK',
  name: 'HDFC Bank Limited',
  sector: 'Banking',
  market_cap: 'Large Cap',
  isin: 'INE040A01034'
});

CREATE (wipro:Company {
  symbol: 'WIPRO',
  name: 'Wipro Limited',
  sector: 'IT Services',
  market_cap: 'Large Cap',
  isin: 'INE075A01022'
});

CREATE (bharti:Company {
  symbol: 'BHARTIARTL',
  name: 'Bharti Airtel Limited',
  sector: 'Telecom',
  market_cap: 'Large Cap',
  isin: 'INE397D01024'
});

CREATE (icici:Company {
  symbol: 'ICICIBANK',
  name: 'ICICI Bank Limited',
  sector: 'Banking',
  market_cap: 'Large Cap',
  isin: 'INE090A01021'
});

CREATE (sbi:Company {
  symbol: 'SBIN',
  name: 'State Bank of India',
  sector: 'Banking',
  market_cap: 'Large Cap',
  isin: 'INE062A01020'
});

// =====================================================================
// PART 2: CREATE ENTITIES (Individuals & Institutions)
// =====================================================================

// ----- RELIANCE INDUSTRIES INSIDERS -----
CREATE (mukesh:Entity {
  entity_id: 'ENT-REL-001',
  entity_type: 'INDIVIDUAL',
  name: 'Mukesh D. Ambani',
  pan_number: 'AABPA1234A',
  registration_id: 'DIN-00001695'
});

CREATE (nita:Entity {
  entity_id: 'ENT-REL-002',
  entity_type: 'INDIVIDUAL',
  name: 'Nita M. Ambani',
  pan_number: 'AABPA1235B',
  registration_id: 'DIN-00005457'
});

CREATE (isha:Entity {
  entity_id: 'ENT-REL-003',
  entity_type: 'INDIVIDUAL',
  name: 'Isha M. Ambani',
  pan_number: 'AABPA1236C',
  registration_id: 'DIN-07889524'
});

CREATE (akash:Entity {
  entity_id: 'ENT-REL-004',
  entity_type: 'INDIVIDUAL',
  name: 'Akash M. Ambani',
  pan_number: 'AABPA1237D',
  registration_id: 'DIN-07889525'
});

CREATE (anant:Entity {
  entity_id: 'ENT-REL-005',
  entity_type: 'INDIVIDUAL',
  name: 'Anant M. Ambani',
  pan_number: 'AABPA1238E',
  registration_id: ''
});

CREATE (reliance_cfo:Entity {
  entity_id: 'ENT-REL-006',
  entity_type: 'INDIVIDUAL',
  name: 'Srikanth Venkatachari',
  pan_number: 'ABCPV2345F',
  registration_id: 'DIN-00076238'
});

CREATE (reliance_cs:Entity {
  entity_id: 'ENT-REL-007',
  entity_type: 'INDIVIDUAL',
  name: 'Savithri Parekh',
  pan_number: 'ABCPP3456G',
  registration_id: 'CS-A12345'
});

// ----- INFOSYS INSIDERS -----
CREATE (salil:Entity {
  entity_id: 'ENT-INF-001',
  entity_type: 'INDIVIDUAL',
  name: 'Salil S. Parekh',
  pan_number: 'BCDPP4567H',
  registration_id: 'DIN-06804050'
});

CREATE (nilanjan:Entity {
  entity_id: 'ENT-INF-002',
  entity_type: 'INDIVIDUAL',
  name: 'Nilanjan Roy',
  pan_number: 'BCDPR5678I',
  registration_id: 'DIN-06981600'
});

CREATE (nandan:Entity {
  entity_id: 'ENT-INF-003',
  entity_type: 'INDIVIDUAL',
  name: 'Nandan M. Nilekani',
  pan_number: 'BCDPN6789J',
  registration_id: 'DIN-00041245'
});

CREATE (infy_cs:Entity {
  entity_id: 'ENT-INF-004',
  entity_type: 'INDIVIDUAL',
  name: 'A.G.S. Manikantha',
  pan_number: 'BCDPM7890K',
  registration_id: 'CS-A23456'
});

// ----- TCS INSIDERS -----
CREATE (rajesh:Entity {
  entity_id: 'ENT-TCS-001',
  entity_type: 'INDIVIDUAL',
  name: 'K. Krithivasan',
  pan_number: 'CDEPR8901L',
  registration_id: 'DIN-07696864'
});

CREATE (tcs_cfo:Entity {
  entity_id: 'ENT-TCS-002',
  entity_type: 'INDIVIDUAL',
  name: 'Samir Seksaria',
  pan_number: 'CDEPS9012M',
  registration_id: 'DIN-02591202'
});

CREATE (natarajan:Entity {
  entity_id: 'ENT-TCS-003',
  entity_type: 'INDIVIDUAL',
  name: 'N. Chandrasekaran',
  pan_number: 'CDEPN0123N',
  registration_id: 'DIN-00121863'
});

// ----- HDFC BANK INSIDERS -----
CREATE (sashidhar:Entity {
  entity_id: 'ENT-HDFC-001',
  entity_type: 'INDIVIDUAL',
  name: 'Sashidhar Jagdishan',
  pan_number: 'DEFPJ1234O',
  registration_id: 'DIN-08122867'
});

CREATE (hdfc_cfo:Entity {
  entity_id: 'ENT-HDFC-002',
  entity_type: 'INDIVIDUAL',
  name: 'Srinivasan Vaidyanathan',
  pan_number: 'DEFPV2345P',
  registration_id: 'DIN-05005648'
});

// ----- TRADING ENTITIES (BROKERS & DEALERS) -----
CREATE (broker1:Entity {
  entity_id: 'BRK-001',
  entity_type: 'BROKER',
  name: 'Alpha Securities Pvt Ltd',
  pan_number: 'AAACA1234A',
  registration_id: 'SEBI-MB-12345'
});

CREATE (broker2:Entity {
  entity_id: 'BRK-002',
  entity_type: 'BROKER',
  name: 'Beta Capital Markets',
  pan_number: 'AAACB2345B',
  registration_id: 'SEBI-MB-23456'
});

CREATE (broker3:Entity {
  entity_id: 'BRK-003',
  entity_type: 'BROKER',
  name: 'Gamma Trading Co',
  pan_number: 'AAACC3456C',
  registration_id: 'SEBI-MB-34567'
});

// ----- INDIVIDUAL TRADERS (Non-Insiders) -----
CREATE (trader1:Entity {
  entity_id: 'TRD-001',
  entity_type: 'INDIVIDUAL',
  name: 'Rahul Kumar Singh',
  pan_number: 'EFGPS4567D',
  registration_id: ''
});

CREATE (trader2:Entity {
  entity_id: 'TRD-002',
  entity_type: 'INDIVIDUAL',
  name: 'Priya Sharma',
  pan_number: 'EFGPS5678E',
  registration_id: ''
});

CREATE (trader3:Entity {
  entity_id: 'TRD-003',
  entity_type: 'INDIVIDUAL',
  name: 'Arun Mehta',
  pan_number: 'EFGPM6789F',
  registration_id: ''
});

CREATE (trader4:Entity {
  entity_id: 'TRD-004',
  entity_type: 'INDIVIDUAL',
  name: 'Kavitha Reddy',
  pan_number: 'EFGPR7890G',
  registration_id: ''
});

// ----- INSTITUTIONAL INVESTORS -----
CREATE (fii1:Entity {
  entity_id: 'FII-001',
  entity_type: 'FII',
  name: 'Global Growth Fund LLC',
  pan_number: 'AAAGG1234H',
  registration_id: 'FII-1234-2020'
});

CREATE (fii2:Entity {
  entity_id: 'FII-002',
  entity_type: 'FII',
  name: 'Asia Pacific Investments',
  pan_number: 'AAAGI2345I',
  registration_id: 'FII-2345-2021'
});

CREATE (dii1:Entity {
  entity_id: 'DII-001',
  entity_type: 'DII',
  name: 'LIC Mutual Fund',
  pan_number: 'AAALL3456J',
  registration_id: 'MF-1234-2015'
});

CREATE (dii2:Entity {
  entity_id: 'DII-002',
  entity_type: 'DII',
  name: 'SBI Equity Fund',
  pan_number: 'AAASB4567K',
  registration_id: 'MF-2345-2016'
});

// ----- SUSPICIOUS ENTITIES (For Testing) -----
CREATE (suspect1:Entity {
  entity_id: 'SUS-001',
  entity_type: 'INDIVIDUAL',
  name: 'Vikram Shell Holdings',
  pan_number: 'GHIPV8901L',
  registration_id: ''
});

CREATE (suspect2:Entity {
  entity_id: 'SUS-002',
  entity_type: 'CORPORATE',
  name: 'Offshore Trading SPV',
  pan_number: 'GHIPO9012M',
  registration_id: 'CIN-U12345'
});

CREATE (suspect3:Entity {
  entity_id: 'SUS-003',
  entity_type: 'INDIVIDUAL',
  name: 'Deepak K. Malhotra',
  pan_number: 'GHIPM0123N',
  registration_id: ''
});

// =====================================================================
// PART 3: CREATE INSIDER RELATIONSHIPS
// =====================================================================

// ----- RELIANCE INSIDERS -----
MATCH (e:Entity {entity_id: 'ENT-REL-001'}), (c:Company {symbol: 'RELIANCE'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'PROMOTER',
  designation: 'Chairman & Managing Director',
  window_status: 'CLOSED',
  since: '1977-01-01'
}]->(c);

MATCH (e:Entity {entity_id: 'ENT-REL-002'}), (c:Company {symbol: 'RELIANCE'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'PROMOTER_GROUP',
  designation: 'Non-Executive Director',
  window_status: 'CLOSED',
  since: '2014-06-12'
}]->(c);

MATCH (e:Entity {entity_id: 'ENT-REL-003'}), (c:Company {symbol: 'RELIANCE'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'PROMOTER_GROUP',
  designation: 'Director - Reliance Retail',
  window_status: 'CLOSED',
  since: '2018-10-01'
}]->(c);

MATCH (e:Entity {entity_id: 'ENT-REL-004'}), (c:Company {symbol: 'RELIANCE'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'PROMOTER_GROUP',
  designation: 'Director - Reliance Jio',
  window_status: 'CLOSED',
  since: '2018-10-01'
}]->(c);

MATCH (e:Entity {entity_id: 'ENT-REL-005'}), (c:Company {symbol: 'RELIANCE'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'PROMOTER_GROUP',
  designation: 'Relative of Promoter',
  window_status: 'CLOSED',
  since: '2023-01-01'
}]->(c);

MATCH (e:Entity {entity_id: 'ENT-REL-006'}), (c:Company {symbol: 'RELIANCE'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'KMP',
  designation: 'Chief Financial Officer',
  window_status: 'CLOSED',
  since: '2014-11-01'
}]->(c);

MATCH (e:Entity {entity_id: 'ENT-REL-007'}), (c:Company {symbol: 'RELIANCE'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'KMP',
  designation: 'Company Secretary',
  window_status: 'CLOSED',
  since: '2017-06-01'
}]->(c);

// ----- INFOSYS INSIDERS -----
MATCH (e:Entity {entity_id: 'ENT-INF-001'}), (c:Company {symbol: 'INFY'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'KMP',
  designation: 'CEO & Managing Director',
  window_status: 'OPEN',
  since: '2018-01-02'
}]->(c);

MATCH (e:Entity {entity_id: 'ENT-INF-002'}), (c:Company {symbol: 'INFY'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'KMP',
  designation: 'Chief Financial Officer',
  window_status: 'OPEN',
  since: '2019-03-01'
}]->(c);

MATCH (e:Entity {entity_id: 'ENT-INF-003'}), (c:Company {symbol: 'INFY'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'PROMOTER',
  designation: 'Non-Executive Chairman',
  window_status: 'OPEN',
  since: '2017-08-24'
}]->(c);

MATCH (e:Entity {entity_id: 'ENT-INF-004'}), (c:Company {symbol: 'INFY'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'KMP',
  designation: 'Company Secretary',
  window_status: 'OPEN',
  since: '2021-04-01'
}]->(c);

// ----- TCS INSIDERS -----
MATCH (e:Entity {entity_id: 'ENT-TCS-001'}), (c:Company {symbol: 'TCS'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'KMP',
  designation: 'CEO & Managing Director',
  window_status: 'OPEN',
  since: '2023-06-01'
}]->(c);

MATCH (e:Entity {entity_id: 'ENT-TCS-002'}), (c:Company {symbol: 'TCS'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'KMP',
  designation: 'Chief Financial Officer',
  window_status: 'OPEN',
  since: '2017-10-01'
}]->(c);

MATCH (e:Entity {entity_id: 'ENT-TCS-003'}), (c:Company {symbol: 'TCS'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'PROMOTER_GROUP',
  designation: 'Chairman - Tata Sons',
  window_status: 'CLOSED',
  since: '2017-01-12'
}]->(c);

// N. Chandrasekaran is also HDFC insider (Tata Sons holds stake)
MATCH (e:Entity {entity_id: 'ENT-TCS-003'}), (c:Company {symbol: 'HDFCBANK'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'CONNECTED_PERSON',
  designation: 'Director',
  window_status: 'OPEN',
  since: '2021-04-01'
}]->(c);

// ----- HDFC BANK INSIDERS -----
MATCH (e:Entity {entity_id: 'ENT-HDFC-001'}), (c:Company {symbol: 'HDFCBANK'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'KMP',
  designation: 'CEO & Managing Director',
  window_status: 'OPEN',
  since: '2020-10-27'
}]->(c);

MATCH (e:Entity {entity_id: 'ENT-HDFC-002'}), (c:Company {symbol: 'HDFCBANK'})
CREATE (e)-[:INSIDER_OF {
  insider_type: 'KMP',
  designation: 'Chief Financial Officer',
  window_status: 'OPEN',
  since: '2022-04-01'
}]->(c);

// =====================================================================
// PART 4: CREATE FAMILY RELATIONSHIPS
// =====================================================================

// Ambani Family
MATCH (a:Entity {entity_id: 'ENT-REL-001'}), (b:Entity {entity_id: 'ENT-REL-002'})
CREATE (a)-[:FAMILY {relation: 'SPOUSE', detail: 'Married', strength: 100, verified: true}]->(b);

MATCH (a:Entity {entity_id: 'ENT-REL-001'}), (b:Entity {entity_id: 'ENT-REL-003'})
CREATE (a)-[:FAMILY {relation: 'CHILD', detail: 'Daughter', strength: 100, verified: true}]->(b);

MATCH (a:Entity {entity_id: 'ENT-REL-001'}), (b:Entity {entity_id: 'ENT-REL-004'})
CREATE (a)-[:FAMILY {relation: 'CHILD', detail: 'Son', strength: 100, verified: true}]->(b);

MATCH (a:Entity {entity_id: 'ENT-REL-001'}), (b:Entity {entity_id: 'ENT-REL-005'})
CREATE (a)-[:FAMILY {relation: 'CHILD', detail: 'Son', strength: 100, verified: true}]->(b);

MATCH (a:Entity {entity_id: 'ENT-REL-002'}), (b:Entity {entity_id: 'ENT-REL-003'})
CREATE (a)-[:FAMILY {relation: 'CHILD', detail: 'Daughter', strength: 100, verified: true}]->(b);

MATCH (a:Entity {entity_id: 'ENT-REL-002'}), (b:Entity {entity_id: 'ENT-REL-004'})
CREATE (a)-[:FAMILY {relation: 'CHILD', detail: 'Son', strength: 100, verified: true}]->(b);

MATCH (a:Entity {entity_id: 'ENT-REL-002'}), (b:Entity {entity_id: 'ENT-REL-005'})
CREATE (a)-[:FAMILY {relation: 'CHILD', detail: 'Son', strength: 100, verified: true}]->(b);

// Siblings
MATCH (a:Entity {entity_id: 'ENT-REL-003'}), (b:Entity {entity_id: 'ENT-REL-004'})
CREATE (a)-[:FAMILY {relation: 'SIBLING', detail: 'Brother-Sister', strength: 100, verified: true}]->(b);

MATCH (a:Entity {entity_id: 'ENT-REL-003'}), (b:Entity {entity_id: 'ENT-REL-005'})
CREATE (a)-[:FAMILY {relation: 'SIBLING', detail: 'Brother-Sister', strength: 100, verified: true}]->(b);

MATCH (a:Entity {entity_id: 'ENT-REL-004'}), (b:Entity {entity_id: 'ENT-REL-005'})
CREATE (a)-[:FAMILY {relation: 'SIBLING', detail: 'Brothers', strength: 100, verified: true}]->(b);

// =====================================================================
// PART 5: CREATE PROFESSIONAL RELATIONSHIPS
// =====================================================================

// Board connections (multiple directorships)
MATCH (a:Entity {entity_id: 'ENT-TCS-003'}), (b:Entity {entity_id: 'ENT-HDFC-001'})
CREATE (a)-[:PROFESSIONAL {
  relation: 'BOARD_COLLEAGUE',
  detail: 'Both serve on HDFC Bank board',
  strength: 70,
  verified: true
}]->(b);

// CFO connections (professional network)
MATCH (a:Entity {entity_id: 'ENT-REL-006'}), (b:Entity {entity_id: 'ENT-INF-002'})
CREATE (a)-[:PROFESSIONAL {
  relation: 'INDUSTRY_PEER',
  detail: 'Fellow CFO - IT/Telecom sector',
  strength: 40,
  verified: false
}]->(b);

MATCH (a:Entity {entity_id: 'ENT-INF-002'}), (b:Entity {entity_id: 'ENT-TCS-002'})
CREATE (a)-[:PROFESSIONAL {
  relation: 'INDUSTRY_PEER',
  detail: 'Fellow CFO - IT Services',
  strength: 50,
  verified: true
}]->(b);

// =====================================================================
// PART 6: CREATE TRADING RELATIONSHIPS (Broker-Client)
// =====================================================================

// Broker 1 clients
MATCH (a:Entity {entity_id: 'BRK-001'}), (b:Entity {entity_id: 'TRD-001'})
CREATE (a)-[:BROKER_CLIENT {
  detail: 'Trading account since 2019',
  strength: 80,
  verified: true
}]->(b);

MATCH (a:Entity {entity_id: 'BRK-001'}), (b:Entity {entity_id: 'TRD-002'})
CREATE (a)-[:BROKER_CLIENT {
  detail: 'Trading account since 2020',
  strength: 75,
  verified: true
}]->(b);

MATCH (a:Entity {entity_id: 'BRK-001'}), (b:Entity {entity_id: 'FII-001'})
CREATE (a)-[:BROKER_CLIENT {
  detail: 'FII custody account',
  strength: 90,
  verified: true
}]->(b);

// Broker 2 clients
MATCH (a:Entity {entity_id: 'BRK-002'}), (b:Entity {entity_id: 'TRD-003'})
CREATE (a)-[:BROKER_CLIENT {
  detail: 'Trading account since 2021',
  strength: 70,
  verified: true
}]->(b);

MATCH (a:Entity {entity_id: 'BRK-002'}), (b:Entity {entity_id: 'TRD-004'})
CREATE (a)-[:BROKER_CLIENT {
  detail: 'Trading account since 2022',
  strength: 65,
  verified: true
}]->(b);

MATCH (a:Entity {entity_id: 'BRK-002'}), (b:Entity {entity_id: 'FII-002'})
CREATE (a)-[:BROKER_CLIENT {
  detail: 'FII trading account',
  strength: 85,
  verified: true
}]->(b);

// Broker 3 clients (institutions)
MATCH (a:Entity {entity_id: 'BRK-003'}), (b:Entity {entity_id: 'DII-001'})
CREATE (a)-[:BROKER_CLIENT {
  detail: 'Mutual fund trading',
  strength: 90,
  verified: true
}]->(b);

MATCH (a:Entity {entity_id: 'BRK-003'}), (b:Entity {entity_id: 'DII-002'})
CREATE (a)-[:BROKER_CLIENT {
  detail: 'Mutual fund trading',
  strength: 85,
  verified: true
}]->(b);

// =====================================================================
// PART 7: CREATE SUSPICIOUS CONNECTIONS (For Insider Trading Detection)
// =====================================================================

// Suspect 1 is secretly connected to Reliance CFO (college friend)
MATCH (a:Entity {entity_id: 'SUS-001'}), (b:Entity {entity_id: 'ENT-REL-006'})
CREATE (a)-[:PERSONAL {
  relation: 'COLLEGE_FRIEND',
  detail: 'IIM Ahmedabad batch 1995',
  strength: 60,
  verified: false
}]->(b);

// Suspect 1 trades through Broker 1
MATCH (a:Entity {entity_id: 'BRK-001'}), (b:Entity {entity_id: 'SUS-001'})
CREATE (a)-[:BROKER_CLIENT {
  detail: 'New account opened 2025',
  strength: 50,
  verified: true
}]->(b);

// Suspect 2 (Offshore SPV) connected to Suspect 3
MATCH (a:Entity {entity_id: 'SUS-002'}), (b:Entity {entity_id: 'SUS-003'})
CREATE (a)-[:OWNERSHIP {
  relation: 'BENEFICIAL_OWNER',
  detail: 'UBO through layered structure',
  strength: 90,
  verified: false
}]->(b);

// Suspect 3 is distant relative of Infosys insider
MATCH (a:Entity {entity_id: 'SUS-003'}), (b:Entity {entity_id: 'ENT-INF-001'})
CREATE (a)-[:FAMILY {
  relation: 'DISTANT_RELATIVE',
  detail: 'Cousin of spouse',
  strength: 40,
  verified: false
}]->(b);

// Trader 1 is friend of Trader 3 (potential coordination)
MATCH (a:Entity {entity_id: 'TRD-001'}), (b:Entity {entity_id: 'TRD-003'})
CREATE (a)-[:PERSONAL {
  relation: 'FRIEND',
  detail: 'Neighbors in same apartment',
  strength: 70,
  verified: false
}]->(b);

// =====================================================================
// PART 8: CREATE CORPORATE RELATIONSHIPS
// =====================================================================

// Cross-holdings and group company relationships
MATCH (a:Company {symbol: 'TCS'}), (b:Company {symbol: 'HDFCBANK'})
CREATE (a)-[:GROUP_COMPANY {
  relation: 'TATA_GROUP',
  detail: 'Both under Tata umbrella',
  strength: 80
}]->(b);

// =====================================================================
// VERIFICATION QUERIES - Run these to verify data loaded correctly
// =====================================================================

// Count all nodes
// MATCH (n) RETURN labels(n)[0] AS label, count(*) AS count ORDER BY count DESC;

// Count all relationships
// MATCH ()-[r]->() RETURN type(r) AS relationship, count(*) AS count ORDER BY count DESC;

// View all insiders for RELIANCE
// MATCH (e:Entity)-[r:INSIDER_OF]->(c:Company {symbol: 'RELIANCE'}) RETURN e.name, r.designation, r.window_status;

// Find connected entities within 2 hops of Mukesh Ambani
// MATCH path = (a:Entity {entity_id: 'ENT-REL-001'})-[*1..2]-(b:Entity) RETURN path;

// Find shortest path between two entities
// MATCH path = shortestPath((a:Entity {entity_id: 'SUS-001'})-[*..5]-(b:Entity {entity_id: 'ENT-REL-001'})) RETURN path;
