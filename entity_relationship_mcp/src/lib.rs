//! # Entity Relationship MCP Server
//!
//! Maps relationships between entities using Neo4j Aura graph database.
//! Critical for insider trading detection - identifies connected entities.
//!
//! ## External Service: Neo4j Aura (Free tier)
//! - Graph database optimized for relationship queries
//! - Uses Cypher query language via HTTP API
//! - Supports multi-hop relationship traversal

use serde::{Deserialize, Serialize};
use weil_macros::{constructor, query, smart_contract, WeilType};
use weil_rs::config::Secrets;
use weil_rs::http::{HttpClient, HttpMethod};

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

// Neo4j API response structures
#[derive(Debug, Serialize, Deserialize)]
struct Neo4jRequest {
    statements: Vec<Neo4jStatement>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Neo4jStatement {
    statement: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Neo4jResponse {
    results: Vec<Neo4jResult>,
    errors: Vec<Neo4jError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Neo4jResult {
    columns: Vec<String>,
    data: Vec<Neo4jRow>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Neo4jRow {
    row: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Neo4jError {
    message: String,
}

// ===== TRAIT DEFINITION =====

trait EntityRelationship {
    fn new() -> Result<Self, String> where Self: Sized;
    async fn get_entity(&self, entity_id: String) -> Result<Entity, String>;
    async fn search_entities(&self, search_query: String, limit: u32) -> Result<Vec<Entity>, String>;
    async fn get_relationships(&self, entity_id: String) -> Result<Vec<Relationship>, String>;
    async fn get_connected_entities(&self, entity_id: String, max_hops: u32) -> Result<Vec<EntityConnection>, String>;
    async fn check_insider_status(&self, entity_id: String, company_symbol: String) -> Result<InsiderStatus, String>;
    async fn get_company_insiders(&self, company_symbol: String) -> Result<Vec<InsiderStatus>, String>;
    async fn are_entities_connected(&self, entity_id_1: String, entity_id_2: String, max_hops: u32) -> Result<EntityConnection, String>;
    async fn get_family_members(&self, entity_id: String) -> Result<Vec<Entity>, String>;
    fn tools(&self) -> String;
    fn prompts(&self) -> String;
}

// ===== CONTRACT STATE =====

#[derive(Serialize, Deserialize, WeilType)]
pub struct EntityRelationshipContractState {
    secrets: Secrets<EntityRelationshipConfig>,
}

impl EntityRelationshipContractState {
    /// Execute a Cypher query against Neo4j Aura
    async fn execute_cypher(&self, cypher: &str) -> Result<Neo4jResponse, String> {
        let config = self.secrets.config();
        
        // Neo4j Aura HTTP API endpoint
        // Convert neo4j+s://xxx.databases.neo4j.io to https://xxx.databases.neo4j.io:7473/db/neo4j/tx/commit
        let uri = config.neo4j_uri
            .replace("neo4j+s://", "https://")
            .replace("neo4j://", "http://");
        let url = format!("{}/db/neo4j/tx/commit", uri);
        
        let request_body = Neo4jRequest {
            statements: vec![Neo4jStatement {
                statement: cypher.to_string(),
            }],
        };
        
        let body = serde_json::to_string(&request_body)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;
        
        // Create Basic auth header
        let auth = format!("{}:{}", config.neo4j_user, config.neo4j_password);
        let auth_encoded = base64_encode(&auth);
        
        let response = HttpClient::request(&url, HttpMethod::Post)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Basic {}", auth_encoded))
            .body(body)
            .send()
            .map_err(|e| format!("Neo4j request failed: {:?}", e))?;
        
        let response_text = response.text();
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
        Ok(EntityRelationshipContractState {
            secrets: Secrets::new(),
        })
    }

    /// Get entity details by ID from Neo4j
    #[query]
    async fn get_entity(&self, entity_id: String) -> Result<Entity, String> {
        let cypher = format!(
            "MATCH (e:Entity {{entity_id: '{}'}}) RETURN e.entity_id, e.entity_type, e.name, e.pan_number, e.registration_id",
            entity_id
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        if let Some(result) = response.results.first() {
            if let Some(row) = result.data.first() {
                if let Some(entity) = self.parse_entity(&row.row) {
                    return Ok(entity);
                }
            }
        }
        
        Err(format!("Entity {} not found", entity_id))
    }

    /// Search entities by name or PAN in Neo4j
    #[query]
    async fn search_entities(&self, search_query: String, limit: u32) -> Result<Vec<Entity>, String> {
        let cypher = format!(
            "MATCH (e:Entity) WHERE e.name CONTAINS '{}' OR e.pan_number CONTAINS '{}' RETURN e.entity_id, e.entity_type, e.name, e.pan_number, e.registration_id LIMIT {}",
            search_query, search_query, limit
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        let mut entities = Vec::new();
        if let Some(result) = response.results.first() {
            for row in &result.data {
                if let Some(entity) = self.parse_entity(&row.row) {
                    entities.push(entity);
                }
            }
        }
        
        Ok(entities)
    }

    /// Get all relationships for an entity from Neo4j graph
    #[query]
    async fn get_relationships(&self, entity_id: String) -> Result<Vec<Relationship>, String> {
        let cypher = format!(
            "MATCH (a:Entity {{entity_id: '{}'}})-[r]->(b:Entity) RETURN a.entity_id, b.entity_id, type(r), r.detail, r.strength, r.verified",
            entity_id
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        let mut relationships = Vec::new();
        if let Some(result) = response.results.first() {
            for row in &result.data {
                if row.row.len() >= 6 {
                    relationships.push(Relationship {
                        source_entity_id: row.row[0].as_str().unwrap_or("").to_string(),
                        target_entity_id: row.row[1].as_str().unwrap_or("").to_string(),
                        relationship_type: row.row[2].as_str().unwrap_or("").to_string(),
                        relationship_detail: row.row[3].as_str().unwrap_or("").to_string(),
                        strength: row.row[4].as_u64().unwrap_or(0) as u32,
                        verified: row.row[5].as_bool().unwrap_or(false),
                    });
                }
            }
        }
        
        Ok(relationships)
    }

    /// Get connected entities within N hops using Neo4j graph traversal
    #[query]
    async fn get_connected_entities(&self, entity_id: String, max_hops: u32) -> Result<Vec<EntityConnection>, String> {
        let cypher = format!(
            "MATCH path = (a:Entity {{entity_id: '{}'}})-[*1..{}]-(b:Entity) WHERE a <> b RETURN DISTINCT b.entity_id, [n IN nodes(path) | n.entity_id] AS path_nodes, length(path) AS hops, [r IN relationships(path) | type(r)] AS rel_types LIMIT 50",
            entity_id, max_hops
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        let mut connections = Vec::new();
        if let Some(result) = response.results.first() {
            for row in &result.data {
                if row.row.len() >= 4 {
                    let path_nodes: Vec<String> = row.row[1].as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_default();
                    let rel_types: Vec<String> = row.row[3].as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_default();
                    
                    connections.push(EntityConnection {
                        entity_id: entity_id.clone(),
                        connected_entity_id: row.row[0].as_str().unwrap_or("").to_string(),
                        connection_path: path_nodes.join(" -> "),
                        hops: row.row[2].as_u64().unwrap_or(0) as u32,
                        relationship_types: rel_types.join(","),
                    });
                }
            }
        }
        
        Ok(connections)
    }

    /// Check if entity is an insider for a company
    #[query]
    async fn check_insider_status(&self, entity_id: String, company_symbol: String) -> Result<InsiderStatus, String> {
        let cypher = format!(
            "MATCH (e:Entity {{entity_id: '{}'}})-[r:INSIDER_OF]->(c:Company {{symbol: '{}'}}) RETURN e.entity_id, c.symbol, true, r.insider_type, r.designation, r.window_status",
            entity_id, company_symbol
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        if let Some(result) = response.results.first() {
            if let Some(row) = result.data.first() {
                if row.row.len() >= 6 {
                    return Ok(InsiderStatus {
                        entity_id: row.row[0].as_str().unwrap_or("").to_string(),
                        company_symbol: row.row[1].as_str().unwrap_or("").to_string(),
                        is_insider: row.row[2].as_bool().unwrap_or(false),
                        insider_type: row.row[3].as_str().unwrap_or("").to_string(),
                        designation: row.row[4].as_str().unwrap_or("").to_string(),
                        window_status: row.row[5].as_str().unwrap_or("OPEN").to_string(),
                    });
                }
            }
        }
        
        // Not an insider
        Ok(InsiderStatus {
            entity_id,
            company_symbol,
            is_insider: false,
            insider_type: "".to_string(),
            designation: "".to_string(),
            window_status: "N/A".to_string(),
        })
    }

    /// Get all insiders for a company from Neo4j
    #[query]
    async fn get_company_insiders(&self, company_symbol: String) -> Result<Vec<InsiderStatus>, String> {
        let cypher = format!(
            "MATCH (e:Entity)-[r:INSIDER_OF]->(c:Company {{symbol: '{}'}}) RETURN e.entity_id, c.symbol, true, r.insider_type, r.designation, r.window_status",
            company_symbol
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        let mut insiders = Vec::new();
        if let Some(result) = response.results.first() {
            for row in &result.data {
                if row.row.len() >= 6 {
                    insiders.push(InsiderStatus {
                        entity_id: row.row[0].as_str().unwrap_or("").to_string(),
                        company_symbol: row.row[1].as_str().unwrap_or("").to_string(),
                        is_insider: true,
                        insider_type: row.row[3].as_str().unwrap_or("").to_string(),
                        designation: row.row[4].as_str().unwrap_or("").to_string(),
                        window_status: row.row[5].as_str().unwrap_or("OPEN").to_string(),
                    });
                }
            }
        }
        
        Ok(insiders)
    }

    /// Check if two entities are connected using Neo4j shortest path
    #[query]
    async fn are_entities_connected(&self, entity_id_1: String, entity_id_2: String, max_hops: u32) -> Result<EntityConnection, String> {
        let cypher = format!(
            "MATCH path = shortestPath((a:Entity {{entity_id: '{}'}})-[*1..{}]-(b:Entity {{entity_id: '{}'}})) RETURN [n IN nodes(path) | n.entity_id] AS path_nodes, length(path) AS hops, [r IN relationships(path) | type(r)] AS rel_types",
            entity_id_1, max_hops, entity_id_2
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        if let Some(result) = response.results.first() {
            if let Some(row) = result.data.first() {
                if row.row.len() >= 3 {
                    let path_nodes: Vec<String> = row.row[0].as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_default();
                    let rel_types: Vec<String> = row.row[2].as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_default();
                    
                    return Ok(EntityConnection {
                        entity_id: entity_id_1,
                        connected_entity_id: entity_id_2,
                        connection_path: path_nodes.join(" -> "),
                        hops: row.row[1].as_u64().unwrap_or(0) as u32,
                        relationship_types: rel_types.join(","),
                    });
                }
            }
        }
        
        Err(format!("No path found between {} and {} within {} hops", entity_id_1, entity_id_2, max_hops))
    }

    /// Get family members of an entity
    #[query]
    async fn get_family_members(&self, entity_id: String) -> Result<Vec<Entity>, String> {
        let cypher = format!(
            "MATCH (a:Entity {{entity_id: '{}'}})-[:FAMILY]-(b:Entity) RETURN b.entity_id, b.entity_type, b.name, b.pan_number, b.registration_id",
            entity_id
        );
        
        let response = self.execute_cypher(&cypher).await?;
        
        if !response.errors.is_empty() {
            return Err(response.errors[0].message.clone());
        }
        
        let mut entities = Vec::new();
        if let Some(result) = response.results.first() {
            for row in &result.data {
                if let Some(entity) = self.parse_entity(&row.row) {
                    entities.push(entity);
                }
            }
        }
        
        Ok(entities)
    }

    #[query]
    fn tools(&self) -> String {
        r#"[
  {"type": "function", "function": {"name": "get_entity", "description": "Get entity details by ID from Neo4j graph database", "parameters": {"type": "object", "properties": {"entity_id": {"type": "string", "description": "Entity identifier"}}, "required": ["entity_id"]}}},
  {"type": "function", "function": {"name": "search_entities", "description": "Search entities by name or PAN in Neo4j", "parameters": {"type": "object", "properties": {"search_query": {"type": "string", "description": "Name or PAN to search"}, "limit": {"type": "integer", "description": "Max results"}}, "required": ["search_query", "limit"]}}},
  {"type": "function", "function": {"name": "get_relationships", "description": "Get all relationships for an entity from the graph", "parameters": {"type": "object", "properties": {"entity_id": {"type": "string"}}, "required": ["entity_id"]}}},
  {"type": "function", "function": {"name": "get_connected_entities", "description": "Get entities connected within N hops (for insider network mapping)", "parameters": {"type": "object", "properties": {"entity_id": {"type": "string"}, "max_hops": {"type": "integer", "description": "1-5 hops"}}, "required": ["entity_id", "max_hops"]}}},
  {"type": "function", "function": {"name": "check_insider_status", "description": "Check if entity is an insider for a company", "parameters": {"type": "object", "properties": {"entity_id": {"type": "string"}, "company_symbol": {"type": "string"}}, "required": ["entity_id", "company_symbol"]}}},
  {"type": "function", "function": {"name": "get_company_insiders", "description": "Get all insiders for a company", "parameters": {"type": "object", "properties": {"company_symbol": {"type": "string"}}, "required": ["company_symbol"]}}},
  {"type": "function", "function": {"name": "are_entities_connected", "description": "Find shortest path between two entities", "parameters": {"type": "object", "properties": {"entity_id_1": {"type": "string"}, "entity_id_2": {"type": "string"}, "max_hops": {"type": "integer"}}, "required": ["entity_id_1", "entity_id_2", "max_hops"]}}},
  {"type": "function", "function": {"name": "get_family_members", "description": "Get family members of an entity", "parameters": {"type": "object", "properties": {"entity_id": {"type": "string"}}, "required": ["entity_id"]}}}
]"#.to_string()
    }

    #[query]
    fn prompts(&self) -> String {
        r#"{ "prompts": [] }"#.to_string()
    }
}
