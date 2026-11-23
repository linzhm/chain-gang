use chain_gang::messages::{OutPoint, Tx, TxIn, TxOut};
use chain_gang::network::Network;
use chain_gang::script::Script;
use chain_gang::util::Hash256;
use linked_hash_map::LinkedHashMap;
use std::collections::HashSet;

#[test]
fn transaction_validation_rejects_duplicate_outpoints() {
    let first = OutPoint {
        hash: Hash256([0x11; 32]),
        index: 0,
    };
    let second = OutPoint {
        hash: first.hash,
        index: 1,
    };
    let input = |prev_output| TxIn {
        prev_output,
        unlock_script: Script(vec![]),
        sequence: u32::MAX,
    };
    let output = TxOut {
        satoshis: 1000,
        lock_script: Script(vec![0x51]),
    };
    let mut utxos = LinkedHashMap::new();
    utxos.insert(first.clone(), output.clone());
    utxos.insert(second.clone(), output);
    let mut tx = Tx {
        version: 2,
        inputs: vec![input(first.clone()), input(second.clone())],
        outputs: vec![TxOut {
            satoshis: 1500,
            lock_script: Script(vec![0x51]),
        }],
        lock_time: 0,
    };
    let pregenesis = HashSet::new();

    // Different indexes of the same transaction are distinct spendable outpoints.
    assert!(tx.validate(true, true, &utxos, &pregenesis).is_ok());
    assert!(tx
        .validate_consensus(true, true, &utxos, &pregenesis)
        .is_ok());

    let assert_rejected = |tx: &Tx| {
        assert!(tx.validate(true, true, &utxos, &pregenesis).is_err());
        assert!(tx
            .validate_consensus(true, true, &utxos, &pregenesis)
            .is_err());
        assert!(tx
            .validate_at_height(
                true,
                true,
                &utxos,
                &pregenesis,
                967_896,
                Network::BSV_Mainnet
            )
            .is_err());
        assert!(tx
            .validate_at_height_consensus(
                true,
                true,
                &utxos,
                &pregenesis,
                967_896,
                Network::BSV_Mainnet
            )
            .is_err());
    };

    // One 1,000 satoshi output cannot fund 1,500 satoshis by being repeated.
    tx.inputs = vec![input(first.clone()), input(first.clone())];
    assert_rejected(&tx);

    // Repeated inputs must also be detected when they are non-adjacent.
    tx.inputs = vec![input(first.clone()), input(second), input(first)];
    assert_rejected(&tx);
}
