// API client module for HTTP requests
use reqwest;
use serde::{Deserialize, Serialize};

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
    token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url,
            token: None,
        }
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T, reqwest::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        let mut request = self.client.get(&url);
        
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        
        request.send().await?.json().await
    }

    pub async fn post<T, R>(&self, endpoint: &str, body: &T) -> Result<R, reqwest::Error>
    where
        T: Serialize + ?Sized,
        R: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        let mut request = self.client.post(&url).json(body);
        
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        
        request.send().await?.json().await
    }
}