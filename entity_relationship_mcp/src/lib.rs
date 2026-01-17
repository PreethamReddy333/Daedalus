//! # Entity Relationship MCP Server
//!
//! Maps relationships between entities using Neo4j Aura graph database.
//! Critical for insider trading detection - identifies connected entities.
//!
//! ## External Service: Neo4j Aura (Free tier)
//! - Graph database optimized for relationship queries
//! - Uses Cypher query language via HTTP API
//! - Supports multi-hop relationship traversal
//!
//! ## Context Cache Feature:
//! - Implements fuzzy resolution for entity_id and company_symbol
//! - Cross-parameter resolution: if one param matches cache, related params are auto-filled
//! - Helps Icarus resolve ambiguous prompts like "that entity", "same company", etc.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use weil_macros::{constructor, mutate, query, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::http::{HttpClient, HttpMethod};
use weil_rs::runtime::Runtime;

// ===== CONFIGURATION =====

#[derive(Debug, Serialize, Deserialize, WeilType, Default, Clone)]
pub struct EntityRelationshipConfig {
    pub dashboard_contract_id: String,
    pub neo4j_uri: String,
    pub neo4j_user: String,
    pub neo4j_password: String,
}

// ===== DATA STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct Entity {
    pub entity_id: String,
    pub entity_type: String,
    pub name: String,
    pub pan_number: String,
    pub registration_id: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct Relationship {
    pub source_entity_id: String,
    pub target_entity_id: String,
    pub relationship_type: String,
    pub relationship_detail: String,
    pub strength: u32,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct EntityConnection {
    pub entity_id: String,
    pub connected_entity_id: String,
    pub connection_path: String,
    pub hops: u32,
    pub relationship_types: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct InsiderStatus {
    pub entity_id: String,
    pub company_symbol: String,
    pub is_insider: bool,
    pub insider_type: String,
    pub designation: String,
    pub window_status: String,
}

// ===== CONTEXT CACHE STRUCTURES =====

#[derive(Debug, Serialize, Deserialize, WeilType, Clone, Default)]
pub struct QueryHistory {
    pub method_name: String,
    pub entity_id: String,
    pub company_symbol: String,
    pub timestamp: u64,
    pub natural_language_prompt: String,
}

#[derive(Debug, Serialize, Deserialize, WeilType, Clone, Default)]
pub struct QueryContext {
    pub recent_queries: Vec<QueryHistory>,
    pub last_entity_id: String,
    pub last_company_symbol: String,
}

// Alert struct for dashboard push
#[derive(Debug, Serialize, Deserialize, WeilType, Clone)]
pub struct Alert {
    pub id: String,
    pub alert_type: String,
    pub severity: String,
    pub risk_score: u32,
    pub entity_id: String,
    pub symbol: String,
    pub description: String,
    pub workflow_id: String,
    pub timestamp: u64,
}

// Neo4j Query API v2 request/response structures
#[derive(Debug, Serialize, Deserialize)]
struct Neo4jQueryRequest {
    statement: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Neo4jResponse {
    data: Option<Neo4jData>,
    #[serde(default)]
    errors: Vec<Neo4jError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Neo4jData {
    fields: Vec<String>,
    values: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Neo4jError {
    message: String,
}

// ===== TRAIT DEFINITION =====

trait EntityRelationship {
    fn new() -> Result<Self, String> where Self: Sized;
    async fn get_context(&mut self) -> QueryContext;
    async fn get_entity(&mut self, entity_id: String) -> Result<Entity, String>;
    async fn search_entities(&mut self, search_query: String, limit: u32) -> Result<Vec<Entity>, String>;
    async fn get_relationships(&mut self, entity_id: String) -> Result<Vec<Relationship>, String>;
    async fn get_connected_entities(&mut self, entity_id: String, max_hops: u32) -> Result<Vec<EntityConnection>, String>;
    async fn check_insider_status(&mut self, entity_id: String, company_symbol: String) -> Result<InsiderStatus, String>;
    async fn get_company_insiders(&mut self, company_symbol: String) -> Result<Vec<InsiderStatus>, String>;
    async fn are_entities_connected(&mut self, entity_id_1: String, entity_id_2: String, max_hops: u32) -> Result<EntityConnection, String>;
    async fn get_family_members(&mut self, entity_id: String) -> Result<Vec<Entity>, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

// ===== CONTRACT STATE =====

#[derive(Serialize, Deserialize, WeilType)]
pub struct EntityRelationshipContractState {
    secrets: Secrets<EntityRelationshipConfig>,
    query_cache: QueryContext,
}

impl EntityRelationshipContractState {
    /// Execute a Cypher query against Neo4j Aura using Query API v2
    async fn execute_cypher(&self, cypher: &str) -> Result<Neo4jResponse, String> {
        let config = self.secrets.config();
        
        let uri = config.neo4j_uri
            .replace("neo4j+s://", "https://")
            .replace("neo4j://", "http://");
        let url = format!("{}/db/neo4j/query/v2", uri);
        
        let request_body = Neo4jQueryRequest {
            statement: cypher.to_string(),
        };
        
        let body = serde_json::to_string(&request_body)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;
        
        let auth = format!("{}:{}", config.neo4j_user, config.neo4j_password);
        let auth_encoded = base64_encode(&auth);
        
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Basic {}", auth_encoded));
        
        let response = HttpClient::request(&url, HttpMethod::Post)
            .headers(headers)
            .body(body)
            .send()
            .map_err(|e| format!("Neo4j request failed: {:?}", e))?;
        
        let status = response.status();
        let response_text = response.text();
        
        if status == 403 {
            return Err(format!("Neo4j authentication failed (403 Forbidden). Check credentials."));
        }
        
        if !(200..300).contains(&status) {
            return Err(format!("Neo4j HTTP {}: {}", status, response_text));
        }
        
        serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse Neo4j response: {} - Body: {}", e, response_text))
    }
    
    /// Parse entity from Neo4j row
    fn parse_entity(&self, row: &[serde_json::Value]) -> Option<Entity> {
        if row.len() >= 5 {
            Some(Entity {
                entity_id: row[0].as_str().unwrap_or("").to_string(),
                entity_type: row[1].as_str().unwrap_or("").to_string(),
                name: row[2].as_str().unwrap_or("").to_string(),
                pan_number: row[3].as_str().unwrap_or("").to_string(),
                registration_id: row[4].as_str().unwrap_or("").to_string(),
            })
        } else {
            None
        }
    }

    // ===== CACHE METHODS =====

    fn update_cache(&mut self, method_name: &str, entity_id: &str, company_symbol: &str, prompt: &str) {
        let already_exists = self.query_cache.recent_queries.iter()
            .any(|q| q.entity_id == entity_id && q.company_symbol == company_symbol);
        
        if !already_exists && (!entity_id.is_empty() || !company_symbol.is_empty()) {
            let timestamp = self.query_cache.recent_queries.len() as u64 + 1;
            
            if self.query_cache.recent_queries.len() >= 10 {
                self.query_cache.recent_queries.remove(0);
            }
            self.query_cache.recent_queries.push(QueryHistory {
                method_name: method_name.to_string(),
                entity_id: entity_id.to_string(),
                company_symbol: company_symbol.to_string(),
                timestamp,
                natural_language_prompt: prompt.to_string(),
            });
        }
        
        if !entity_id.is_empty() {
            self.query_cache.last_entity_id = entity_id.to_string();
        }
        if !company_symbol.is_empty() {
            self.query_cache.last_company_symbol = company_symbol.to_string();
        }
    }

    /// Resolve a partial entity_id from cache using fuzzy matching
    /// "REL-001" → "ENT-REL-001", "SUS" → "SUS-001"
    fn resolve_entity(&self, partial: &str) -> String {
        if partial.is_empty() {
            return self.query_cache.last_entity_id.clone();
        }
        
        let partial_lower = partial.to_lowercase();
        
        if self.query_cache.last_entity_id.to_lowercase().contains(&partial_lower) {
            return self.query_cache.last_entity_id.clone();
        }
        
        for query in self.query_cache.recent_queries.iter().rev() {
            if !query.entity_id.is_empty() && query.entity_id.to_lowercase().contains(&partial_lower) {
                return query.entity_id.clone();
            }
            if query.natural_language_prompt.to_lowercase().contains(&partial_lower) {
                if !query.entity_id.is_empty() {
                    return query.entity_id.clone();
                }
            }
        }
        
        partial.to_string()
    }

    /// Resolve a partial company_symbol from cache
    /// "RELI" → "RELIANCE", "INF" → "INFY"
    fn resolve_company(&self, partial: &str) -> String {
        if partial.is_empty() {
            return self.query_cache.last_company_symbol.clone();
        }
        
        let partial_lower = partial.to_lowercase();
        
        if self.query_cache.last_company_symbol.to_lowercase().contains(&partial_lower) {
            return self.query_cache.last_company_symbol.clone();
        }
        
        for query in self.query_cache.recent_queries.iter().rev() {
            if !query.company_symbol.is_empty() && query.company_symbol.to_lowercase().contains(&partial_lower) {
                return query.company_symbol.clone();
            }
        }
        
        partial.to_string()
    }

    /// Cross-parameter resolution
    fn resolve_from_cache(&self, entity_partial: &str, company_partial: &str) -> (String, String) {
        let entity_lower = entity_partial.to_lowercase();
        let company_lower = company_partial.to_lowercase();
        
        for query in self.query_cache.recent_queries.iter().rev() {
            let entity_matches = !entity_partial.is_empty() && 
                !query.entity_id.is_empty() && 
                query.entity_id.to_lowercase().contains(&entity_lower);
            
            let company_matches = !company_partial.is_empty() && 
                !query.company_symbol.is_empty() && 
                query.company_symbol.to_lowercase().contains(&company_lower);
            
            if entity_matches || company_matches {
                let resolved_entity = if query.entity_id.is_empty() {
                    self.resolve_entity(entity_partial)
                } else {
                    query.entity_id.clone()
                };
                
                let resolved_company = if query.company_symbol.is_empty() {
                    self.resolve_company(company_partial)
                } else {
                    query.company_symbol.clone()
                };
                
                return (resolved_entity, resolved_company);
            }
        }
        
        (self.resolve_entity(entity_partial), self.resolve_company(company_partial))
    }

    /// Push alert to surveillance dashboard via cross-contract call
    fn maybe_push_alert(&self, alert_type: &str, severity: &str, risk_score: u32, entity_id: &str, symbol: &str, description: &str) {
        let config = self.secrets.config();
        if config.dashboard_contract_id.is_empty() {
            return;
        }

        let alert = Alert {
            id: format!("ENTITY-{}", 0u64),
            alert_type: alert_type.to_string(),
            severity: severity.to_string(),
            risk_score,
            entity_id: entity_id.to_string(),
            symbol: symbol.to_string(),
            description: description.to_string(),
            workflow_id: "".to_string(),
            timestamp: 0,
        };

        let args = serde_json::to_string(&alert).unwrap_or_default();
        let _ = Runtime::call_contract::<String>(
            config.dashboard_contract_id.clone(),
            "push_alert".to_string(),
            Some(args),
        );
    }
}

/// Simple base64 encoding for auth
fn base64_encode(input: &str) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::new();
    
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).map(|&b| b as u32).unwrap_or(0);
        let b2 = chunk.get(2).map(|&b| b as u32).unwrap_or(0);
        
        let combined = (b0 << 16) | (b1 << 8) | b2;
        
        result.push(ALPHABET[((combined >> 18) & 0x3F) as usize] as char);
        result.push(ALPHABET[((combined >> 12) & 0x3F) as usize] as char);
        
        if chunk.len() > 1 {
            result.push(ALPHABET[((combined >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(ALPHABET[(combined & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}

// ===== CONTRACT IMPLEMENTATION =====

#[smart_contract]
impl EntityRelationship for EntityRelationshipContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        let sample_histories = vec![
            QueryHistory {
                method_name: "get_entity".to_string(),
                entity_id: "ENT-REL-001".to_string(),
                company_symbol: "RELIANCE".to_string(),
                timestamp: 1,
                natural_language_prompt: "Get Mukesh Ambani entity".to_string(),
            },
            QueryHistory {
                method_name: "get_company_insiders".to_string(),
                entity_id: "".to_string(),
                company_symbol: "INFY".to_string(),
                timestamp: 2,
                natural_language_prompt: "Get all Infosys insiders".to_string(),
            },
            QueryHistory {
                method_name: "check_insider_status".to_string(),
                entity_id: "SUS-001".to_string(),
                company_symbol: "RELIANCE".to_string(),
                timestamp: 3,
                natural_language_prompt: "Is suspect SUS-001 a RELIANCE insider?".to_string(),
            },
            QueryHistory {
                method_name: "get_relationships".to_string(),
                entity_id: "ENT-REL-006".to_string(),
                company_symbol: "".to_string(),
                timestamp: 4,
                natural_language_prompt: "Get relationships for Reliance CFO".to_string(),
            },
            QueryHistory {
                method_name: "get_company_insiders".to_string(),
                entity_id: "".to_string(),
                company_symbol: "TCS".to_string(),
                timestamp: 5,
                natural_language_prompt: "List TCS insiders".to_string(),
            },
        ];
        
        Ok(EntityRelationshipContractState {
            secrets: Secrets::new(),
            query_cache: QueryContext {
                recent_queries: sample_histories,
                last_entity_id: "ENT-REL-001".to_string(),
                last_company_symbol: "RELIANCE".to_string(),
            },
        })
    }

    #[mutate]
    async fn get_context(&mut self) -> QueryContext {
        self.query_cache.clone()
    }

    #[mutate]
    async fn get_entity(&mut self, entity_id: String) -> Result<Entity, String> {
        let resolved_entity = self.resolve_entity(&entity_id);
        self.update_cache("get_entity", &resolved_entity, "", 
            &format!("Get entity {}", resolved_entity));
        
        let cypher = format!(
            "MATCH (e:Entity {{entity_id: '{}'}}) RETURN e.entity_id, e.entity_type, e.name, e.pan_number, e.registration_id",
            resolved_entity
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        if let Some(ref data) = response.data {
            if let Some(row) = data.values.first() {
                if let Some(entity) = self.parse_entity(row) {
                    return Ok(entity);
                }
            }
        }
        
        Err(format!("Entity {} not found", resolved_entity))
    }

    #[mutate]
    async fn search_entities(&mut self, search_query: String, limit: u32) -> Result<Vec<Entity>, String> {
        self.update_cache("search_entities", "", "", 
            &format!("Search for {}", search_query));
        
        let cypher = format!(
            "MATCH (e:Entity) WHERE e.name CONTAINS '{}' OR e.pan_number CONTAINS '{}' RETURN e.entity_id, e.entity_type, e.name, e.pan_number, e.registration_id LIMIT {}",
            search_query, search_query, limit
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        let mut entities = Vec::new();
        if let Some(ref data) = response.data {
            for row in &data.values {
                if let Some(entity) = self.parse_entity(row) {
                    entities.push(entity);
                }
            }
        }
        
        Ok(entities)
    }

    #[mutate]
    async fn get_relationships(&mut self, entity_id: String) -> Result<Vec<Relationship>, String> {
        let resolved_entity = self.resolve_entity(&entity_id);
        self.update_cache("get_relationships", &resolved_entity, "", 
            &format!("Get relationships for {}", resolved_entity));
        
        let cypher = format!(
            "MATCH (a:Entity {{entity_id: '{}'}})-[r]->(b:Entity) RETURN a.entity_id, b.entity_id, type(r), r.detail, r.strength, r.verified",
            resolved_entity
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        let mut relationships = Vec::new();
        if let Some(ref data) = response.data {
            for row in &data.values {
                if row.len() >= 6 {
                    relationships.push(Relationship {
                        source_entity_id: row[0].as_str().unwrap_or("").to_string(),
                        target_entity_id: row[1].as_str().unwrap_or("").to_string(),
                        relationship_type: row[2].as_str().unwrap_or("").to_string(),
                        relationship_detail: row[3].as_str().unwrap_or("").to_string(),
                        strength: row[4].as_u64().unwrap_or(0) as u32,
                        verified: row[5].as_bool().unwrap_or(false),
                    });
                }
            }
        }
        
        Ok(relationships)
    }

    #[mutate]
    async fn get_connected_entities(&mut self, entity_id: String, max_hops: u32) -> Result<Vec<EntityConnection>, String> {
        let resolved_entity = self.resolve_entity(&entity_id);
        self.update_cache("get_connected_entities", &resolved_entity, "", 
            &format!("Get connected entities for {}", resolved_entity));
        
        let cypher = format!(
            "MATCH path = (a:Entity {{entity_id: '{}'}})-[*1..{}]-(b:Entity) WHERE a <> b RETURN DISTINCT b.entity_id, [n IN nodes(path) | n.entity_id] AS path_nodes, length(path) AS hops, [r IN relationships(path) | type(r)] AS rel_types LIMIT 50",
            resolved_entity, max_hops
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        let mut connections = Vec::new();
        if let Some(ref data) = response.data {
            for row in &data.values {
                if row.len() >= 4 {
                    let path_nodes: Vec<String> = row[1].as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_default();
                    let rel_types: Vec<String> = row[3].as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_default();
                    
                    connections.push(EntityConnection {
                        entity_id: resolved_entity.clone(),
                        connected_entity_id: row[0].as_str().unwrap_or("").to_string(),
                        connection_path: path_nodes.join(" -> "),
                        hops: row[2].as_u64().unwrap_or(0) as u32,
                        relationship_types: rel_types.join(","),
                    });
                }
            }
        }
        
        Ok(connections)
    }

    #[mutate]
    async fn check_insider_status(&mut self, entity_id: String, company_symbol: String) -> Result<InsiderStatus, String> {
        let (resolved_entity, resolved_company) = self.resolve_from_cache(&entity_id, &company_symbol);
        self.update_cache("check_insider_status", &resolved_entity, &resolved_company, 
            &format!("Check if {} is {} insider", resolved_entity, resolved_company));
        
        let cypher = format!(
            "MATCH (e:Entity {{entity_id: '{}'}})-[r:INSIDER_OF]->(c:Company {{symbol: '{}'}}) RETURN e.entity_id, c.symbol, true, r.insider_type, r.designation, r.window_status",
            resolved_entity, resolved_company
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        if let Some(ref data) = response.data {
            if let Some(row) = data.values.first() {
                if row.len() >= 6 {
                    let status = InsiderStatus {
                        entity_id: row[0].as_str().unwrap_or("").to_string(),
                        company_symbol: row[1].as_str().unwrap_or("").to_string(),
                        is_insider: row[2].as_bool().unwrap_or(false),
                        insider_type: row[3].as_str().unwrap_or("").to_string(),
                        designation: row[4].as_str().unwrap_or("").to_string(),
                        window_status: row[5].as_str().unwrap_or("OPEN").to_string(),
                    };
                    
                    // Push alert if confirmed insider
                    if status.is_insider {
                        self.maybe_push_alert(
                            "INSIDER_CONFIRMED",
                            "HIGH",
                            70,
                            &status.entity_id,
                            &status.company_symbol,
                            &format!("Confirmed insider: {} is {} ({}) for {}", status.entity_id, status.insider_type, status.designation, status.company_symbol),
                        );
                    }
                    
                    return Ok(status);
                }
            }
        }
        
        Ok(InsiderStatus {
            entity_id: resolved_entity,
            company_symbol: resolved_company,
            is_insider: false,
            insider_type: "".to_string(),
            designation: "".to_string(),
            window_status: "N/A".to_string(),
        })
    }

    #[mutate]
    async fn get_company_insiders(&mut self, company_symbol: String) -> Result<Vec<InsiderStatus>, String> {
        let resolved_company = self.resolve_company(&company_symbol);
        self.update_cache("get_company_insiders", "", &resolved_company, 
            &format!("Get insiders for {}", resolved_company));
        
        let cypher = format!(
            "MATCH (e:Entity)-[r:INSIDER_OF]->(c:Company {{symbol: '{}'}}) RETURN e.entity_id, c.symbol, true, r.insider_type, r.designation, r.window_status",
            resolved_company
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        let mut insiders = Vec::new();
        if let Some(ref data) = response.data {
            for row in &data.values {
                if row.len() >= 6 {
                    insiders.push(InsiderStatus {
                        entity_id: row[0].as_str().unwrap_or("").to_string(),
                        company_symbol: row[1].as_str().unwrap_or("").to_string(),
                        is_insider: true,
                        insider_type: row[3].as_str().unwrap_or("").to_string(),
                        designation: row[4].as_str().unwrap_or("").to_string(),
                        window_status: row[5].as_str().unwrap_or("OPEN").to_string(),
                    });
                }
            }
        }
        
        Ok(insiders)
    }

    #[mutate]
    async fn are_entities_connected(&mut self, entity_id_1: String, entity_id_2: String, max_hops: u32) -> Result<EntityConnection, String> {
        let resolved_entity_1 = self.resolve_entity(&entity_id_1);
        let resolved_entity_2 = self.resolve_entity(&entity_id_2);
        self.update_cache("are_entities_connected", &resolved_entity_1, "", 
            &format!("Check connection {} to {}", resolved_entity_1, resolved_entity_2));
        
        let cypher = format!(
            "MATCH path = shortestPath((a:Entity {{entity_id: '{}'}})-[*1..{}]-(b:Entity {{entity_id: '{}'}})) RETURN [n IN nodes(path) | n.entity_id] AS path_nodes, length(path) AS hops, [r IN relationships(path) | type(r)] AS rel_types",
            resolved_entity_1, max_hops, resolved_entity_2
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        if let Some(ref data) = response.data {
            if let Some(row) = data.values.first() {
                if row.len() >= 3 {
                    let path_nodes: Vec<String> = row[0].as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_default();
                    let rel_types: Vec<String> = row[2].as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_default();
                    
                    return Ok(EntityConnection {
                        entity_id: resolved_entity_1,
                        connected_entity_id: resolved_entity_2,
                        connection_path: path_nodes.join(" -> "),
                        hops: row[1].as_u64().unwrap_or(0) as u32,
                        relationship_types: rel_types.join(","),
                    });
                }
            }
        }
        
        Err(format!("No path found between {} and {} within {} hops", resolved_entity_1, resolved_entity_2, max_hops))
    }

    #[mutate]
    async fn get_family_members(&mut self, entity_id: String) -> Result<Vec<Entity>, String> {
        let resolved_entity = self.resolve_entity(&entity_id);
        self.update_cache("get_family_members", &resolved_entity, "", 
            &format!("Get family members of {}", resolved_entity));
        
        let cypher = format!(
            "MATCH (a:Entity {{entity_id: '{}'}})-[:FAMILY]-(b:Entity) RETURN b.entity_id, b.entity_type, b.name, b.pan_number, b.registration_id",
            resolved_entity
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        let mut entities = Vec::new();
        if let Some(ref data) = response.data {
            for row in &data.values {
                if let Some(entity) = self.parse_entity(row) {
                    entities.push(entity);
                }
            }
        }
        
        Ok(entities)
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_context",
      "description": "IMPORTANT: Call this FIRST before any other method. Returns recent query history with entity_ids and company_symbols to help resolve ambiguous user references like 'that entity', 'same company', etc.\n",
      "parameters": {
        "type": "object",
        "properties": {},
        "required": []
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_entity",
      "description": "Get entity details by ID from Neo4j - supports fuzzy matching\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity identifier (e.g., ENT-REL-001, SUS-001) - partial matches work\n"
          }
        },
        "required": [
          "entity_id"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "search_entities",
      "description": "Search entities by name or PAN in Neo4j\n",
      "parameters": {
        "type": "object",
        "properties": {
          "search_query": {
            "type": "string",
            "description": "Name or PAN number to search for\n"
          },
          "limit": {
            "type": "integer",
            "description": "Maximum number of results to return\n"
          }
        },
        "required": [
          "search_query",
          "limit"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_relationships",
      "description": "Get all relationships for an entity - supports fuzzy matching\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity identifier\n"
          }
        },
        "required": [
          "entity_id"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_connected_entities",
      "description": "Get entities connected within N hops for insider network mapping\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Starting entity ID - supports fuzzy matching\n"
          },
          "max_hops": {
            "type": "integer",
            "description": "Maximum hops to traverse (1-5)\n"
          }
        },
        "required": [
          "entity_id",
          "max_hops"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "check_insider_status",
      "description": "Check if an entity is a designated insider for a company - supports fuzzy matching\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity ID to check\n"
          },
          "company_symbol": {
            "type": "string",
            "description": "Stock symbol (e.g., RELIANCE, INFY)\n"
          }
        },
        "required": [
          "entity_id",
          "company_symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_company_insiders",
      "description": "Get all designated insiders for a company - supports fuzzy matching\n",
      "parameters": {
        "type": "object",
        "properties": {
          "company_symbol": {
            "type": "string",
            "description": "Stock symbol - partial matches work\n"
          }
        },
        "required": [
          "company_symbol"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "are_entities_connected",
      "description": "Find shortest path between two entities in the graph\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id_1": {
            "type": "string",
            "description": "First entity ID\n"
          },
          "entity_id_2": {
            "type": "string",
            "description": "Second entity ID\n"
          },
          "max_hops": {
            "type": "integer",
            "description": "Maximum hops to search (1-5)\n"
          }
        },
        "required": [
          "entity_id_1",
          "entity_id_2",
          "max_hops"
        ]
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "get_family_members",
      "description": "Get family members of an entity for insider detection\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity ID - supports fuzzy matching\n"
          }
        },
        "required": [
          "entity_id"
        ]
      }
    }
  }
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{
  "prompts": []
}"#.to_string()
    }
}
