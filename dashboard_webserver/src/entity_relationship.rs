
use serde::{Deserialize, Serialize};
use anyhow::Result;
use weil_rs::runtime::Runtime;


#[derive(Debug, Serialize, Deserialize, Clone)]
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


pub struct EntityRelationshipProxy {
    contract_id: String,
}

impl EntityRelationshipProxy {
    pub fn new(contract_id: String) -> Self {
        EntityRelationshipProxy {
            contract_id,
        }
    }
}

impl EntityRelationshipProxy {
    pub fn get_context(&self) -> Result<QueryContext> {
        let serialized_args = None;

        let resp = Runtime::call_contract::<QueryContext>(
            self.contract_id.to_string(),
            "get_context".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_entity(&self, entity_id: String) -> Result<Entity> {

        #[derive(Debug, Serialize)]
        struct get_entityArgs {
            entity_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&get_entityArgs { entity_id }).unwrap());

        let resp = Runtime::call_contract::<Entity>(
            self.contract_id.to_string(),
            "get_entity".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn search_entities(&self, search_query: String, limit: u32) -> Result<Vec<Entity>> {

        #[derive(Debug, Serialize)]
        struct search_entitiesArgs {
            search_query: String,
            limit: u32,
        }

        let serialized_args = Some(serde_json::to_string(&search_entitiesArgs { search_query, limit }).unwrap());

        let resp = Runtime::call_contract::<Vec<Entity>>(
            self.contract_id.to_string(),
            "search_entities".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_relationships(&self, entity_id: String) -> Result<Vec<Relationship>> {

        #[derive(Debug, Serialize)]
        struct get_relationshipsArgs {
            entity_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&get_relationshipsArgs { entity_id }).unwrap());

        let resp = Runtime::call_contract::<Vec<Relationship>>(
            self.contract_id.to_string(),
            "get_relationships".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_connected_entities(&self, entity_id: String, max_hops: u32) -> Result<Vec<EntityConnection>> {

        #[derive(Debug, Serialize)]
        struct get_connected_entitiesArgs {
            entity_id: String,
            max_hops: u32,
        }

        let serialized_args = Some(serde_json::to_string(&get_connected_entitiesArgs { entity_id, max_hops }).unwrap());

        let resp = Runtime::call_contract::<Vec<EntityConnection>>(
            self.contract_id.to_string(),
            "get_connected_entities".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn check_insider_status(&self, entity_id: String, company_symbol: String) -> Result<InsiderStatus> {

        #[derive(Debug, Serialize)]
        struct check_insider_statusArgs {
            entity_id: String,
            company_symbol: String,
        }

        let serialized_args = Some(serde_json::to_string(&check_insider_statusArgs { entity_id, company_symbol }).unwrap());

        let resp = Runtime::call_contract::<InsiderStatus>(
            self.contract_id.to_string(),
            "check_insider_status".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_company_insiders(&self, company_symbol: String) -> Result<Vec<InsiderStatus>> {

        #[derive(Debug, Serialize)]
        struct get_company_insidersArgs {
            company_symbol: String,
        }

        let serialized_args = Some(serde_json::to_string(&get_company_insidersArgs { company_symbol }).unwrap());

        let resp = Runtime::call_contract::<Vec<InsiderStatus>>(
            self.contract_id.to_string(),
            "get_company_insiders".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn are_entities_connected(&self, entity_id_1: String, entity_id_2: String, max_hops: u32) -> Result<EntityConnection> {

        #[derive(Debug, Serialize)]
        struct are_entities_connectedArgs {
            entity_id_1: String,
            entity_id_2: String,
            max_hops: u32,
        }

        let serialized_args = Some(serde_json::to_string(&are_entities_connectedArgs { entity_id_1, entity_id_2, max_hops }).unwrap());

        let resp = Runtime::call_contract::<EntityConnection>(
            self.contract_id.to_string(),
            "are_entities_connected".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

    pub fn get_family_members(&self, entity_id: String) -> Result<Vec<Entity>> {

        #[derive(Debug, Serialize)]
        struct get_family_membersArgs {
            entity_id: String,
        }

        let serialized_args = Some(serde_json::to_string(&get_family_membersArgs { entity_id }).unwrap());

        let resp = Runtime::call_contract::<Vec<Entity>>(
            self.contract_id.to_string(),
            "get_family_members".to_string(),
            serialized_args,
        )?;

        Ok(resp)
    }

}
