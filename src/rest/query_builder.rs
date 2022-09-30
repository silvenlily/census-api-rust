use std::sync::Arc;

use serde_json::Value;

use crate::utils::CensusError;

pub trait Resolveable {
    fn from_resolve_string(resolve: &str) -> Option<Self>
    where
        Self: Sized;
    fn to_resolve_string(&self) -> String;
}

#[derive(Clone)]
pub struct QueryBuilder {
    collection: String,
    serviceid: String,
    endpoint: String,
    pub reqwest_client: Arc<reqwest::Client>,
    resolves_vec: Vec<String>,
    search_vec: Vec<(String, String)>,
    limit: u64,
    start: u64,
    lang: String,
    should_retry: bool,
}

impl QueryBuilder {
    pub fn new(
        serviceid: String,
        endpoint: String,
        reqwest_client: Arc<reqwest::Client>,
        collection: String,
    ) -> QueryBuilder {
        return QueryBuilder {
            collection,
            serviceid,
            endpoint,
            reqwest_client,
            resolves_vec: Vec::new(),
            search_vec: Vec::new(),
            limit: 0,
            lang: crate::rest::api::langs::ENGLISH.to_string(),
            start: 0,
            should_retry: false,
        };
    }

    pub fn resolve(&mut self, res: &str) {
        self.resolves_vec.push(res.to_string());
    }

    pub fn resolves(&mut self, res: Vec<&str>) {
        for i in res {
            self.resolves_vec.push(i.to_string());
        }
    }

    pub fn limit(&mut self, limit: u64) {
        self.limit = limit;
    }

    pub fn start(&mut self, start: u64) {
        self.start = start;
    }

    pub fn lang(&mut self, lang: &str) {
        self.lang = lang.to_string();
    }

    pub fn retry(&mut self, should_retry: bool) {
        self.should_retry = should_retry;
    }

    pub fn search(&mut self, field: String, value: String) {
        self.search_vec.push((field, value));
    }

    // &[("foo", "a"), ("foo", "b")]
    fn parse_commands(&self) -> Vec<(String, String)> {
        let mut cmd: Vec<(String, String)> = Vec::with_capacity(5);

        if self.resolves_vec.len() > 0 {
            cmd.push(("c:resolve".to_string(), self.resolves_vec.join(",")));
        }

        if self.limit > 0 {
            cmd.push(("c:limit".to_string(), self.limit.to_string()));
        }

        if self.start > 0 {
            cmd.push(("c:start".to_string(), self.start.to_string()));
        }

        if self.lang != "" {
            cmd.push(("c:lang".to_string(), self.lang.clone()));
        }

        cmd.push(("c:retry".to_string(), self.should_retry.to_string()));

        return cmd;
    }

    pub fn build_query(&self, method: &str) -> reqwest::RequestBuilder {
        let url: String = crate::rest::CENSUS_URL.to_string()
            + "s:"
            + &self.serviceid
            + "/"
            + method
            + "/"
            + &self.endpoint
            + "/"
            + &self.collection
            + "/";

        let mut req = self.reqwest_client.get(url);

        req = req.query(&self.parse_commands());

        req = req.query(&self.search_vec);

        return req;
    }

    pub async fn get(&self) -> Result<Value, CensusError> {
        let req = self.build_query("get");

        println!("query");

        let res = req.send().await;

        if res.is_err() {
            return Err(CensusError {
                err_msg: res.expect_err("Unreachable").to_string(),
                parent_err: None,
            });
        }

        let response = res.unwrap();

        let response_res = response.text().await;

        if response_res.is_err() {
            return Err(CensusError {
                err_msg: response_res.expect_err("Unreachable").to_string(),
                parent_err: None,
            });
        }

        let res_wrapped: Result<Value, serde_json::Error> =
            serde_json::from_str(&response_res.unwrap());

        if res_wrapped.is_err() {
            return Err(CensusError {
                err_msg: res_wrapped.expect_err("Unreachable").to_string(),
                parent_err: None,
            });
        }

        return Ok(res_wrapped.unwrap());
    }
}
