use std::collections::HashMap;

use crate::stellar_sdk::api_call::api_call;
use crate::stellar_sdk::endpoints::{horizon::Record, CallBuilder, Server};
use crate::stellar_sdk::types::Effect;
use crate::stellar_sdk::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct EffectCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<String, String>,
    token: &'a Option<String>,
}

impl<'a> EffectCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        Self {
            server_url: &s.server_url,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
            token: &s.options.auth_token,
        }
    }
}

impl<'a> CallBuilder<Effect> for EffectCallBuilder<'a> {
    fn cursor(&mut self, cursor: &str) -> &mut Self {
        self.query_params
            .insert(String::from("cursor"), String::from(cursor));

        self
    }

    fn order(&mut self, dir: Direction) -> &mut Self {
        self.query_params
            .insert(String::from("order"), dir.to_string());

        self
    }

    fn limit(&mut self, limit: u8) -> &mut Self {
        self.query_params
            .insert(String::from("limit"), limit.to_string());

        self
    }

    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }

    fn call(&self) -> Result<Record<Effect>, anyhow::Error> {
        let url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/effects",
        );

        api_call::<Record<Effect>>(
            url,
            crate::stellar_sdk::types::HttpMethod::GET,
            &self.query_params,
            self.token,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_horizon_test() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let mut ecb = EffectCallBuilder::new(&s);

        let effect_records = ecb.limit(200).call().unwrap();

        assert_eq!(effect_records._embedded.records.len(), 200);
    }
}
