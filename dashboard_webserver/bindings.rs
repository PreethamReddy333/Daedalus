
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


#[derive(Debug, Serialize, Deserialize, WeilType, Default)]
pub struct WebServerConfig {
    pub name: String,
}

trait DashboardWebserver {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn version(&self) -> String;

    // webserver specific functions
    fn start_file_upload(&mut self, path: String, total_chunks: u32) -> Result<(), String>;
    fn add_path_content(
        &mut self,
        path: String,
        chunk: Vec<u8>,
        index: u32,
    ) -> Result<(), String>;
    fn finish_upload(&mut self, path: String, size_bytes: u32) -> Result<(), String>;
    fn total_chunks(&self, path: String) -> Result<u32, String>;
    fn http_content(
        &self,
        path: String,
        index: u32,
        method: String,
    ) -> (u16, std::collections::HashMap<String, String>, Vec<u8>);
    fn size_bytes(&self, path: String) -> Result<u32, String>;
    fn get_chunk_size(&self) -> u32;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct DashboardWebserverContractState {
    // define your contract state here!
    secrets: Secrets<WebServerConfig>,
    server: WebServer,
}

#[smart_contract]
impl DashboardWebserver for DashboardWebserverContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!();
    }


    #[query]
    async fn version(&self) -> String {
        unimplemented!();
    }

    #[mutate]
    fn start_file_upload(&mut self, path: String, total_chunks: u32) -> Result<(), String> {
        self.server.start_file_upload(path, total_chunks)
    }

    #[query]
    fn total_chunks(&self, path: String) -> Result<u32, String> {
        self.server.total_chunks(path)
    }

    #[mutate]
    fn add_path_content(
        &mut self,
        path: String,
        chunk: Vec<u8>,
        index: u32,
    ) -> Result<(), String> {
        self.server.add_path_content(path, chunk, index)
    }

    #[mutate]
    fn finish_upload(&mut self, path: String, size_bytes: u32) -> Result<(), String> {
        self.server.finish_upload(path, size_bytes)
    }

    #[query]
    fn http_content(
        &self,
        path: String,
        index: u32,
        method: String,
    ) -> (u16, std::collections::HashMap<String, String>, Vec<u8>) {
        self.server.http_content(path, index, method)
    }

    #[query]
    fn size_bytes(&self, path: String) -> Result<u32, String> {
        self.server.size_bytes(path)
    }

    #[query]
    fn get_chunk_size(&self) -> u32 {
        self.server.get_chunk_size()
    }
}

