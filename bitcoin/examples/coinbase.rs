// SPDX-License-Identifier: CC0-1.0

//! Demonstrates building a coinbase transaction with BIP-34 block height, Stratum extranonce fields, and BIP-141 witness commitment.
//!
//! This example shows how to:
//! 1. Build a coinbase scriptSig with BIP-34 block height encoding
//! 2. Include Stratum mining extranonce1 and extranonce2 fields
//! 3. Create a BIP-141 compliant witness commitment output
//! 4. Construct the complete coinbase transaction
//!
//! BIP-34 requires the block height to be the first item in coinbase scriptSig as a script integer.
//! Stratum mining protocol uses extranonce1 (server-assigned) and extranonce2 (miner-assigned) for nonce space extension.
//! BIP-141 requires a witness commitment in one of the coinbase outputs with specific format.

use bitcoin::blockdata::script::Builder;
use bitcoin::consensus::encode;
use bitcoin::hashes::{hash160, sha256d};
use bitcoin::opcodes::all::*;
use bitcoin::script::ScriptBuf;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::transaction::TransactionExt;
use bitcoin::{Amount, Network, PrivateKey, PublicKey, Transaction, Witness};
use primitives::absolute;
use primitives::sequence::Sequence;
use primitives::transaction::{OutPoint, TxIn, TxOut};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating BIP-34 + Stratum + BIP-141 compliant coinbase transaction");

    // Example block height for BIP-34 compliance
    let block_height: u32 = 123456;
    
    // Stratum mining extranonce fields
    let extranonce1 = [0x12, 0x34, 0x56, 0x78]; // 4-byte server-assigned extranonce1 (example)
    let extranonce2 = [0xab, 0xcd, 0xef, 0x00]; // 4-byte miner-assigned extranonce2 (example)
    
    // Build BIP-34 + Stratum compliant coinbase scriptSig
    // Structure: block_height extranonce1 extranonce2 arbitrary_data
    let coinbase_scriptsig = Builder::new()
        .push_int(block_height as i32)? // BIP-34: block height must be first
        .push_slice(&extranonce1) // Stratum: server-assigned extranonce1
        .push_slice(&extranonce2) // Stratum: miner-assigned extranonce2  
        .push_slice(b"/rust-bitcoin/") // Pool/miner identification
        .into_script();
    
    println!("Coinbase scriptSig with BIP-34 height + Stratum extranonces: {}", coinbase_scriptsig);
    println!("  Block height: {}", block_height);
    println!("  Extranonce1 (server): {:02x?}", extranonce1);
    println!("  Extranonce2 (miner):  {:02x?}", extranonce2);
    
    // Create coinbase input - demonstrate both cases
    println!("\n=== Demonstrating Both Coinbase Witness Cases ===");
    
    // Case 1: Empty witness (current behavior)
    let coinbase_input_empty = TxIn {
        previous_output: OutPoint::COINBASE_PREVOUT,
        script_sig: coinbase_scriptsig.clone(),
        sequence: Sequence::MAX,
        witness: Witness::default(), // Empty witness
    };
    
    // Case 2: With 32-byte witness reserved value (for SegWit blocks)
    let coinbase_input_witness = TxIn {
        previous_output: OutPoint::COINBASE_PREVOUT,
        script_sig: coinbase_scriptsig,
        sequence: Sequence::MAX,
        witness: Witness::from_slice(&[vec![0u8; 32]]), // 32-byte witness reserved value
    };
    
    // Generate a random private key for the coinbase reward
    let secp = Secp256k1::new();
    let private_key = PrivateKey::generate(Network::Bitcoin);
    let public_key = PublicKey::from_private_key(&secp, private_key);
    
    // Create P2PKH output for block reward
    let reward_script = Builder::new()
        .push_opcode(OP_DUP)
        .push_opcode(OP_HASH160)
        .push_slice(&hash160::Hash::hash(&public_key.to_bytes()).to_byte_array())
        .push_opcode(OP_EQUALVERIFY)
        .push_opcode(OP_CHECKSIG)
        .into_script();
    
    let block_reward = Amount::from_sat(50_00000000)?; // 50 BTC block reward (pre-halving example)
    let reward_output = TxOut {
        value: block_reward,
        script_pubkey: reward_script,
    };
    
    // Create BIP-141 witness commitment output
    // Format: OP_RETURN OP_PUSHBYTES_36 0x6a24aa21a9ed{32-byte commitment}
    let witness_root_hash = sha256d::Hash::from_byte_array([0u8; 32]); // Placeholder - would be merkle root of witness data in block
    let witness_reserved_value = [0u8; 32]; // Reserved value for future extensions
    
    // Calculate witness commitment: Double-SHA256(witness_root_hash || witness_reserved_value)
    let mut commitment_data = Vec::new();
    commitment_data.extend_from_slice(&witness_root_hash.to_byte_array());
    commitment_data.extend_from_slice(&witness_reserved_value);
    let witness_commitment = sha256d::Hash::hash(&commitment_data);
    
    // Build the witness commitment scriptPubKey
    let mut commitment_script_data = Vec::new();
    commitment_script_data.extend_from_slice(&[0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed]); // Magic bytes
    commitment_script_data.extend_from_slice(&witness_commitment.to_byte_array());
    
    let commitment_output = TxOut {
        value: Amount::ZERO, // Commitment output has zero value
        script_pubkey: ScriptBuf::from_bytes(commitment_script_data),
    };
    
    // Construct both versions of coinbase transaction
    let coinbase_tx_empty = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: vec![coinbase_input_empty],
        output: vec![reward_output.clone(), commitment_output.clone()],
    };
    
    let coinbase_tx_witness = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: vec![coinbase_input_witness],
        output: vec![reward_output, commitment_output],
    };
    
    println!("\n=== CASE 1: Coinbase WITHOUT Witness ===");
    println!("Transaction ID: {}", coinbase_tx_empty.compute_txid());
    println!("Witness Transaction ID: {}", coinbase_tx_empty.compute_wtxid());
    println!("txid == wtxid: {}", coinbase_tx_empty.compute_txid().to_string() == coinbase_tx_empty.compute_wtxid().to_string());
    println!("Transaction size: {} bytes", encode::serialize(&coinbase_tx_empty).len());
    println!("Uses SegWit serialization: {}", coinbase_tx_empty.input.iter().any(|input| !input.witness.is_empty()));
    
    println!("\n=== CASE 2: Coinbase WITH 32-byte Witness ===");
    println!("Transaction ID: {}", coinbase_tx_witness.compute_txid());
    println!("Witness Transaction ID: {}", coinbase_tx_witness.compute_wtxid());
    println!("txid == wtxid: {}", coinbase_tx_witness.compute_txid().to_string() == coinbase_tx_witness.compute_wtxid().to_string());
    println!("Transaction size: {} bytes", encode::serialize(&coinbase_tx_witness).len());
    println!("Uses SegWit serialization: {}", coinbase_tx_witness.input.iter().any(|input| !input.witness.is_empty()));
    
    println!("\n=== MERKLE ROOT IMPLICATIONS ===");
    println!("Regular Merkle Tree (block header):");
    println!("  Case 1 txid: {}", coinbase_tx_empty.compute_txid());
    println!("  Case 2 txid: {}", coinbase_tx_witness.compute_txid());
    println!("  Same for regular merkle: {}", coinbase_tx_empty.compute_txid() == coinbase_tx_witness.compute_txid());
    
    println!("Witness Merkle Tree (commitment):");
    println!("  Case 1 wtxid: {}", coinbase_tx_empty.compute_wtxid());
    println!("  Case 2 wtxid: {}", coinbase_tx_witness.compute_wtxid());
    println!("  Same for witness merkle: {}", coinbase_tx_empty.compute_wtxid().to_string() == coinbase_tx_witness.compute_wtxid().to_string());
    
    println!("\n--- Stratum Mining Information ---");
    println!("Extranonce1 is assigned by the mining pool/server and remains constant for a session");
    println!("Extranonce2 is controlled by the miner and can be incremented to create unique work");
    println!("Together they extend the nonce space beyond the 32-bit header nonce field");
    
    println!("\n=== BITCOIN CORE ACCEPTANCE ===");
    println!("Bitcoin Core accepts BOTH formats:");
    println!("1. Empty witness coinbase (most common)");
    println!("2. 32-byte witness coinbase (required for SegWit blocks with witness commitment)");
    println!();
    println!("KEY POINT: The witness commitment calculation must match the actual wtxid used!");
    println!("If block contains SegWit transactions, coinbase MUST have witness reserved value.");
    
    // Verify BIP-34 compliance (using empty witness version)
    let scriptsig_bytes = coinbase_tx_empty.input[0].script_sig.as_bytes();
    if scriptsig_bytes.len() >= 4 && scriptsig_bytes[0] == 0x03 {
        let height_bytes = &scriptsig_bytes[1..4];
        let decoded_height = u32::from_le_bytes([height_bytes[0], height_bytes[1], height_bytes[2], 0]);
        println!("BIP-34 block height decoded: {}", decoded_height);
        assert_eq!(decoded_height, block_height);
        println!("✓ BIP-34 compliance verified");
    }
    
    // Show serialization differences
    println!("\n=== SERIALIZATION COMPARISON ===");
    println!("Case 1 (empty witness) hex: {}", encode::serialize_hex(&coinbase_tx_empty));
    println!("Case 2 (with witness) hex:  {}", encode::serialize_hex(&coinbase_tx_witness));
    
    Ok(())
}