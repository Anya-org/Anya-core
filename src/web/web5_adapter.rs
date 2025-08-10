// Web5Adapter: MIT-compliant isolates all web5-rust logic for DID, DWN, and VC operations
// This is a scaffold. Implementations will be filled in as we refactor usages.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum Web5AdapterError {
    Http(String),
    Serialization(String),
}

impl std::fmt::Display for Web5AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Web5AdapterError::Http(m) | Web5AdapterError::Serialization(m) => write!(f, "{m}"),
        }
    }
}

impl std::error::Error for Web5AdapterError {}

#[derive(Clone, Debug)]
pub struct Web5Adapter {
    pub service_url: String,
    pub agent: ureq::Agent,
    // In-memory DWN record store for test/local mode
    records: Arc<Mutex<HashMap<String, crate::web5::dwn::DWNRecord>>>,
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
            agent: ureq::Agent::new(),
            records: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_did(
        &self,
        method: &str,
    ) -> Result<DidDocumentResponse, Box<dyn Error + Send + Sync>> {
        let req = CreateDidRequest {
            method: method.to_string(),
        };
        let url = format!("{}/did/create", self.service_url);
        let resp = self.agent.post(&url).send_json(&req)?;
        let did_doc: DidDocumentResponse = resp.into_json()?;
        Ok(did_doc)
    }

    pub fn resolve_did(
        &self,
        did: &str,
    ) -> Result<DidDocumentResponse, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/did/resolve?did={}", self.service_url, did);
        let resp = self.agent.get(&url).call()?;
        let did_doc: DidDocumentResponse = resp.into_json()?;
        Ok(did_doc)
    }

    pub fn dwn_send_message(
        &self,
        message: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/dwn/send", self.service_url);
        let resp = self.agent.post(&url).send_json(message)?;
        let result: serde_json::Value = resp.into_json()?;
        Ok(result)
    }

    pub fn dwn_get_messages(
        &self,
        filter: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/dwn/messages", self.service_url);
        let req = filter.unwrap_or(&serde_json::Value::Null);
        let resp = self.agent.post(&url).send_json(req)?;
        let result: serde_json::Value = resp.into_json()?;
        Ok(result)
    }

    pub fn issue_credential(
        &self,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/vc/issue", self.service_url);
        let resp = self.agent.post(&url).send_json(payload)?;
        let result: serde_json::Value = resp.into_json()?;
        Ok(result)
    }

    pub fn verify_credential(
        &self,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/vc/verify", self.service_url);
        let resp = self.agent.post(&url).send_json(payload)?;
        let result: serde_json::Value = resp.into_json()?;
        Ok(result)
    }

    pub fn store_credential(
        &self,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/vc/store", self.service_url);
        let resp = self.agent.post(&url).send_json(payload)?;
        let result: serde_json::Value = resp.into_json()?;
        Ok(result)
    }

    pub fn get_credential(
        &self,
        id: &str,
    ) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/vc/get?id={}", self.service_url, id);
        let resp = self.agent.get(&url).call()?;
        let result: serde_json::Value = resp.into_json()?;
        Ok(result)
    }

    pub fn list_credentials(&self) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/vc/list", self.service_url);
        let resp = self.agent.get(&url).call()?;
        let result: serde_json::Value = resp.into_json()?;
        Ok(result)
    }

    // Helper methods for testing and validation
    pub fn health_check(&self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/health", self.service_url);
        let resp = self.agent.get(&url).call()?;
        Ok(resp.status() == 200)
    }

    // DWN record query stub (synchronous placeholder)
    pub fn query_records(
        &self,
        owner: &str,
        schema: &str,
    ) -> Result<Vec<crate::web5::dwn::DWNRecord>, Web5AdapterError> {
        let storage = self
            .records
            .lock()
            .map_err(|e| Web5AdapterError::Http(format!("lock error: {e}")))?;
        let results = storage
            .values()
            .filter(|r| (owner == "*" || r.owner == owner) && r.schema == schema)
            .cloned()
            .collect();
        Ok(results)
    }

    pub fn store_record(
        &self,
        record: &crate::web5::dwn::DWNRecord,
    ) -> Result<String, Web5AdapterError> {
        let mut storage = self
            .records
            .lock()
            .map_err(|e| Web5AdapterError::Http(format!("lock error: {e}")))?;
        storage.insert(record.id.clone(), record.clone());
        Ok(record.id.clone())
    }
}
