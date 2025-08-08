# Layer2 Production Runbook

This runbook describes how to operate Anya Core’s Layer2 stack in production, with self-node primary behavior by default and automatic peer adoption when peers are available.

## Defaults

- Prefer self as master: enabled by default
- Self-node fallback: enabled by default
- Real networking: enabled by default (can be disabled for simulation)

## Configuration

Primary config file: ~/.config/anya-core/layer2.toml

Global toggles:

- prefer_self_as_master = true
- enable_self_node_fallback = true
- enable_real_networking = true
- min_peers = 2

See config/examples/layer2.toml for a template.

Environment overrides:

- ANYA_LAYER2_PREFER_SELF_AS_MASTER=true|false
- ANYA_LAYER2_ENABLE_SELF_NODE_FALLBACK=true|false
- ANYA_LAYER2_ENABLE_REAL_NETWORKING=true|false

## Wiring Real Nodes

Lightning:

- node_url: host:port or https endpoint
- macaroon: hex string
- cert: base64 string
- bootstrap_peers: optional static peers

RGB:

- endpoint: URL to RGB node/core
- min_peers: often 1 is acceptable

DLC:

- oracle_endpoint and bootstrap peers

Taproot Assets:

- bitcoin_rpc_url, user, pass, network

## Operational Checks

- Health (per protocol): healthy when Synced and (peer_count >= min_peers OR is_primary)
- State: version, connections, height, hash, timestamp

## Smoke Tests

- Connect + sync: expect Synced and is_primary=true by default
- Submit tx: ensure it’s recorded and broadcast attempts occur when peers exist
- Proof generation/verification: validate happy path

## Failure Modes

- No peers reachable: self-node activates, is_primary=true, health still OK
- Real networking disabled: simulate or self-node per fallback flag
- Low peer count: remain primary if prefer_self_as_master=true

## Notes

- Primary flag is internal to the networking state; use for ops dashboards, not public APIs.
- Disable prefer_self_as_master to force external leadership when integrating with managed clusters.
