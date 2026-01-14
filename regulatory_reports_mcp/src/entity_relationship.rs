//! Cross-contract bindings for Entity Relationship MCP
//!
//! Provides proxy methods to call the deployed Entity Relationship MCP contract.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use weil_rs::runtime::Runtime;

/// Proxy struct for Entity Relationship MCP cross-contract calls
pub struct EntityRelationshipMcp {
    contract_id: String,
}

impl EntityRelationshipMcp {
    pub fn new(contract_id: String) -> Self {
        EntityRelationshipMcp { contract_id }
    }
}

// ===== Response Types =====

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entity {
    pub entity_id: String,
    pub entity_type: String,
    pub name: String,
    pub pan_number: String,
    pub registration_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Relationship {
    pub source_entity_id: String,
    pub target_entity_id: String,
    pub relationship_type: String,
    pub relationship_detail: String,
    pub strength: u32,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityConnection {
    pub entity_id: String,
    pub connected_entity_id: String,
    pub connection_path: String,
    pub hops: u32,
    pub relationship_types: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsiderStatus {
    pub entity_id: String,
    pub company_symbol: String,
    pub is_insider: bool,
    pub insider_type: String,
    pub designation: String,
    pub window_status: String,
}

impl EntityRelationshipMcp {
    /// Get entity details by ID from Neo4j
    pub fn get_entity(&self, entity_id: String) -> Result<Entity> {
        #[derive(Debug, Serialize)]
        struct GetEntityArgs {
            entity_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&GetEntityArgs { entity_id })?);

        let resp = Runtime::call_contract::<Entity>(
            self.contract_id.clone(),
            "get_entity".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Search entities by name or PAN in Neo4j
    pub fn search_entities(&self, search_query: String, limit: u32) -> Result<Vec<Entity>> {
        #[derive(Debug, Serialize)]
        struct SearchEntitiesArgs {
            search_query: String,
            limit: u32,
        }

        let serialized_args = Some(serde_json::to_string(&SearchEntitiesArgs {
            search_query,
            limit,
        })?);

        let resp = Runtime::call_contract::<Vec<Entity>>(
            self.contract_id.clone(),
            "search_entities".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Get all relationships for an entity from Neo4j graph
    pub fn get_relationships(&self, entity_id: String) -> Result<Vec<Relationship>> {
        #[derive(Debug, Serialize)]
        struct GetRelationshipsArgs {
            entity_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&GetRelationshipsArgs { entity_id })?);

        let resp = Runtime::call_contract::<Vec<Relationship>>(
            self.contract_id.clone(),
            "get_relationships".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Get connected entities within N hops using Neo4j graph traversal
    pub fn get_connected_entities(&self, entity_id: String, max_hops: u32) -> Result<Vec<EntityConnection>> {
        #[derive(Debug, Serialize)]
        struct GetConnectedEntitiesArgs {
            entity_id: String,
            max_hops: u32,
        }

        let serialized_args = Some(serde_json::to_string(&GetConnectedEntitiesArgs {
            entity_id,
            max_hops,
        })?);

        let resp = Runtime::call_contract::<Vec<EntityConnection>>(
            self.contract_id.clone(),
            "get_connected_entities".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Check if entity is an insider for a company
    pub fn check_insider_status(&self, entity_id: String, company_symbol: String) -> Result<InsiderStatus> {
        #[derive(Debug, Serialize)]
        struct CheckInsiderStatusArgs {
            entity_id: String,
            company_symbol: String,
        }

        let serialized_args = Some(serde_json::to_string(&CheckInsiderStatusArgs {
            entity_id,
            company_symbol,
        })?);

        let resp = Runtime::call_contract::<InsiderStatus>(
            self.contract_id.clone(),
            "check_insider_status".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Get all insiders for a company from Neo4j
    pub fn get_company_insiders(&self, company_symbol: String) -> Result<Vec<InsiderStatus>> {
        #[derive(Debug, Serialize)]
        struct GetCompanyInsidersArgs {
            company_symbol: String,
        }

        let serialized_args = Some(serde_json::to_string(&GetCompanyInsidersArgs { company_symbol })?);

        let resp = Runtime::call_contract::<Vec<InsiderStatus>>(
            self.contract_id.clone(),
            "get_company_insiders".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Check if two entities are connected using Neo4j shortest path
    pub fn are_entities_connected(
        &self,
        entity_id_1: String,
        entity_id_2: String,
        max_hops: u32,
    ) -> Result<EntityConnection> {
        #[derive(Debug, Serialize)]
        struct AreEntitiesConnectedArgs {
            entity_id_1: String,
            entity_id_2: String,
            max_hops: u32,
        }

        let serialized_args = Some(serde_json::to_string(&AreEntitiesConnectedArgs {
            entity_id_1,
            entity_id_2,
            max_hops,
        })?);

        let resp = Runtime::call_contract::<EntityConnection>(
            self.contract_id.clone(),
            "are_entities_connected".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    /// Get family members of an entity
    pub fn get_family_members(&self, entity_id: String) -> Result<Vec<Entity>> {
        #[derive(Debug, Serialize)]
        struct GetFamilyMembersArgs {
            entity_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&GetFamilyMembersArgs { entity_id })?);

        let resp = Runtime::call_contract::<Vec<Entity>>(
            self.contract_id.clone(),
            "get_family_members".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }
}
