use std::{collections::HashSet, io::Cursor};

use chain_gang::{
    messages::{OutPoint, Tx, TxIn, TxOut},
    script::{eval_two_phase, op_codes::*, Script, TxVersionChecker, CONSENSUS_ONLY},
    util::{Hash256, Serializable},
};
use linked_hash_map::LinkedHashMap;
use serde_json::json;

#[test]
fn chronicle_version_conditions_in_complete_transactions() {
    // These cases exercise the actual transaction version and unlocking script.
    // The inputs are synthetic and the prevout is supplied explicitly.
    let cases = [
        (
            "flow-01071",
            "0200000001ae82cc6b9e3b9ca11134f70e84d51a6d6a4fca6d3e11d4c3b493a395450894b800000000050401000000ffffffff010100000000000000015100000000",
            "6651670068",
            true,
        ),
        (
            "flow-01073",
            "020000000181642801742371895602c446791efcc4d487145e4a3aa55fc32faf1a5f4231ba000000000151ffffffff010100000000000000015100000000",
            "6551670068",
            false,
        ),
        (
            "flow-01074",
            "010000000199a7b8176a073eb4777c8474b2481f1c53b8484474f829943c0c43dde2680c4a000000000151ffffffff010100000000000000015100000000",
            "6551670068",
            false,
        ),
    ];
    let mut failures = Vec::new();
    for (name, raw_tx, lock_script, expected) in cases {
        let tx = Tx::read(&mut Cursor::new(hex::decode(raw_tx).unwrap())).unwrap();
        let mut utxos = LinkedHashMap::new();
        utxos.insert(
            tx.inputs[0].prev_output.clone(),
            TxOut {
                satoshis: 1000,
                lock_script: Script(hex::decode(lock_script).unwrap()),
            },
        );
        let result = tx.validate_consensus(true, true, &utxos, &HashSet::new());
        if result.is_ok() != expected {
            failures.push(format!("{name}: expected {expected}, got {result:?}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn chronicle_opcode_node_cases() {
    let mut cases: Vec<(&str, Vec<u8>, bool)> = vec![
        ("version-raw", vec![OP_VER, 4, 2, 0, 0, 0, OP_EQUAL], true),
        (
            "verif-equal",
            vec![4, 2, 0, 0, 0, OP_VERIF, OP_1, OP_ELSE, OP_0, OP_ENDIF],
            true,
        ),
        (
            "verif-less-is-not-equal",
            vec![4, 1, 0, 0, 0, OP_VERIF, OP_0, OP_ELSE, OP_1, OP_ENDIF],
            true,
        ),
        (
            "verif-short-is-not-equal",
            vec![OP_2, OP_VERNOTIF, OP_1, OP_ELSE, OP_0, OP_ENDIF],
            true,
        ),
        (
            "left-shift-negative",
            vec![OP_1NEGATE, OP_1, OP_LSHIFTNUM, 1, 0x82, OP_EQUAL],
            true,
        ),
        (
            "right-shift-negative",
            vec![1, 0x83, OP_1, OP_RSHIFTNUM, OP_1NEGATE, OP_EQUAL],
            true,
        ),
        (
            "left-shift-grow",
            vec![OP_1, OP_8, OP_LSHIFTNUM, 2, 0, 1, OP_EQUAL],
            true,
        ),
        (
            "right-shift-huge",
            vec![OP_1, 5, 0, 0, 0, 0, 1, OP_RSHIFTNUM, OP_0, OP_EQUAL],
            false,
        ),
        (
            "left-shift-huge-zero",
            vec![OP_0, 5, 0, 0, 0, 0, 1, OP_LSHIFTNUM, OP_0, OP_EQUAL],
            false,
        ),
        (
            "left-shift-huge-nonzero",
            vec![OP_1, 5, 0, 0, 0, 0, 1, OP_LSHIFTNUM],
            false,
        ),
        (
            "negative-shift",
            vec![OP_1, OP_1NEGATE, OP_RSHIFTNUM],
            false,
        ),
        (
            "substr-end-zero",
            vec![1, 42, OP_1, OP_0, OP_SUBSTR, OP_DROP, OP_1],
            false,
        ),
        (
            "substr-valid-zero",
            vec![1, 42, OP_0, OP_0, OP_SUBSTR, OP_0, OP_EQUAL],
            true,
        ),
        ("substr-empty", vec![OP_0, OP_0, OP_0, OP_SUBSTR], false),
        (
            "left-empty",
            vec![OP_0, OP_0, OP_LEFT, OP_0, OP_EQUAL],
            true,
        ),
        (
            "right-empty",
            vec![OP_0, OP_0, OP_RIGHT, OP_0, OP_EQUAL],
            true,
        ),
    ];
    for (name, op) in [
        ("left-padded-number", OP_LEFT),
        ("right-padded-number", OP_RIGHT),
    ] {
        cases.push((
            name,
            vec![1, 42, 5, 1, 0, 0, 0, 0, op, 1, 42, OP_EQUAL],
            true,
        ));
    }
    let mut rows = vec![];
    for (name, code, expected) in cases {
        let library = eval_two_phase(
            &[],
            &code,
            &mut TxVersionChecker { tx_version: 2 },
            CONSENSUS_ONLY,
        )
        .is_ok();
        let tx = Tx {
            version: 2,
            inputs: vec![TxIn {
                prev_output: OutPoint {
                    hash: Hash256([3; 32]),
                    index: 0,
                },
                unlock_script: Script::new(),
                sequence: u32::MAX,
            }],
            outputs: vec![TxOut {
                satoshis: 1,
                lock_script: Script(vec![OP_1]),
            }],
            lock_time: 0,
        };
        rows.push(json!({"name":name,"expected":expected,"library":library,"request":{"tx":tx.as_hexstr(),"n":0,"flags":(1u32<<18)|(1<<19)|(1<<20)|(1<<21),"reportflags":true,"txo":{"lock":hex::encode(&code),"value":1}}}));
        assert_eq!(library, expected, "{name}");
    }
    if let Ok(path) = std::env::var("CHRONICLE_DIFF_OUTPUT") {
        std::fs::write(path, serde_json::to_vec_pretty(&rows).unwrap()).unwrap();
    }
}
