# Production Storage Implementation Plan - DWN + IPFS + Bitcoin Anchoring

## Document Information

- **Date**: July 5, 2025
- **Purpose**: Production implementation plan based on latest DWN SDK v0.5.2 and IPFS features
- **Status**: Evidence-based implementation roadmap
- **Evidence Source**: Web research of DWN SDK and rust-ipfs-api features

## Executive Summary

This document outlines the comprehensive implementation plan for replacing all SQLite dependencies with a production-ready decentralized storage solution using:

1. **DWN (Decentralized Web Nodes) v0.5.2** - Structured data with DID-based access control
2. **IPFS with Kademlia DHT** - Content-addressed immutable storage
3. **Bitcoin Anchoring** - Tamper-proof timestamping and integrity verification

## Implementation Architecture

### 1. Three-Layer Storage Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    APPLICATION LAYER                           │
│ RGB Assets │ Transfers │ Balances │ Invoices │ History │ Metadata │
├─────────────────────────────────────────────────────────────────┤
│              UNIFIED STORAGE INTERFACE (TRAIT)                 │
├─────────────────────────────────────────────────────────────────┤
│ DWN LAYER (v0.5.2)  │  IPFS LAYER (DHT)   │  BITCOIN LAYER     │
│ ┌─────────────────┐ │ ┌─────────────────┐  │ ┌─────────────────┐ │
│ │ RecordsWrite    │ │ │ Content Store   │  │ │ Merkle Proofs   │ │
│ │ RecordsQuery    │ │ │ Kademlia DHT    │  │ │ Timestamping    │ │
│ │ ProtocolConfig  │ │ │ Pin Services    │  │ │ Anchor Batch    │ │
│ │ DID Auth        │ │ │ Bitswap         │  │ │ Verification    │ │
│ └─────────────────┘ │ └─────────────────┘  │ └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 2. Production Storage Implementation

```rust
// Production Decentralized Storage
pub struct ProductionDecentralizedStorage {
    // DWN Layer - v0.5.2 features
    dwn: Arc<DWNManager>,
    message_store: MessageStoreLevel,
    data_store: DataStoreLevel,
    event_log: EventLogLevel,
    tenant_gate: CustomTenantGate,
    
    // IPFS Layer - Full stack
    ipfs: Arc<IPFSStorage>,
    dht: KademliaDHT,
    content_routing: ContentRouter,
    pin_manager: PinManager,
    
    // Bitcoin Layer
    bitcoin_anchor: Arc<BitcoinAnchorService>,
    merkle_tree: MerkleTree,
    
    // Performance Layer
    cache: Arc<MultiLayerCache>,
    batch_processor: Arc<BatchProcessor>,
}
```

## Phase 1: DWN Production Backend (Weeks 1-2)

### 1.1 Replace HashMap with Persistent Storage

**Current Issue**: `Arc<Mutex<HashMap<String, DWNRecord>>>` in `/src/web5/dwn.rs`

**Implementation**:

```rust
// Production DWN Manager
pub struct ProductionDWNManager {
    message_store: MessageStoreLevel,
    data_store: DataStoreLevel,
    event_log: EventLogLevel,
    tenant_gate: CustomTenantGate,
    protocol_registry: ProtocolRegistry,
    sync_manager: SyncManager,
    encryption: DWNEncryption,
}

impl ProductionDWNManager {
    pub async fn new(config: DWNConfig) -> Result<Self> {
        Ok(Self {
            message_store: MessageStoreLevel::new(&config.message_store_path).await?,
            data_store: DataStoreLevel::new(&config.data_store_path).await?,
            event_log: EventLogLevel::new(&config.event_log_path).await?,
            tenant_gate: CustomTenantGate::new(config.allowed_dids),
            protocol_registry: ProtocolRegistry::new(),
            sync_manager: SyncManager::new(config.sync_endpoints),
            encryption: DWNEncryption::new(config.encryption_key)?,
        })
    }
}
```

### 1.2 Implement DWN Message Types (from v0.5.2 SDK)

```rust
// Production message handling
#[async_trait]
impl DWNMessageHandler for ProductionDWNManager {
    async fn process_records_write(&self, message: RecordsWrite) -> DWNResult<ProcessResult> {
        // 1. Validate message signature (JWS)
        self.validate_jws_signature(&message).await?;
        
        // 2. Check tenant permissions
        self.tenant_gate.authorize_write(&message.descriptor.recipient).await?;
        
        // 3. Store in encrypted data store
        let encrypted_data = self.encryption.encrypt(&message.encoded_data)?;
        self.data_store.put(&message.record_id, encrypted_data).await?;
        
        // 4. Index in message store
        self.message_store.put(&message.record_id, &message.descriptor).await?;
        
        // 5. Log event
        self.event_log.append(WriteEvent::new(&message.record_id)).await?;
        
        Ok(ProcessResult::Accepted)
    }
    
    async fn process_records_query(&self, query: RecordsQuery) -> DWNResult<QueryResult> {
        // Implement efficient querying with indexing
        let filter = &query.descriptor.filter;
        let records = self.message_store.query(filter).await?;
        Ok(QueryResult { records })
    }
}
```

### 1.3 DID-Based Access Control

```rust
// Production access control
pub struct DWNAccessControl {
    did_resolver: DIDResolver,
    capability_store: CapabilityStore,
    permission_engine: PermissionEngine,
}

impl DWNAccessControl {
    pub async fn authorize_operation(
        &self,
        did: &str,
        operation: Operation,
        resource: &str,
    ) -> Result<bool> {
        // 1. Resolve DID to DID Document
        let did_doc = self.did_resolver.resolve(did).await?;
        
        // 2. Verify signature using DID verification methods
        let verified = self.verify_did_signature(&did_doc, &operation.signature)?;
        if !verified { return Ok(false); }
        
        // 3. Check capabilities and permissions
        let capabilities = self.capability_store.get_capabilities(did, resource).await?;
        self.permission_engine.check_permission(&operation, &capabilities)
    }
}
```

## Phase 2: IPFS Integration (Weeks 3-4)

### 2.1 Content-Addressed Storage with CIDv1

```rust
// Production IPFS integration
pub struct IPFSStorage {
    client: IpfsClient,
    dht: KademliaDHT,
    pin_manager: PinManager,
    content_router: ContentRouter,
    encryption: ContentEncryption,
}

impl IPFSStorage {
    pub async fn store_content(&self, data: &[u8]) -> Result<ContentId> {
        // 1. Encrypt data
        let encrypted_data = self.encryption.encrypt(data)?;
        
        // 2. Create CIDv1 with SHA-256
        let cid = self.create_cidv1(&encrypted_data).await?;
        
        // 3. Store in IPFS
        self.client.add(encrypted_data).await?;
        
        // 4. Announce to DHT
        self.dht.provide(&cid).await?;
        
        // 5. Pin content
        self.pin_manager.pin(&cid, PinType::Recursive).await?;
        
        Ok(ContentId::from(cid))
    }
    
    pub async fn retrieve_content(&self, cid: &ContentId) -> Result<Vec<u8>> {
        // 1. Find providers via DHT
        let providers = self.dht.find_providers(cid).await?;
        
        // 2. Retrieve content via Bitswap
        let encrypted_data = self.client.cat(cid).await?;
        
        // 3. Decrypt
        let data = self.encryption.decrypt(&encrypted_data)?;
        
        Ok(data)
    }
}
```

### 2.2 Kademlia DHT Implementation

```rust
// Production DHT integration
pub struct KademliaDHT {
    routing_table: RoutingTable,
    peer_store: PeerStore,
    provider_store: ProviderStore,
    bootstrap_peers: Vec<PeerId>,
}

impl KademliaDHT {
    pub async fn bootstrap(&mut self) -> Result<()> {
        // Connect to bootstrap peers
        for peer_id in &self.bootstrap_peers {
            self.connect_peer(peer_id).await?;
        }
        
        // Perform initial DHT queries to populate routing table
        self.refresh_routing_table().await?;
        
        Ok(())
    }
    
    pub async fn find_providers(&self, cid: &CID) -> Result<Vec<PeerId>> {
        // Query k-closest peers for providers
        let closest_peers = self.routing_table.closest_peers(&cid.hash(), 20);
        
        let mut providers = Vec::new();
        for peer in closest_peers {
            if let Ok(peer_providers) = self.query_providers(&peer, cid).await {
                providers.extend(peer_providers);
            }
        }
        
        Ok(providers)
    }
    
    pub async fn provide(&self, cid: &CID) -> Result<()> {
        // Announce that we provide this content
        let record = ProviderRecord::new(cid.clone(), self.local_peer_id());
        self.provider_store.add_provider(record).await?;
        
        // Announce to k-closest peers
        let closest_peers = self.routing_table.closest_peers(&cid.hash(), 20);
        for peer in closest_peers {
            self.announce_provider(&peer, cid).await?;
        }
        
        Ok(())
    }
}
```

## Phase 3: Bitcoin Anchoring (Weeks 5-6)

### 3.1 Merkle Tree Anchoring

```rust
// Production Bitcoin anchoring
pub struct BitcoinAnchorService {
    bitcoin_client: BitcoinClient,
    merkle_tree: MerkleTree,
    anchor_queue: AnchorQueue,
    batch_size: usize,
    anchor_frequency: Duration,
}

impl BitcoinAnchorService {
    pub async fn anchor_content_batch(&self, content_hashes: Vec<Hash>) -> Result<AnchorResult> {
        // 1. Create Merkle tree from content hashes
        let merkle_tree = MerkleTree::from_hashes(content_hashes);
        let merkle_root = merkle_tree.root();
        
        // 2. Create OP_RETURN transaction
        let anchor_tx = self.create_anchor_transaction(merkle_root).await?;
        
        // 3. Broadcast to Bitcoin network
        let txid = self.bitcoin_client.send_raw_transaction(&anchor_tx).await?;
        
        // 4. Store anchoring metadata
        let anchor_record = AnchorRecord {
            txid,
            merkle_root,
            content_hashes: merkle_tree.leaves().clone(),
            timestamp: SystemTime::now(),
        };
        
        self.store_anchor_record(anchor_record).await?;
        
        Ok(AnchorResult { txid, merkle_root })
    }
    
    pub async fn verify_content_anchoring(&self, content_hash: &Hash) -> Result<AnchorProof> {
        // 1. Find anchor record containing this content
        let anchor_record = self.find_anchor_record(content_hash).await?;
        
        // 2. Generate Merkle proof
        let merkle_proof = anchor_record.generate_merkle_proof(content_hash)?;
        
        // 3. Verify transaction is confirmed
        let tx_status = self.bitcoin_client.get_transaction_status(&anchor_record.txid).await?;
        
        Ok(AnchorProof {
            txid: anchor_record.txid,
            merkle_proof,
            confirmations: tx_status.confirmations,
            block_height: tx_status.block_height,
        })
    }
}
```

### 3.2 Batch Processing for Efficiency

```rust
// Efficient batch anchoring
pub struct AnchorBatchProcessor {
    queue: Arc<Mutex<VecDeque<PendingAnchor>>>,
    batch_size: usize,
    batch_timeout: Duration,
}

impl AnchorBatchProcessor {
    pub async fn queue_for_anchoring(&self, content_hash: Hash, priority: AnchorPriority) {
        let mut queue = self.queue.lock().await;
        queue.push_back(PendingAnchor {
            content_hash,
            priority,
            queued_at: SystemTime::now(),
        });
        
        // Trigger batch processing if queue is full
        if queue.len() >= self.batch_size {
            self.process_batch().await;
        }
    }
    
    async fn process_batch(&self) {
        let mut queue = self.queue.lock().await;
        let batch: Vec<_> = queue.drain(..self.batch_size.min(queue.len())).collect();
        drop(queue);
        
        if !batch.is_empty() {
            let content_hashes: Vec<Hash> = batch.into_iter().map(|p| p.content_hash).collect();
            if let Err(e) = self.anchor_service.anchor_content_batch(content_hashes).await {
                log::error!("Batch anchoring failed: {}", e);
            }
        }
    }
}
```

## Phase 4: Performance Optimization (Weeks 7-8)

### 4.1 Multi-Layer Caching

```rust
// Production caching strategy
pub struct MultiLayerCache {
    // L1: Hot cache for frequently accessed data
    hot_cache: Arc<Mutex<LruCache<ContentId, CachedContent>>>,
    
    // L2: Query result cache
    query_cache: Arc<Mutex<LruCache<QueryHash, QueryResult>>>,
    
    // L3: Metadata cache
    metadata_cache: Arc<Mutex<LruCache<ContentId, ContentMetadata>>>,
    
    cache_stats: Arc<CacheStatistics>,
}

impl MultiLayerCache {
    pub async fn get_content(&self, cid: &ContentId) -> Option<CachedContent> {
        // Try L1 cache first
        if let Some(content) = self.hot_cache.lock().await.get(cid) {
            self.cache_stats.record_hit(CacheLayer::Hot);
            return Some(content.clone());
        }
        
        // Try loading from storage if not in cache
        None
    }
    
    pub async fn cache_content(&self, cid: ContentId, content: CachedContent) {
        let mut cache = self.hot_cache.lock().await;
        cache.put(cid, content);
        self.cache_stats.record_insert(CacheLayer::Hot);
    }
    
    pub async fn cache_statistics(&self) -> CacheStatistics {
        self.cache_stats.snapshot()
    }
}
```

### 4.2 Batch Operations

```rust
// High-performance batch operations
#[async_trait]
impl BatchOperations for ProductionDecentralizedStorage {
    async fn batch_store(&self, items: Vec<StoreItem>) -> Result<BatchResult> {
        const BATCH_SIZE: usize = 100;
        let mut results = Vec::new();
        
        for chunk in items.chunks(BATCH_SIZE) {
            let batch_tasks: Vec<_> = chunk.iter().map(|item| {
                let storage = self.clone();
                async move {
                    storage.store_item(item.clone()).await
                }
            }).collect();
            
            let batch_results = futures::future::join_all(batch_tasks).await;
            results.extend(batch_results);
            
            // Rate limiting between batches
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        Ok(BatchResult { results })
    }
    
    async fn batch_retrieve(&self, cids: Vec<ContentId>) -> Result<Vec<Option<Content>>> {
        // Parallel retrieval with connection pooling
        let semaphore = Arc::new(Semaphore::new(20)); // Max 20 concurrent retrievals
        
        let tasks: Vec<_> = cids.into_iter().map(|cid| {
            let storage = self.clone();
            let semaphore = semaphore.clone();
            
            async move {
                let _permit = semaphore.acquire().await.unwrap();
                storage.retrieve_content(&cid).await
            }
        }).collect();
        
        let results = futures::future::join_all(tasks).await;
        Ok(results.into_iter().map(|r| r.ok()).collect())
    }
}
```

## Phase 5: Production Deployment (Weeks 9-10)

### 5.1 Configuration Management

```toml
# production.toml
[storage.decentralized]
# DWN Configuration
dwn_message_store_path = "/var/lib/anya/dwn/messages"
dwn_data_store_path = "/var/lib/anya/dwn/data"
dwn_event_log_path = "/var/lib/anya/dwn/events"
dwn_encryption_algorithm = "ChaCha20-Poly1305"

# IPFS Configuration
ipfs_endpoints = [
    "http://127.0.0.1:5001",
    "https://ipfs.io",
    "https://gateway.pinata.cloud"
]
ipfs_dht_enabled = true
ipfs_bitswap_enabled = true
ipfs_content_routing = "DHT"

# Pinning Services
[[storage.decentralized.pin_services]]
name = "pinata"
api_key = "${PINATA_API_KEY}"
secret_key = "${PINATA_SECRET_KEY}"

[[storage.decentralized.pin_services]]
name = "web3storage"
api_token = "${WEB3_STORAGE_TOKEN}"

# Bitcoin Configuration
bitcoin_network = "mainnet"
bitcoin_rpc_url = "http://127.0.0.1:8332"
bitcoin_anchor_frequency = "6h"
bitcoin_batch_size = 1000
bitcoin_fee_rate = "10 sat/vB"

# Performance Configuration
cache_size_mb = 512
batch_size = 100
max_concurrent_operations = 50
query_timeout_seconds = 30
```

### 5.2 Monitoring and Observability

```rust
// Production monitoring
pub struct StorageMonitoring {
    metrics: Arc<StorageMetrics>,
    health_checker: HealthChecker,
    alert_manager: AlertManager,
}

#[derive(Debug, Clone)]
pub struct StorageMetrics {
    // Performance metrics
    operation_latency: HistogramVec,
    cache_hit_rate: GaugeVec,
    throughput: CounterVec,
    
    // Health metrics
    dwn_status: Gauge,
    ipfs_connectivity: Gauge,
    bitcoin_anchor_status: Gauge,
    
    // Error metrics
    error_rate: CounterVec,
    failed_operations: CounterVec,
}

impl StorageMonitoring {
    pub async fn start_monitoring(&self) -> Result<()> {
        // Start periodic health checks
        let health_checker = self.health_checker.clone();
        tokio::spawn(async move {
            loop {
                health_checker.check_all_services().await;
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        });
        
        // Start metrics collection
        let metrics = self.metrics.clone();
        tokio::spawn(async move {
            loop {
                metrics.collect_system_metrics().await;
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        });
        
        Ok(())
    }
}
```

## Migration Strategy

### Week 1-2: Infrastructure Setup

1. Set up production DWN storage backends
2. Implement DID-based authentication
3. Create encryption layer with HSM integration

### Week 3-4: IPFS Integration  

1. Deploy IPFS nodes with DHT enabled
2. Implement content addressing and retrieval
3. Set up pinning services and redundancy

### Week 5-6: Bitcoin Anchoring

1. Implement Merkle tree anchoring
2. Set up batch processing pipeline
3. Create verification and proof systems

### Week 7-8: Performance & Testing

1. Implement caching layers
2. Optimize batch operations
3. Conduct load testing and benchmarking

### Week 9-10: Production Deployment

1. Deploy to production environment
2. Migrate existing data
3. Enable monitoring and alerting

## Success Metrics

### Performance Targets

- **Storage Latency**: < 100ms for cached operations
- **Retrieval Latency**: < 500ms for IPFS content
- **Cache Hit Rate**: > 90% for frequently accessed content
- **Batch Throughput**: > 1000 operations/minute
- **Availability**: 99.9% uptime

### Security Targets

- **Encryption**: 100% of data encrypted at rest and in transit
- **Access Control**: DID-based authentication for all operations
- **Integrity**: Bitcoin anchoring for critical data with Merkle proof verification
- **Key Management**: HSM-based key storage and rotation

### Scalability Targets

- **Storage Capacity**: 10TB+ per node
- **Content Items**: 10M+ items
- **Concurrent Users**: 10K+ simultaneous operations
- **Network Peers**: 1000+ IPFS peers

## Risk Mitigation

### Technical Risks

1. **IPFS Network Partitions**: Multiple gateway endpoints and peer redundancy
2. **Bitcoin Network Congestion**: Batch anchoring and fee optimization
3. **DWN Sync Failures**: Eventual consistency and conflict resolution
4. **Key Management**: HSM integration and secure key rotation

### Operational Risks

1. **Data Migration**: Staged migration with rollback capability
2. **Performance Degradation**: Comprehensive monitoring and alerting
3. **Security Breaches**: Defense in depth with multiple security layers
4. **Compliance**: Regular security audits and penetration testing

## Conclusion

This implementation plan provides a comprehensive roadmap for replacing all SQLite dependencies with a production-ready decentralized storage solution. The three-layer architecture (DWN + IPFS + Bitcoin) ensures data sovereignty, performance, security, and integrity while maintaining compatibility with existing Anya Core APIs.

The staged approach allows for incremental deployment and testing, reducing risks while ensuring each component is production-ready before proceeding to the next phase. Success metrics and monitoring ensure the system meets enterprise-grade requirements for performance, security, and reliability.
