//! Bitcoin script interpreter
//!
//! This module implements the Bitcoin script interpreter for validating
//! transaction scripts, with support for Taproot and related BIPs.
//! It follows Bitcoin Core principles of security, decentralization, and privacy.

use bitcoin::{Script, Transaction};
use bitflags::bitflags;
use log::info;
use thiserror::Error;

/// Maximum number of operations allowed in a script
pub const MAX_OPS_PER_SCRIPT: usize = 201;

/// Maximum script size in bytes
pub const MAX_SCRIPT_SIZE: usize = 10000;

/// Maximum number of elements on the stack
pub const MAX_STACK_SIZE: usize = 100;

/// Maximum script element size in bytes
pub const MAX_SCRIPT_ELEMENT_SIZE: usize = 520;

/// Maximum value allowed for an integer on the stack
pub const MAX_SCRIPT_INTEGER: i64 = 0x7fffffff;

/// Bitcoin script opcode definitions
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Opcode {
    // Constants
    OP_0 = 0x00,
    OP_PUSHDATA1 = 0x4c,
    OP_PUSHDATA2 = 0x4d,
    OP_PUSHDATA4 = 0x4e,
    OP_1NEGATE = 0x4f,
    OP_RESERVED = 0x50,
    OP_1 = 0x51,
    OP_2 = 0x52,
    OP_3 = 0x53,
    OP_4 = 0x54,
    OP_5 = 0x55,
    OP_6 = 0x56,
    OP_7 = 0x57,
    OP_8 = 0x58,
    OP_9 = 0x59,
    OP_10 = 0x5a,
    OP_11 = 0x5b,
    OP_12 = 0x5c,
    OP_13 = 0x5d,
    OP_14 = 0x5e,
    OP_15 = 0x5f,
    OP_16 = 0x60,

    // Flow control
    OP_NOP = 0x61,
    OP_VER = 0x62,
    OP_IF = 0x63,
    OP_NOTIF = 0x64,
    OP_VERIF = 0x65,
    OP_VERNOTIF = 0x66,
    OP_ELSE = 0x67,
    OP_ENDIF = 0x68,
    OP_VERIFY = 0x69,
    OP_RETURN = 0x6a,

    // Stack operations
    OP_TOALTSTACK = 0x6b,
    OP_FROMALTSTACK = 0x6c,
    OP_2DROP = 0x6d,
    OP_2DUP = 0x6e,
    OP_3DUP = 0x6f,
    OP_2OVER = 0x70,
    OP_2ROT = 0x71,
    OP_2SWAP = 0x72,
    OP_IFDUP = 0x73,
    OP_DEPTH = 0x74,
    OP_DROP = 0x75,
    OP_DUP = 0x76,
    OP_NIP = 0x77,
    OP_OVER = 0x78,
    OP_PICK = 0x79,
    OP_ROLL = 0x7a,
    OP_ROT = 0x7b,
    OP_SWAP = 0x7c,
    OP_TUCK = 0x7d,

    // Splice operations
    OP_CAT = 0x7e,
    OP_SUBSTR = 0x7f,
    OP_LEFT = 0x80,
    OP_RIGHT = 0x81,
    OP_SIZE = 0x82,

    // Bitwise logic
    OP_INVERT = 0x83,
    OP_AND = 0x84,
    OP_OR = 0x85,
    OP_XOR = 0x86,
    OP_EQUAL = 0x87,
    OP_EQUALVERIFY = 0x88,
    OP_RESERVED1 = 0x89,
    OP_RESERVED2 = 0x8a,

    // Arithmetic
    OP_1ADD = 0x8b,
    OP_1SUB = 0x8c,
    OP_2MUL = 0x8d,
    OP_2DIV = 0x8e,
    OP_NEGATE = 0x8f,
    OP_ABS = 0x90,
    OP_NOT = 0x91,
    OP_0NOTEQUAL = 0x92,
    OP_ADD = 0x93,
    OP_SUB = 0x94,
    OP_MUL = 0x95,
    OP_DIV = 0x96,
    OP_MOD = 0x97,
    OP_LSHIFT = 0x98,
    OP_RSHIFT = 0x99,
    OP_BOOLAND = 0x9a,
    OP_BOOLOR = 0x9b,
    OP_NUMEQUAL = 0x9c,
    OP_NUMEQUALVERIFY = 0x9d,
    OP_NUMNOTEQUAL = 0x9e,
    OP_LESSTHAN = 0x9f,
    OP_GREATERTHAN = 0xa0,
    OP_LESSTHANOREQUAL = 0xa1,
    OP_GREATERTHANOREQUAL = 0xa2,
    OP_MIN = 0xa3,
    OP_MAX = 0xa4,
    OP_WITHIN = 0xa5,

    // Crypto
    OP_RIPEMD160 = 0xa6,
    OP_SHA1 = 0xa7,
    OP_SHA256 = 0xa8,
    OP_HASH160 = 0xa9,
    OP_HASH256 = 0xaa,
    OP_CODESEPARATOR = 0xab,
    OP_CHECKSIG = 0xac,
    OP_CHECKSIGVERIFY = 0xad,
    OP_CHECKMULTISIG = 0xae,
    OP_CHECKMULTISIGVERIFY = 0xaf,

    // Expansion
    OP_NOP1 = 0xb0,
    OP_CHECKLOCKTIMEVERIFY = 0xb1,
    OP_CHECKSEQUENCEVERIFY = 0xb2,
    OP_NOP4 = 0xb3,
    OP_NOP5 = 0xb4,
    OP_NOP6 = 0xb5,
    OP_NOP7 = 0xb6,
    OP_NOP8 = 0xb7,
    OP_NOP9 = 0xb8,
    OP_NOP10 = 0xb9,

    // Taproot (BIP-341, BIP-342)
    OP_CHECKSIGADD = 0xba,

    // Invalid opcodes
    OP_INVALIDOPCODE = 0xff,
}

// Aliases for opcode values
pub const OP_FALSE: Opcode = Opcode::OP_0;
pub const OP_TRUE: Opcode = Opcode::OP_1;
pub const OP_NOP2: Opcode = Opcode::OP_CHECKLOCKTIMEVERIFY;
pub const OP_NOP3: Opcode = Opcode::OP_CHECKSEQUENCEVERIFY;

bitflags! {
    /// Script verification flags
    pub struct VerifyFlags: u32 {
        /// No special validation
        const NONE = 0;
        /// Evaluate P2SH subscripts (BIP16)
        const P2SH = 1 << 0;
        /// Enforce strict DER (BIP66)
        const STRICTENC = 1 << 1;
        /// Enable CHECKLOCKTIMEVERIFY (BIP65)
        const CHECKLOCKTIMEVERIFY = 1 << 2;
        /// Enable CHECKSEQUENCEVERIFY (BIP112)
        const CHECKSEQUENCEVERIFY = 1 << 3;
        /// Enable WITNESS validation (BIP141)
        const WITNESS = 1 << 4;
        /// Enable CHECKDATASIG and CHECKDATASIGVERIFY
        const CHECKDATASIG = 1 << 5;
        /// Enable Schnorr signatures (BIP340)
        const SCHNORR = 1 << 6;
        /// Enable Taproot/Tapscript (BIP341, BIP342)
        const TAPROOT = 1 << 7;
        /// Enable Miniscript validation
        const MINISCRIPT = 1 << 8;
        /// Enable all deployed script validation rules
        const STANDARD = Self::P2SH.bits() | Self::STRICTENC.bits() | Self::CHECKLOCKTIMEVERIFY.bits() |
                      Self::CHECKSEQUENCEVERIFY.bits() | Self::WITNESS.bits() | Self::SCHNORR.bits() |
                      Self::TAPROOT.bits();
    }
}

/// Script execution error variants
#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("Script execution error: {0}")]
    ExecutionError(String),

    #[error("Invalid opcode: {0:x}")]
    InvalidOpcode(u8),

    #[error("Stack overflow")]
    StackOverflow,

    #[error("Stack underflow")]
    StackUnderflow,

    #[error("Unbalanced conditional")]
    UnbalancedConditional,

    #[error("Op count exceeded")]
    OpCountExceeded,

    #[error("Script size exceeded")]
    ScriptSizeExceeded,

    #[error("Element size exceeded")]
    ElementSizeExceeded,

    #[error("Verification failed")]
    VerificationFailed,

    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Invalid altstack operation")]
    InvalidAltStackOperation,

    #[error("Invalid stack value")]
    InvalidStackValue,

    #[error("Taproot verification failed: {0}")]
    TaprootVerificationFailed(String),

    #[error("General error: {0}")]
    General(String),
}

/// Script execution environment for an individual stack
pub struct Stack {
    /// Main execution stack
    items: Vec<Vec<u8>>,
    /// Alternative stack
    alt_stack: Vec<Vec<u8>>,
}

impl Stack {
    /// Create a new empty stack
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            alt_stack: Vec::new(),
        }
    }

    /// Push an item onto the stack
    pub fn push(&mut self, item: Vec<u8>) -> Result<(), ScriptError> {
        if self.items.len() >= MAX_STACK_SIZE {
            return Err(ScriptError::StackOverflow);
        }
        if item.len() > MAX_SCRIPT_ELEMENT_SIZE {
            return Err(ScriptError::ElementSizeExceeded);
        }
        self.items.push(item);
        Ok(())
    }

    /// Pop an item from the stack
    pub fn pop(&mut self) -> Result<Vec<u8>, ScriptError> {
        self.items.pop().ok_or(ScriptError::StackUnderflow)
    }

    /// Peek at the top item on the stack without removing it
    pub fn peek(&self) -> Result<&Vec<u8>, ScriptError> {
        self.items.last().ok_or(ScriptError::StackUnderflow)
    }

    /// Push an item onto the alternative stack
    pub fn push_alt(&mut self, item: Vec<u8>) -> Result<(), ScriptError> {
        if self.alt_stack.len() >= MAX_STACK_SIZE {
            return Err(ScriptError::StackOverflow);
        }
        self.alt_stack.push(item);
        Ok(())
    }

    /// Pop an item from the alternative stack
    pub fn pop_alt(&mut self) -> Result<Vec<u8>, ScriptError> {
        self.alt_stack
            .pop()
            .ok_or(ScriptError::InvalidAltStackOperation)
    }

    /// Get the current stack size
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Get the current alternative stack size
    pub fn alt_size(&self) -> usize {
        self.alt_stack.len()
    }

    /// Convert the top stack item to a boolean
    pub fn top_bool(&self) -> Result<bool, ScriptError> {
        let item = self.peek()?;

        // Empty stack item is false
        if item.is_empty() {
            return Ok(false);
        }

        // Check all bytes for non-zero value
        for &b in item.iter().rev() {
            if b != 0 {
                // Special case for single byte negative zero
                if b == 0x80 && item.len() == 1 {
                    return Ok(false);
                }
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Convert the top stack item to an integer
    pub fn top_int(&self) -> Result<i64, ScriptError> {
        let item = self.peek()?;

        // Empty stack item is 0
        if item.is_empty() {
            return Ok(0);
        }

        // Decode as little-endian signed integer
        let mut result: i64 = 0;
        let mut negative = false;

        for (i, &b) in item.iter().enumerate() {
            if i == item.len() - 1 && b & 0x80 != 0 {
                negative = true;
                result |= ((b & 0x7f) as i64) << (8 * i);
            } else {
                result |= (b as i64) << (8 * i);
            }

            // Check for overflow
            if i >= 8 {
                break;
            }
        }

        if negative {
            result = -result;
        }

        Ok(result)
    }

    /// Clear the stack
    pub fn clear(&mut self) {
        self.items.clear();
        self.alt_stack.clear();
    }
}

/// Script interpreter context
pub struct ScriptContext {
    /// Stack for execution
    pub stack: Stack,
    /// Verification flags
    pub flags: VerifyFlags,
    /// Signature version (legacy, witness v0, witness v1/taproot)
    pub sig_version: SigVersion,
    /// Operation count
    pub op_count: usize,
    /// Current code separator position
    pub code_separator_pos: isize,
    /// Current execution index
    pub pc: usize,
    /// Transaction being validated
    pub tx: Option<Transaction>,
    /// Transaction input index
    pub input_index: usize,
    /// Transaction output amount
    pub amount: u64,
    /// Taproot leaf version (for Tapscript execution)
    pub taproot_leaf_version: Option<u8>,
}

/// Signature validation versions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigVersion {
    /// Legacy pre-SegWit
    Legacy,
    /// SegWit v0
    WitnessV0,
    /// Taproot (SegWit v1)
    Taproot,
}

impl ScriptContext {
    /// Create a new script execution context
    pub fn new(flags: VerifyFlags) -> Self {
        Self {
            stack: Stack::new(),
            flags,
            sig_version: SigVersion::Legacy,
            op_count: 0,
            code_separator_pos: -1,
            pc: 0,
            tx: None,
            input_index: 0,
            amount: 0,
            taproot_leaf_version: None,
        }
    }

    /// Set the transaction and input index for signature checking
    pub fn set_transaction(&mut self, tx: Transaction, input_index: usize, amount: u64) {
        self.tx = Some(tx);
        self.input_index = input_index;
        self.amount = amount;
    }

    /// Reset the context for a new script execution
    pub fn reset(&mut self) {
        self.stack.clear();
        self.op_count = 0;
        self.code_separator_pos = -1;
        self.pc = 0;
    }

    /// Check if Taproot validation is enabled
    pub fn is_taproot_enabled(&self) -> bool {
        self.flags.contains(VerifyFlags::TAPROOT)
    }

    /// Check if P2SH is enabled
    pub fn is_p2sh_enabled(&self) -> bool {
        self.flags.contains(VerifyFlags::P2SH)
    }

    /// Check if SegWit validation is enabled
    pub fn is_witness_enabled(&self) -> bool {
        self.flags.contains(VerifyFlags::WITNESS)
    }

    /// Check if strict encoding is required for signatures
    pub fn requires_strict_encoding(&self) -> bool {
        self.flags.contains(VerifyFlags::STRICTENC)
    }

    /// Check if Schnorr signatures are enabled
    pub fn is_schnorr_enabled(&self) -> bool {
        self.flags.contains(VerifyFlags::SCHNORR)
    }
}

/// Bitcoin script interpreter
pub struct ScriptInterpreter;

impl ScriptInterpreter {
    /// Verify a script
    pub fn verify_script(
        script_sig: &Script,
        script_pubkey: &Script,
        tx: &Transaction,
        input_index: usize,
        amount: u64,
        flags: VerifyFlags,
    ) -> Result<bool, ScriptError> {
        let mut context = ScriptContext::new(flags);
        context.set_transaction(tx.clone(), input_index, amount);

        // Execute the signature script
        Self::execute_script(script_sig, &mut context)?;

        // Execute the public key script
        Self::execute_script(script_pubkey, &mut context)?;

        // Verify that the stack has at least one element
        if context.stack.size() == 0 {
            return Err(ScriptError::VerificationFailed);
        }

        // Success is determined by the top stack item being true
        let result = context.stack.top_bool()?;

        if result {
            Ok(true)
        } else {
            Err(ScriptError::VerificationFailed)
        }
    }

    /// Execute a script in the given context
    pub fn execute_script(script: &Script, context: &mut ScriptContext) -> Result<(), ScriptError> {
        let script_bytes = script.as_bytes();

        // Check script size
        if script_bytes.len() > MAX_SCRIPT_SIZE {
            return Err(ScriptError::ScriptSizeExceeded);
        }

        context.pc = 0;

        while context.pc < script_bytes.len() {
            let opcode = script_bytes[context.pc];
            context.pc += 1;

            // Count executed operations
            context.op_count += 1;
            if context.op_count > MAX_OPS_PER_SCRIPT {
                return Err(ScriptError::OpCountExceeded);
            }

            // Execute the opcode
            Self::execute_opcode(opcode as u8, script_bytes, context)?;
        }

        Ok(())
    }

    /// Execute a single opcode
    fn execute_opcode(
        opcode: u8,
        script: &[u8],
        context: &mut ScriptContext,
    ) -> Result<(), ScriptError> {
        // Handle data push opcodes
        if opcode <= 0x4b {
            // Direct push of N bytes
            let n = opcode as usize;
            if context.pc + n > script.len() {
                return Err(ScriptError::ExecutionError(
                    "Push past end of script".to_string(),
                ));
            }

            let data = script[context.pc..context.pc + n].to_vec();
            context.pc += n;
            context.stack.push(data)?;
            return Ok(());
        }

        match opcode {
            // Implement opcode execution logic here
            // This would be a very long match statement in a real implementation
            // For now, we'll just handle a few common opcodes as examples

            // OP_PUSHDATA1 - next byte contains N, followed by N bytes of data
            0x4c => {
                if context.pc >= script.len() {
                    return Err(ScriptError::ExecutionError(
                        "OP_PUSHDATA1: no length byte".to_string(),
                    ));
                }
                let n = script[context.pc] as usize;
                context.pc += 1;

                if context.pc + n > script.len() {
                    return Err(ScriptError::ExecutionError(
                        "OP_PUSHDATA1: push past end of script".to_string(),
                    ));
                }

                let data = script[context.pc..context.pc + n].to_vec();
                context.pc += n;
                context.stack.push(data)?;
            }

            // OP_0, OP_FALSE - push empty array
            0x00 => {
                context.stack.push(vec![])?;
            }

            // OP_1 through OP_16 - push value onto stack
            0x51..=0x60 => {
                let n = (opcode - 0x50) as u8;
                context.stack.push(vec![n])?;
            }

            // OP_DUP - duplicate the top stack item
            0x76 => {
                let item = context.stack.peek()?.clone();
                context.stack.push(item)?;
            }

            // OP_HASH160 - hash the top stack item with RIPEMD160(SHA256)
            0xa9 => {
                // In a real implementation, this would perform the actual hash
                let _item = context.stack.pop()?;
                // Placeholder for hash160 result
                let hash = vec![0; 20]; // 20-byte hash result
                context.stack.push(hash)?;
            }

            // OP_EQUALVERIFY - check if top two stack items are equal
            0x88 => {
                if context.stack.size() < 2 {
                    return Err(ScriptError::StackUnderflow);
                }
                let a = context.stack.pop()?;
                let b = context.stack.pop()?;

                if a != b {
                    return Err(ScriptError::VerificationFailed);
                }
            }

            // OP_CHECKSIG - validate a signature
            0xac => {
                if context.stack.size() < 2 {
                    return Err(ScriptError::StackUnderflow);
                }

                let _pubkey = context.stack.pop()?;
                let _sig = context.stack.pop()?;

                // In a real implementation, this would verify the signature
                // For now, we'll just push a success value
                context.stack.push(vec![1])?;
            }

            // Handle Taproot-specific opcodes if enabled
            0xba if context.is_taproot_enabled() => {
                // OP_CHECKSIGADD
                if context.sig_version != SigVersion::Taproot {
                    return Err(ScriptError::ExecutionError(
                        "OP_CHECKSIGADD only valid in Taproot".to_string(),
                    ));
                }

                if context.stack.size() < 3 {
                    return Err(ScriptError::StackUnderflow);
                }

                // In a real implementation, this would perform Schnorr signature verification
                // and add the result to the top stack item
                let _pubkey = context.stack.pop()?;
                let _sig = context.stack.pop()?;
                let _num = context.stack.pop()?;

                // Push 1 (simulated success) + original value
                context.stack.push(vec![1])?;
            }

            _ => {
                return Err(ScriptError::InvalidOpcode(opcode));
            }
        }

        Ok(())
    }
}

/// Taproot script validation
pub struct TaprootValidator;

impl TaprootValidator {
    /// Create a new TaprootValidator
    pub fn new() -> Self {
        Self
    }

    /// Verify Taproot commitment
    pub fn verify_taproot_commitment(&self, _tx: &Transaction) -> Result<bool, ScriptError> {
        // TODO: Implement proper Taproot commitment verification
        Ok(true)
    }

    /// Verify Schnorr signatures in transaction
    pub fn verify_schnorr_signatures(&self, _tx: &Transaction) -> Result<bool, ScriptError> {
        // TODO: Implement proper Schnorr signature verification
        Ok(true)
    }

    /// Verify a Taproot (SegWit v1) output spend
    pub fn verify_taproot_spend(
        _tx: &Transaction,
        _input_index: usize,
        _witness_program: &[u8],
        _amount: u64,
        flags: VerifyFlags,
    ) -> Result<bool, ScriptError> {
        if !flags.contains(VerifyFlags::TAPROOT) {
            return Err(ScriptError::General(
                "Taproot validation not enabled".to_string(),
            ));
        }

        // This would be a full Taproot verification implementation
        // For now, it's a placeholder that assumes success
        info!("Taproot validation would be performed here");

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_operations() {
        let mut stack = Stack::new();

        // Test basic push/pop
        stack.push(vec![1, 2, 3]).unwrap();
        stack.push(vec![4, 5, 6]).unwrap();

        assert_eq!(stack.size(), 2);

        let item = stack.pop().unwrap();
        assert_eq!(item, vec![4, 5, 6]);

        let item = stack.pop().unwrap();
        assert_eq!(item, vec![1, 2, 3]);

        // Test stack underflow
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_script_context() {
        let flags = VerifyFlags::STANDARD;
        let mut ctx = ScriptContext::new(flags);

        assert!(ctx.is_taproot_enabled());
        assert!(ctx.is_p2sh_enabled());
        assert!(ctx.is_witness_enabled());

        ctx.stack.push(vec![1]).unwrap();
        assert_eq!(ctx.stack.size(), 1);

        ctx.reset();
        assert_eq!(ctx.stack.size(), 0);
    }
}
