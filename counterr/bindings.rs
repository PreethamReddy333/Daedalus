
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


trait Counterr {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn add(&self, x: i32, y: i32) -> i32;
    async fn multiply(&self, x: i32, y: i32) -> i32;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct CounterrContractState {
    // define your contract state here!
}

#[smart_contract]
impl Counterr for CounterrContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!();
    }


    #[query]
    async fn add(&self, x: i32, y: i32) -> i32 {
        unimplemented!();
    }

    #[query]
    async fn multiply(&self, x: i32, y: i32) -> i32 {
        unimplemented!();
    }
}

