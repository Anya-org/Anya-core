# Decentralized SQLite Replacement Strategy for Anya Core

## 🎯 Executive Summary

**Objective**: Replace all SQLite dependencies with decentralized alternatives that align with our Web5/Bitcoin infrastructure and maintain data sovereignty.

**Current SQLite Usage**: 18 TODOs requiring database operations
**Recommended Approach**: Hybrid decentralized storage using IPFS + DWN + Bitcoin anchoring
**Benefits**: Eliminates single points of failure, ensures data sovereignty, provides cryptographic integrity

## 📊 Current SQLite Usage Analysis

### **SQLite TODOs Breakdown** (18 total)

```bash
# EVIDENCE: Current SQLite usage patterns
./mod.rs:                // TODO: Implement actual SQLite asset existence check
./mod.rs:                // TODO: Implement actual SQLite storage  
./mod.rs:                // TODO: Implement actual SQLite queries
./mod.rs:                // TODO: Implement actual SQLite balance queries
./mod.rs:                // TODO: Implement actual SQLite invoice storage
./mod.rs:                // TODO: Implement actual SQLite transfer storage and balance updates
./mod.rs:                // TODO: Implement actual SQLite transfer status query
./mod.rs:                // TODO: Implement actual SQLite transfer validation
./mod.rs:                // TODO: Implement actual SQLite asset history query
```

### **Usage Categories**

1. **Asset Management** (5 operations): Creation, existence checks, metadata queries
2. **Transaction Processing** (4 operations): Transfers, status tracking, validation
3. **Financial Operations** (3 operations): Balance queries, invoice storage
4. **Historical Data** (3 operations): Asset history, transaction logs
5. **System Queries** (3 operations): General data retrieval and validation

## 🌐 Recommended Decentralized Storage Architecture

### **1. Primary Storage: IPFS + Content Addressing**

**Use Case**: Immutable data storage (assets, transactions, history)
**Benefits**: Content-addressed, globally distributed, censorship-resistant

```rust
// IPFS Implementation for Asset Storage
pub struct IPFSAssetStorage {
    client: ipfs_api::IpfsClient,
    pinning_service: PinningService,
    encryption: ChaCha20Poly1305,
}

impl IPFSAssetStorage {
    // Replace: TODO: Implement actual SQLite asset existence check
    pub async fn asset_exists(&self, asset_id: &str) -> AnyaResult<bool> {
        let cid = self.asset_id_to_cid(asset_id)?;
        Ok(self.client.pin_ls(Some(&cid)).await.is_ok())
    }
    
    // Replace: TODO: Implement actual SQLite storage
    pub async fn store_asset(&self, asset: &RGBAsset) -> AnyaResult<String> {
        let encrypted_data = self.encryption.encrypt(&asset.serialize()?)?;
        let add_result = self.client.add(encrypted_data).await?;
        self.pinning_service.pin(&add_result.hash).await?;
        Ok(add_result.hash)
    }
    
    // Replace: TODO: Implement actual SQLite queries  
    pub async fn query_assets(&self, owner_did: &str) -> AnyaResult<Vec<RGBAsset>> {
        let owner_index = self.get_owner_index(owner_did).await?;
        let mut assets = Vec::new();
        
        for asset_cid in owner_index.asset_cids {
            let encrypted_data = self.client.cat(&asset_cid).await?;
            let asset_data = self.encryption.decrypt(&encrypted_data)?;
            let asset: RGBAsset = serde_json::from_slice(&asset_data)?;
            assets.push(asset);
        }
        
        Ok(assets)
    }
}
```

### **2. Indexing & Queries: DWN (Decentralized Web Nodes)**

**Use Case**: Queryable indexes, user-specific data, mutable references
**Benefits**: DID-based access control, schema validation, real-time queries

```rust
// DWN Implementation for Queryable Data
pub struct DWNIndexStorage {
    dwn_manager: DWNManager,
    user_did: String,
}

impl DWNIndexStorage {
    // Replace: TODO: Implement actual SQLite balance queries
    pub async fn get_asset_balance(&self, asset_id: &str) -> AnyaResult<u64> {
        let balance_record = self.dwn_manager.query_records(
            &self.user_did,
            "anya/rgb/balance"
        )?;
        
        let balance_data: BalanceRecord = balance_record
            .into_iter()
            .find(|r| r.metadata.get("asset_id") == Some(&asset_id.to_string()))
            .map(|r| serde_json::from_value(r.data))
            .transpose()?
            .unwrap_or_default();
            
        Ok(balance_data.amount)
    }
    
    // Replace: TODO: Implement actual SQLite invoice storage
    pub async fn store_invoice(&self, invoice: &RGBInvoice) -> AnyaResult<String> {
        let invoice_record = DWNRecord {
            id: generate_id(),
            owner: self.user_did.clone(),
            schema: "anya/rgb/invoice".to_string(),
            data: serde_json::to_value(invoice)?,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("asset_id".to_string(), invoice.asset_id.clone());
                meta.insert("amount".to_string(), invoice.amount.to_string());
                meta.insert("created_at".to_string(), invoice.created_at.to_string());
                meta
            },
            attestations: Vec::new(),
        };
        
        self.dwn_manager.store_record(invoice_record)
    }
    
    // Replace: TODO: Implement actual SQLite transfer status query
    pub async fn get_transfer_status(&self, transfer_id: &str) -> AnyaResult<TransferStatus> {
        let transfer_records = self.dwn_manager.query_records(
            &self.user_did,
            "anya/rgb/transfer"
        )?;
        
        let transfer_record = transfer_records
            .into_iter()
            .find(|r| r.id == transfer_id)
            .ok_or_else(|| AnyaError::NotFound(format!("Transfer {}", transfer_id)))?;
            
        let status_data: TransferRecord = serde_json::from_value(transfer_record.data)?;
        Ok(status_data.status)
    }
}
```

### **3. Data Integrity: Bitcoin Anchoring**

**Use Case**: Immutable timestamps, proof of existence, critical state commits
**Benefits**: Cryptographic proof, censorship resistance, long-term archival

```rust
// Bitcoin Anchoring for Critical Operations
pub struct BitcoinAnchoredStorage {
    bitcoin_client: BitcoinClient,
    network: Network,
}

impl BitcoinAnchoredStorage {
    // Replace: TODO: Implement actual SQLite transfer validation
    pub async fn validate_transfer_with_anchoring(
        &self, 
        transfer: &AssetTransfer
    ) -> AnyaResult<bool> {
        // 1. Validate transfer logic
        let is_valid = self.validate_transfer_logic(transfer)?;
        if !is_valid {
            return Ok(false);
        }
        
        // 2. Create commitment hash
        let transfer_hash = self.compute_transfer_hash(transfer)?;
        
        // 3. Check if already anchored
        if let Some(anchor_tx) = self.find_anchor_transaction(&transfer_hash).await? {
            return Ok(self.verify_anchor_transaction(&anchor_tx)?);
        }
        
        // 4. Create new anchor transaction
        let anchor_txid = self.create_anchor_transaction(&transfer_hash).await?;
        
        // 5. Store anchor reference in DWN
        self.store_anchor_reference(transfer.id.clone(), anchor_txid).await?;
        
        Ok(true)
    }
    
    // Replace: TODO: Implement actual SQLite asset history query
    pub async fn get_asset_history_with_proofs(
        &self, 
        asset_id: &str
    ) -> AnyaResult<Vec<AssetHistoryEntry>> {
        // Query DWN for history entries
        let history_records = self.dwn_manager.query_records(
            "*", // Query all owners for this asset
            "anya/rgb/history"
        )?;
        
        let mut history_entries = Vec::new();
        
        for record in history_records {
            if let Some(record_asset_id) = record.metadata.get("asset_id") {
                if record_asset_id == asset_id {
                    let mut entry: AssetHistoryEntry = serde_json::from_value(record.data)?;
                    
                    // Add Bitcoin proof if available
                    if let Some(anchor_txid) = record.metadata.get("bitcoin_anchor") {
                        entry.bitcoin_proof = Some(
                            self.get_transaction_proof(anchor_txid).await?
                        );
                    }
                    
                    history_entries.push(entry);
                }
            }
        }
        
        // Sort by timestamp
        history_entries.sort_by_key(|e| e.timestamp);
        Ok(history_entries)
    }
}
```

## 🔧 Implementation Strategy

### **Phase 1: Core Storage Layer (Week 1-2)**

1. **IPFS Integration**

   ```rust
   // Add to Cargo.toml
   ipfs-api = "0.17"
   ipfs-embed = "0.23"
   
   // Implement storage trait
   pub trait DecentralizedStorage {
       async fn store(&self, data: &[u8]) -> AnyaResult<ContentId>;
       async fn retrieve(&self, id: &ContentId) -> AnyaResult<Vec<u8>>;
       async fn exists(&self, id: &ContentId) -> AnyaResult<bool>;
   }
   ```

2. **DWN Query Engine**

   ```rust
   // Extend existing DWN implementation
   impl DWNManager {
       pub fn create_index(&self, schema: &str, fields: &[&str]) -> AnyaResult<()>;
       pub fn query_with_filter(&self, filter: DWNQueryFilter) -> AnyaResult<Vec<DWNRecord>>;
       pub fn aggregate(&self, pipeline: &[AggregationStage]) -> AnyaResult<serde_json::Value>;
   }
   ```

### **Phase 2: Data Migration (Week 2-3)**

1. **Asset Storage Migration**

   ```bash
   # Migration script for existing data
   ./scripts/migrate_to_decentralized_storage.sh
   
   # Verification script
   ./scripts/verify_decentralized_migration.sh
   ```

2. **Schema Definition**

   ```json
   {
     "schemas": {
       "anya/rgb/asset": "1.0.0",
       "anya/rgb/transfer": "1.0.0", 
       "anya/rgb/invoice": "1.0.0",
       "anya/rgb/balance": "1.0.0",
       "anya/rgb/history": "1.0.0"
     }
   }
   ```

### **Phase 3: Bitcoin Integration (Week 3-4)**

1. **Anchor Service**

   ```rust
   pub struct BitcoinAnchorService {
       pub fn anchor_data_hash(&self, hash: &[u8]) -> AnyaResult<Transaction>;
       pub fn verify_anchor(&self, tx: &Transaction, hash: &[u8]) -> AnyaResult<bool>;
       pub fn get_anchor_proof(&self, txid: &str) -> AnyaResult<BitcoinProof>;
   }
   ```

## 📊 Comparison: SQLite vs Decentralized

| Feature | SQLite | Decentralized (IPFS+DWN+Bitcoin) |
|---------|--------|-----------------------------------|
| **Data Sovereignty** | ❌ Local file dependency | ✅ User-controlled via DID |
| **Censorship Resistance** | ❌ Single point of failure | ✅ Distributed, no central authority |
| **Scalability** | ❌ Limited by disk/memory | ✅ Network scales globally |
| **Integrity Proofs** | ❌ No cryptographic guarantees | ✅ Bitcoin-anchored proofs |
| **Access Control** | ❌ File system permissions | ✅ DID-based cryptographic control |
| **Synchronization** | ❌ Manual replication | ✅ Built-in P2P sync |
| **Schema Evolution** | ❌ Manual migration scripts | ✅ Versioned schemas in DWN |
| **Backup/Recovery** | ❌ Manual backup required | ✅ Automatic IPFS replication |
| **Offline Capability** | ✅ Local access | ✅ Local IPFS node + DWN cache |
| **Query Performance** | ✅ Optimized indexes | 🟡 Network latency (mitigated by caching) |

## 🎯 Performance Optimizations

### **1. Caching Strategy**

```rust
pub struct DecentralizedStorageCache {
    pub hot_cache: LruCache<ContentId, Vec<u8>>,     // 100MB, 1 hour TTL
    pub query_cache: LruCache<String, QueryResult>,  // 50MB, 5 min TTL  
    pub metadata_cache: LruCache<String, Metadata>,  // 25MB, 15 min TTL
}
```

### **2. Batch Operations**

```rust
impl DWNManager {
    pub async fn batch_store(&self, records: Vec<DWNRecord>) -> AnyaResult<Vec<String>> {
        const BATCH_SIZE: usize = 50; // From existing implementation
        
        let mut results = Vec::new();
        for chunk in records.chunks(BATCH_SIZE) {
            let batch_results = self.store_batch(chunk).await?;
            results.extend(batch_results);
        }
        Ok(results)
    }
}
```

### **3. Compression & Encryption**

```rust
pub struct StorageLayer {
    compression: CompressionAlgorithm,    // LZ4 for speed
    encryption: ChaCha20Poly1305,         // From existing PRD requirements
}
```

## 🔒 Security Benefits

### **1. Data Integrity**

- **Content Addressing**: IPFS ensures data hasn't been tampered with
- **Bitcoin Anchoring**: Provides immutable timestamps and existence proofs
- **Cryptographic Signatures**: DWN records are signed by DID controllers

### **2. Access Control**

- **DID-based Authentication**: Only DID controllers can modify their data
- **Capability-based Authorization**: Fine-grained permissions per data type
- **End-to-End Encryption**: Data encrypted before storage

### **3. Privacy**

- **Encrypted Storage**: All sensitive data encrypted with user keys
- **Metadata Protection**: Query patterns hidden via private indexing
- **Selective Disclosure**: Users control what data is shared

## 📋 Implementation Roadmap

### **Week 1: Foundation**

- [ ] Implement IPFS storage trait
- [ ] Extend DWN with indexing capabilities
- [ ] Create migration scripts for existing data

### **Week 2: Core Functions**

- [ ] Replace asset storage SQLite TODOs (5 functions)
- [ ] Replace transaction processing SQLite TODOs (4 functions)
- [ ] Implement batch operations

### **Week 3: Advanced Features**

- [ ] Replace financial operations SQLite TODOs (3 functions)
- [ ] Replace historical data SQLite TODOs (3 functions)
- [ ] Add Bitcoin anchoring for critical operations

### **Week 4: Integration & Testing**

- [ ] Replace system query SQLite TODOs (3 functions)
- [ ] Performance optimization and caching
- [ ] Comprehensive testing and validation

## ✅ Success Metrics

### **Technical Metrics**

- [ ] **SQLite TODOs**: 18 → 0 (100% elimination)
- [ ] **Query Performance**: <100ms for cached queries
- [ ] **Storage Efficiency**: 60-80% compression ratio
- [ ] **Availability**: 99.9% uptime via IPFS redundancy

### **Quality Gate Compliance**

- [ ] **Zero unimplemented!() macros** in storage modules
- [ ] **Compilation success** with no warnings
- [ ] **Security compliance** - no hardcoded secrets
- [ ] **Test coverage** >90% for all storage operations

## 🚀 Business Value

### **Immediate Benefits**

1. **Eliminates SQLite dependency** - Removes centralized database requirement
2. **Improves data sovereignty** - Users control their own data via DIDs
3. **Enhances security** - Cryptographic integrity and access control

### **Long-term Benefits**

1. **Enables global scale** - IPFS network provides unlimited scalability
2. **Future-proofs architecture** - Decentralized by design, no vendor lock-in
3. **Regulatory compliance** - Data sovereignty meets international privacy laws

## 🎯 Next Actions

1. **Create IPFS storage implementation** for asset management
2. **Extend DWN query capabilities** for complex data operations  
3. **Implement Bitcoin anchoring service** for data integrity
4. **Create migration scripts** to move from SQLite TODOs to decentralized storage
5. **Update quality gate script** to validate decentralized storage compliance

**Result**: A fully decentralized, censorship-resistant, and user-sovereign storage system that eliminates traditional database dependencies while maintaining performance and adding cryptographic guarantees.
