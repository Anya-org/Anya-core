# Anya Core System Architecture

## Repository Structure

```mermaid
graph TB
    subgraph anya-core[Anya Core]
        Core[Core Components]
        AI[AI Engine]
        Security[Security Layer]
        Bitcoin[Bitcoin & Lightning]
    end

    subgraph Submodules[Primary Submodules]
        dash33[dash33 - AI Decision Engine]
        enterprise[Enterprise Integration]
        mobile[Mobile Interface]
        web5[Web5 Implementation]
    end

    subgraph Integration[Integration Points]
        API[API Layer]
        Events[Event System]
        Data[Data Layer]
    end

    %% Core Connections
    Core --> AI
    Core --> Security
    Core --> Bitcoin
    AI --> Security

    %% Submodule Connections
    dash33 --> AI
    enterprise --> Core
    mobile --> API
    web5 --> Security
    Bitcoin --> Security

    %% Integration Layer
    API --> Security
    Events --> Core
    Data --> Security
```

## Component Details

### Core Repository
1. **Core Components**
   - Base system functionality
   - Configuration management
   - Service orchestration
   - Resource management

2. **AI Engine**
   - Model management
   - Decision making
   - Learning systems
   - Performance optimization

3. **Security Layer**
   - Access control
   - Encryption
   - Audit logging
   - Policy enforcement
   
4. **Bitcoin & Lightning**
   - Bitcoin protocol implementation
   - Lightning Network integration
   - Payment channels
   - Wallet management

### Bitcoin & Lightning Architecture
```mermaid
graph TB
    subgraph Bitcoin[Bitcoin Layer]
        Core[Bitcoin Core]
        Wallet[Wallet]
        Network[Network]
        Transactions[Transactions]
    end

    subgraph Lightning[Lightning Layer]
        LNode[Lightning Node]
        Channels[Channel Management]
        Payments[Payment Processing]
        Bridge[Bitcoin-Lightning Bridge]
    end

    subgraph Integration[Integration Layer]
        API[Bitcoin/Lightning API]
        Events[Event Handling]
        Security[Security & Encryption]
    end

    %% Connections
    Core --> Wallet
    Core --> Network
    Core --> Transactions
    
    LNode --> Channels
    LNode --> Payments
    Bridge --> Channels
    
    Wallet --> Bridge
    Network --> Bridge
    Transactions --> Bridge
    
    API --> Core
    API --> LNode
    Events --> Core
    Events --> LNode
    Security --> Core
    Security --> LNode
```

### Submodules

1. **dash33 (AI Decision Engine)**
   ```mermaid
   graph LR
       Core[Core Engine]
       Analytics[Analytics]
       Models[Models]
       API[API]

       Core --> Analytics
       Core --> Models
       Core --> API
   ```

2. **Enterprise Integration**
   ```mermaid
   graph LR
       Core[Enterprise Core]
       Auth[Authentication]
       Admin[Administration]
       Integration[Integration]

       Core --> Auth
       Core --> Admin
       Core --> Integration
   ```

3. **Mobile Interface**
   ```mermaid
   graph LR
       Core[Mobile Core]
       UI[UI Components]
       Services[Services]
       State[State Management]

       Core --> UI
       Core --> Services
       Core --> State
   ```

4. **Web5 Implementation**
   ```mermaid
   graph LR
       Core[Web5 Core]
       DID[DID System]
       Storage[Storage]
       Protocol[Protocol]

       Core --> DID
       Core --> Storage
       Core --> Protocol
   ```

## Integration Architecture

```mermaid
sequenceDiagram
    participant User
    participant Mobile
    participant Core
    participant dash33
    participant Web5
    participant Lightning

    User->>Mobile: Payment Request
    Mobile->>Core: Process
    Core->>dash33: Analyze
    dash33-->>Core: Decision
    Core->>Lightning: Create Invoice
    Lightning-->>Core: Invoice
    Core-->>Mobile: BOLT11 Invoice
    Mobile-->>User: Display QR Code
    User->>Mobile: Confirm
    Mobile->>Core: Pay
    Core->>Lightning: Execute Payment
    Lightning-->>Core: Payment Confirmation
    Core->>Web5: Store Receipt
    Web5-->>Core: Confirm
    Core-->>Mobile: Success
    Mobile-->>User: Result
```

## Lightning Network Component Flow

```mermaid
graph TB
    subgraph LightningNode[Lightning Node]
        NodeInfo[Node Management]
        PeerConn[Peer Connections]
        ChannelMgmt[Channel Management]
        InvoiceMgmt[Invoice Management]
        PaymentMgmt[Payment Management]
    end

    subgraph Bridge[Bitcoin-Lightning Bridge]
        Funding[Channel Funding]
        Monitoring[Blockchain Monitoring]
        Closing[Channel Closing]
    end

    subgraph BitcoinIntegration[Bitcoin Integration]
        Wallet[Bitcoin Wallet]
        UTXO[UTXO Management]
        TxBroadcast[Transaction Broadcasting]
    end

    %% Connections
    NodeInfo --> PeerConn
    PeerConn --> ChannelMgmt
    ChannelMgmt --> Bridge
    ChannelMgmt --> InvoiceMgmt
    InvoiceMgmt --> PaymentMgmt
    
    Bridge --> Funding
    Bridge --> Monitoring
    Bridge --> Closing
    
    Funding --> BitcoinIntegration
    Closing --> BitcoinIntegration
    Monitoring --> BitcoinIntegration
    
    BitcoinIntegration --> Wallet
    BitcoinIntegration --> UTXO
    BitcoinIntegration --> TxBroadcast
```

## Security Model

```mermaid
graph TB
    subgraph Security[Security Layer]
        Auth[Authentication]
        Crypto[Cryptography]
        Access[Access Control]
        Audit[Audit System]
    end

    subgraph Components[System Components]
        Core[Core System]
        AI[AI Engine]
        Data[Data Layer]
        API[API Layer]
        Bitcoin[Bitcoin & Lightning]
    end

    Security --> Components
```

## Data Flow

```mermaid
graph LR
    subgraph Input[Input Layer]
        Mobile[Mobile]
        Web[Web]
        API[API]
        Payment[Payment Channels]
    end

    subgraph Processing[Processing Layer]
        Core[Core Engine]
        dash33[dash33 AI]
        Analytics[Analytics]
        LightningNode[Lightning Node]
    end

    subgraph Storage[Storage Layer]
        Web5[Web5 Storage]
        Local[Local Storage]
        Cache[Cache]
        ChannelDB[Channel State DB]
    end

    Input --> Processing
    Processing --> Storage
    Payment --> LightningNode
    LightningNode --> ChannelDB
```

## Development Workflow

```mermaid
graph LR
    Dev[Development]
    Test[Testing]
    Review[Review]
    Deploy[Deployment]

    Dev --> Test
    Test --> Review
    Review --> Deploy
    Deploy -.-> Dev
```

## Monitoring and Metrics

```mermaid
graph TB
    subgraph Collection[Data Collection]
        Perf[Performance]
        Usage[Usage]
        Errors[Errors]
        LightningMetrics[Lightning Metrics]
    end

    subgraph Analysis[Analysis]
        AI[AI Processing]
        Stats[Statistics]
        Alerts[Alerts]
        ChannelHealth[Channel Health]
    end

    subgraph Actions[Actions]
        Scale[Auto-scaling]
        Notify[Notifications]
        Log[Logging]
        ChannelBalancing[Channel Balancing]
    end

    Collection --> Analysis
    Analysis --> Actions
    LightningMetrics --> ChannelHealth
    ChannelHealth --> ChannelBalancing
```
