use reqwest::{
    Response,
    header::{ACCEPT, HeaderMap, USER_AGENT},
};
use serde::Serialize;

pub struct Client {
    client: reqwest::Client,
    headers: Option<HeaderMap>,
}

pub trait HttpEndpoint {
    fn as_full_url(&self) -> String;
}

impl Client {
    pub fn new(headers: Option<HeaderMap>) -> Self {
        Self {
            headers,
            client: reqwest::Client::new(),
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(
            USER_AGENT,
            "mason-registry-api (+https://github.com/mason-org/mason-registry-api)"
                .parse()
                .unwrap(),
        );
        if let Some(custom_headers) = self.headers.as_ref() {
            headers.extend(custom_headers.to_owned());
        }
        headers
    }

    pub async fn get_with_query<Endpoint: HttpEndpoint, Query: Serialize + ?Sized>(
        &self,
        endpoint: Endpoint,
        query: &Query,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .get(endpoint.as_full_url())
            .query(query)
            .headers(self.headers())
            .send()
            .await?
            .error_for_status()
    }

    pub async fn get<Endpoint: HttpEndpoint>(
        &self,
        endpoint: Endpoint,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .get(endpoint.as_full_url())
            .headers(self.headers())
            .send()
            .await?
            .error_for_status()
    }

    pub async fn post<Json: Serialize, Endpoint: HttpEndpoint>(
        &self,
        endpoint: Endpoint,
        json: &Json,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .post(endpoint.as_full_url())
            .headers(self.headers())
            .json(json)
            .send()
            .await?
            .error_for_status()
    }
}
