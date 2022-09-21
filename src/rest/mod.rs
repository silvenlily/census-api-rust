use std::{sync::Arc, time::SystemTime};

pub const CENSUS_URL: &str = "https://census.daybreakgames.com/";

pub mod api;
pub mod query_builder;

pub mod character;
pub mod item;
pub mod outfit;

#[derive(Clone)]
pub(crate) struct CensusValue<T: Clone> {
    pub last_updated: Option<SystemTime>,
    pub value: Option<T>,
}

impl<T: Clone> CensusValue<T> {
    pub fn update(&mut self, value: T) {
        self.value = Some(value);
        self.last_updated = Some(SystemTime::now())
    }
    pub fn new_empty<V: Clone>() -> CensusValue<V> {
        CensusValue {
            last_updated: None,
            value: None,
        }
    }
    pub fn new_value<V: Clone>(value: V) -> CensusValue<V> {
        CensusValue {
            value: Some(value),
            last_updated: Some(SystemTime::now()),
        }
    }
}

pub struct LocalisedString {
    pub de: Option<String>,
    pub en: Option<String>,
    pub es: Option<String>,
    pub fr: Option<String>,
    pub it: Option<String>,
    pub tr: Option<String>,
}

pub struct RestClient {
    pub endpoint: String,
    pub(crate) serviceid: String,
    pub reqwest_client: Arc<reqwest::Client>,
}

impl RestClient {
    pub fn new(serviceid: String) -> RestClient {
        return RestClient {
            endpoint: String::from("ps2:v2"),
            serviceid: serviceid,
            reqwest_client: Arc::new(reqwest::Client::new()),
        };
    }

    pub fn new_with_reqwest(serviceid: String, reqwest_client: Arc<reqwest::Client>) -> RestClient {
        return RestClient {
            endpoint: String::from("ps2:v2"),
            serviceid: serviceid,
            reqwest_client: reqwest_client,
        };
    }

    pub fn get_query_builder(&self, collection: &str) -> query_builder::QueryBuilder {
        return query_builder::QueryBuilder::new(
            self.serviceid.clone(),
            self.endpoint.clone(),
            self.reqwest_client.clone(),
            collection.to_string(),
        );
    }
}
