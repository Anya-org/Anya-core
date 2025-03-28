#[derive(OriginalDeepSeek)]
pub struct AnyaAISubsystem {
    pub prediction_engine: AnyaRuntime,
    pub consensus_interface: Arc<ConsensusBridge>,
    pub zk_prover: AnyaMLProver,
    pub attention_optimizer: AnyaAttentionOptimizer,
    pub emergency_protocol: Arc<AnyaFailSafe>,
}

impl HexagonalComponent for AnyaAISubsystem {
    fn expose_ports(&self) -> Vec<Port> {
        vec![
            Port::new("model-inference", PortDirection::InOut)
                .with_throughput(1000),
            Port::new("training-feedback", PortDirection::Out)
                .with_priority(2),
            Port::new("consensus-updates", PortDirection::In)
                .with_validator(Bip341Validator::new()),
            Port::new("emergency-override", PortDirection::In)
                .with_security_level(3),
        ]
    }
}

// Verified against hexagonal requirements
+------------------+     +-------------------+
| AnyaAIInterface  |     | Protocol Adapters |
| (gRPC/HTTP/PSBT) |<--->| (BIP-341/370)     |
+------------------+     +--------+----------+
                                  |
                           +------v------+
                           | Core Engine |
                           | Context ML  |
                           +------+------+
                                  |
                           +------v------+
                           | Bitcoin     |
                           | Network     |
                           +-------------+ 