#![allow(missing_docs)]

use darling as _;
use quote as _;
use ssz::{Decode, Encode};
use ssz_derive::{Decode, Encode};
use ssz_types::{BitVector, Optional};
use syn as _;

type AccountId = [u8; 32];

/// Union with variable-length content
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[ssz(enum_behaviour = "union")]
enum TransactionPayload {
    GenericAccountMessage(GamTxPayload),
    SnarkAccountUpdate(SnarkAccountUpdateTxPayload),
}

/// Container with variable-length field (Vec encodes as variable-length)
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
struct GamTxPayload {
    target: AccountId,
    payload: Vec<u8>,
}

/// Container with fixed-length field
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
struct SnarkAccountUpdateTxPayload {
    target: AccountId,
    data: u64,
}

/// Nested StableContainer
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[ssz(struct_behaviour = "stable_container", max_fields = 16)]
struct TransactionAttachment {
    min_slot: Optional<u64>,
    max_slot: Optional<u64>,
}

/// Top-level StableContainer with variable-length fields
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[ssz(struct_behaviour = "stable_container", max_fields = 16)]
struct OLTransaction {
    payload: Optional<TransactionPayload>,
    attachment: Optional<TransactionAttachment>,
}

#[test]
fn test_transaction_with_gam() {
    let tx = OLTransaction {
        payload: Optional::Some(TransactionPayload::GenericAccountMessage(GamTxPayload {
            target: [0u8; 32],
            payload: vec![],
        })),
        attachment: Optional::Some(TransactionAttachment {
            min_slot: Optional::None,
            max_slot: Optional::None,
        }),
    };

    // Debug: check field properties
    eprintln!(
        "TransactionPayload is_fixed_len: {}",
        <TransactionPayload as ssz::Encode>::is_ssz_fixed_len()
    );
    eprintln!(
        "TransactionPayload fixed_len: {}",
        <TransactionPayload as ssz::Encode>::ssz_fixed_len()
    );
    eprintln!(
        "TransactionAttachment is_fixed_len: {}",
        <TransactionAttachment as ssz::Encode>::is_ssz_fixed_len()
    );
    eprintln!(
        "TransactionAttachment fixed_len: {}",
        <TransactionAttachment as ssz::Encode>::ssz_fixed_len()
    );

    let attachment = TransactionAttachment {
        min_slot: Optional::None,
        max_slot: Optional::None,
    };
    let attachment_encoded = attachment.as_ssz_bytes();
    eprintln!("TransactionAttachment encoded: {:?}", attachment_encoded);
    eprintln!(
        "TransactionAttachment encoded length: {}",
        attachment_encoded.len()
    );

    let encoded = tx.as_ssz_bytes();
    eprintln!("Transaction encoded: {:?}", encoded);
    eprintln!("Transaction length: {}", encoded.len());

    let decoded = OLTransaction::from_ssz_bytes(&encoded).unwrap();
    assert_eq!(tx, decoded);
}

#[test]
fn test_transaction_with_snark_update() {
    let tx = OLTransaction {
        payload: Optional::Some(TransactionPayload::SnarkAccountUpdate(
            SnarkAccountUpdateTxPayload {
                target: [1u8; 32],
                data: 42,
            },
        )),
        attachment: Optional::None,
    };

    let encoded = tx.as_ssz_bytes();
    eprintln!("Snark update encoded: {:?}", encoded);
    eprintln!("Snark update length: {}", encoded.len());

    let decoded = OLTransaction::from_ssz_bytes(&encoded).unwrap();
    assert_eq!(tx, decoded);
}

#[test]
fn test_transaction_empty() {
    let tx = OLTransaction {
        payload: Optional::None,
        attachment: Optional::None,
    };

    let encoded = tx.as_ssz_bytes();
    eprintln!("Empty transaction encoded: {:?}", encoded);
    eprintln!("Empty transaction length: {}", encoded.len());

    let decoded = OLTransaction::from_ssz_bytes(&encoded).unwrap();
    assert_eq!(tx, decoded);
}
