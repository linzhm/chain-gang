use chain_gang::{
    messages::{OutPoint, Tx, TxIn, TxOut},
    script::{eval_two_phase, op_codes::*, Script, TxVersionChecker, CONSENSUS_ONLY},
    util::Hash256,
};
use serde_json::json;

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
