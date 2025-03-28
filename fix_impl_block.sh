#!/bin/bash
set -e

echo "Fixing the impl block in l4_protocol/mod.rs..."

# Create a backup
cp core/src/l4_protocol/mod.rs core/src/l4_protocol/mod.rs.bak

# First, check if our additional methods were added (and remove them if they were)
sed -i '/pub fn create_dlc_contract_sync/,/self.hsm_initialized/d' core/src/l4_protocol/mod.rs

# Now find the end of the impl AnyaL4Protocol block and insert our methods there
awk '
/impl AnyaL4Protocol/ {
    in_impl = 1;
}
/^}/ && in_impl {
    print "    // Add a synchronous version of create_dlc_contract for testing"
    print "    pub fn create_dlc_contract_sync("
    print "        &self,"
    print "        oracle_pubkey: PublicKey,"
    print "        outcomes: Vec<String>,"
    print "    ) -> DlcContract {"
    print "        // Create a contract with non-interactive oracle pattern"
    print "        DlcContract::new_non_interactive(oracle_pubkey)"
    print "            .with_outcomes(outcomes)"
    print "    }"
    print ""
    print "    // Getter for the endpoint for testing"
    print "    pub fn get_endpoint(&self) -> String {"
    print "        if self.rpc_adapter.endpoints.is_empty() {"
    print "            String::new()"
    print "        } else {"
    print "            self.rpc_adapter.endpoints[0].clone()"
    print "        }"
    print "    }"
    print ""
    print "    // Getter for HSM initialization status"
    print "    pub fn is_hsm_initialized(&self) -> bool {"
    print "        self.hsm_initialized"
    print "    }"
    in_impl = 0;
}
{ print $0 }
' core/src/l4_protocol/mod.rs.bak > core/src/l4_protocol/mod.rs

echo "Checking if the fix worked..."
cargo check -p core

echo "Now running tests..."
cargo test -p core --lib
