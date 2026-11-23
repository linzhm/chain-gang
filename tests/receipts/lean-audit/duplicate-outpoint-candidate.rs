use chain_gang::messages::{OutPoint, Tx, TxIn, TxOut};
use chain_gang::script::Script;
use chain_gang::util::Hash256;
use linked_hash_map::LinkedHashMap;
use std::collections::HashSet;

fn main() {
    let outpoint = OutPoint { hash: Hash256([0x11; 32]), index: 0 };
    let input = TxIn { prev_output: outpoint.clone(), unlock_script: Script(vec![]), sequence: u32::MAX };
    let tx = Tx { version: 2, inputs: vec![input.clone(), input], outputs: vec![TxOut { satoshis: 1500, lock_script: Script(vec![0x51]) }], lock_time: 0 };
    let mut utxos = LinkedHashMap::new();
    utxos.insert(outpoint, TxOut { satoshis: 1000, lock_script: Script(vec![0x51]) });
    println!("tx={}", tx.as_hexstr());
    println!("policy={:?}", tx.validate(true, true, &utxos, &HashSet::new()));
    println!("consensus={:?}", tx.validate_consensus(true, true, &utxos, &HashSet::new()));
}
