// Web5Adapter: Isolates all web5-rust logic for DID, DWN, and VC operations
// This is a scaffold. Implementations will be filled in as we refactor usages.

use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::error::Error;

#[derive(Clone)]
pub struct Web5Adapter {
    pub service_url: String,
    pub client: Client,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDidRequest {
    pub method: String,
}

#[derive(Serialize, Deserialize)]
pub struct DidDocumentResponse {
    pub did: String,
    pub document: serde_json::Value,
}

impl Web5Adapter {
    pub fn new(service_url: &str) -> Self {
        Self {
            service_url: service_url.to_string(),
            client: Client::new(),
        }
    }

    pub async fn create_did(&self, method: &str) -> Result<DidDocumentResponse, Box<dyn Error>> {
        let req = CreateDidRequest {
            method: method.to_string(),
        };
        let url = format!("{}/did/create", self.service_url);
        let resp = self.client.post(&url)
            .json(&req)
            .send()
            .await?;
        let did_doc = resp.json::<DidDocumentResponse>().await?;
        Ok(did_doc)
    }
    pub async fn resolve_did(&self, did: &str) -> Result<DidDocumentResponse, Box<dyn Error>> {
        let url = format!("{}/did/resolve", self.service_url);
        let resp = self.client.get(&url)
            .query(&[("did", did)])
            .send()
            .await?;
        let did_doc = resp.json::<DidDocumentResponse>().await?;
        Ok(did_doc)
    }

    pub async fn dwn_send_message(&self, message: serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
        let url = format!("{}/dwn/send", self.service_url);
        let resp = self.client.post(&url)
            .json(&message)
            .send()
            .await?;
        let result = resp.json::<serde_json::Value>().await?;
        Ok(result)
    }

    pub async fn dwn_get_messages(&self, filter: Option<serde_json::Value>) -> Result<serde_json::Value, Box<dyn Error>> {
        let url = format!("{}/dwn/messages", self.service_url);
        let req = filter.unwrap_or_default();
        let resp = self.client.post(&url)
            .json(&req)
            .send()
            .await?;
        let result = resp.json::<serde_json::Value>().await?;
        Ok(result)
    }

    pub async fn issue_credential(&self, payload: serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
        let url = format!("{}/vc/issue", self.service_url);
        let resp = self.client.post(&url)
            .json(&payload)
            .send()
            .await?;
        let result = resp.json::<serde_json::Value>().await?;
        Ok(result)
    }

    pub async fn verify_credential(&self, payload: serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
        let url = format!("{}/vc/verify", self.service_url);
        let resp = self.client.post(&url)
            .json(&payload)
            .send()
            .await?;
        let result = resp.json::<serde_json::Value>().await?;
        Ok(result)
    }

    pub async fn store_credential(&self, payload: serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
        let url = format!("{}/vc/store", self.service_url);
        let resp = self.client.post(&url)
            .json(&payload)
            .send()
            .await?;
        let result = resp.json::<serde_json::Value>().await?;
        Ok(result)
    }

    pub async fn get_credential(&self, id: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let url = format!("{}/vc/get", self.service_url);
        let resp = self.client.get(&url)
            .query(&[("id", id)])
            .send()
            .await?;
        let result = resp.json::<serde_json::Value>().await?;
        Ok(result)
    }

    pub async fn list_credentials(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        let url = format!("{}/vc/list", self.service_url);
        let resp = self.client.get(&url)
            .send()
            .await?;
        let result = resp.json::<serde_json::Value>().await?;
        Ok(result)
    }
}
