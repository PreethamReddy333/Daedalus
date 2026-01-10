
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


trait Counter {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn get_count(&self) -> u32;
    async fn increment(&mut self);
    async fn set_value(&mut self, val: u32);
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct CounterContractState {
    // define your contract state here!
}

#[smart_contract]
impl Counter for CounterContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!();
    }


    #[query]
    async fn get_count(&self) -> u32 {
        unimplemented!();
    }

    #[mutate]
    async fn increment(&mut self) {
        unimplemented!();
    }

    #[mutate]
    async fn set_value(&mut self, val: u32) {
        unimplemented!();
    }
}

