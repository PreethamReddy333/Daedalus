
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


#[derive(Debug, Serialize, Deserialize, WeilType, Default)]
pub struct EntityRelationshipConfig {
    pub dashboard_contract_id: String,
    pub neo4j_uri: String,
    pub neo4j_user: String,
    pub neo4j_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub entity_id: String,
    pub entity_type: String,
    pub name: String,
    pub pan_number: String,
    pub registration_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub source_entity_id: String,
    pub target_entity_id: String,
    pub relationship_type: String,
    pub relationship_detail: String,
    pub strength: u32,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityConnection {
    pub entity_id: String,
    pub connected_entity_id: String,
    pub connection_path: String,
    pub hops: u32,
    pub relationship_types: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsiderStatus {
    pub entity_id: String,
    pub company_symbol: String,
    pub is_insider: bool,
    pub insider_type: String,
    pub designation: String,
    pub window_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryHistory {
    pub method_name: String,
    pub entity_id: String,
    pub company_symbol: String,
    pub timestamp: u64,
    pub natural_language_prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryContext {
    pub recent_queries: Vec<QueryHistory>,
    pub last_entity_id: String,
    pub last_company_symbol: String,
}

trait EntityRelationship {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
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

#[derive(Serialize, Deserialize, WeilType)]
pub struct EntityRelationshipContractState {
    // define your contract state here!
    secrets: Secrets<EntityRelationshipConfig>,
}

#[smart_contract]
impl EntityRelationship for EntityRelationshipContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!();
    }


    #[mutate]
    async fn get_context(&mut self) -> QueryContext {
        unimplemented!();
    }

    #[mutate]
    async fn get_entity(&mut self, entity_id: String) -> Result<Entity, String> {
        unimplemented!();
    }

    #[mutate]
    async fn search_entities(&mut self, search_query: String, limit: u32) -> Result<Vec<Entity>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_relationships(&mut self, entity_id: String) -> Result<Vec<Relationship>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_connected_entities(&mut self, entity_id: String, max_hops: u32) -> Result<Vec<EntityConnection>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn check_insider_status(&mut self, entity_id: String, company_symbol: String) -> Result<InsiderStatus, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_company_insiders(&mut self, company_symbol: String) -> Result<Vec<InsiderStatus>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn are_entities_connected(&mut self, entity_id_1: String, entity_id_2: String, max_hops: u32) -> Result<EntityConnection, String> {
        unimplemented!();
    }

    #[mutate]
    async fn get_family_members(&mut self, entity_id: String) -> Result<Vec<Entity>, String> {
        unimplemented!();
    }


    #[query]
    fn tools(&self) -> String {
        r#"[
  {
    "type": "function",
    "function": {
      "name": "get_context",
      "description": "CALL THIS FIRST - Get context from recent queries\nReturns cached entity_ids and company_symbols for fuzzy resolution\n",
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
      "description": "Get entity details by ID from Neo4j\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity identifier - supports fuzzy matching\n"
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
            "description": "Search query (name or PAN)\n"
          },
          "limit": {
            "type": "integer",
            "description": "Maximum results\n"
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
      "description": "Get all relationships for an entity from Neo4j graph\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity to get relationships for - supports fuzzy matching\n"
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
      "description": "Get connected entities within N hops using Neo4j graph traversal\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Starting entity - supports fuzzy matching\n"
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
      "description": "Check if entity is an insider for a company\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id": {
            "type": "string",
            "description": "Entity to check - supports fuzzy matching\n"
          },
          "company_symbol": {
            "type": "string",
            "description": "Company symbol - supports fuzzy matching\n"
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
      "description": "Get all insiders for a company from Neo4j\n",
      "parameters": {
        "type": "object",
        "properties": {
          "company_symbol": {
            "type": "string",
            "description": "Company symbol - supports fuzzy matching\n"
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
      "description": "Check if two entities are connected using Neo4j shortest path\n",
      "parameters": {
        "type": "object",
        "properties": {
          "entity_id_1": {
            "type": "string",
            "description": "First entity - supports fuzzy matching\n"
          },
          "entity_id_2": {
            "type": "string",
            "description": "Second entity - supports fuzzy matching\n"
          },
          "max_hops": {
            "type": "integer",
            "description": "Maximum hops to check\n"
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
      "description": "Get family members of an entity\n",
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

