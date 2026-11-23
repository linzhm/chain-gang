"""Regtest replay of the four Lean/chain-gang differential candidates.

Requires the pinned node-chain harness and a newly launched regtest node.
Funded inputs and signatures are test-only. Output goes to CHAIN_OUT.
"""
from harness import (
    RPCError, Script, Tx, TxIn, block_check, fund, log, mine_check,
    pools, raw, rpc, tx_for,
)


def observe(name, transaction, parents, expected):
    try:
        result = rpc("sendrawtransaction", raw(transaction))
        error = None
    except RPCError as exc:
        result = None
        error = str(exc)
    accepted = transaction.id() in pools()["regular"]
    row = {
        "name": name,
        "transaction": raw(transaction),
        "parents": [raw(parent) for parent in parents],
        "candidate_height": rpc("getblockcount") + 1,
        "mempool_accepted": accepted,
        "rpc_result": result,
        "rpc_error": error,
        "expected": expected,
    }
    log(row)
    assert accepted == expected, row
    if accepted:
        mine_check(name + "-mined", transaction)
    else:
        block_check(name + "-block", [transaction], False)


def replay_flow(name, lock_hex, argument, version, expected):
    lock = Script()
    lock.append_data(bytes.fromhex(lock_hex))
    parent = fund(lock)
    unlock = Script()
    unlock.append_data(argument)
    base = tx_for([parent], version=version)
    transaction = Tx(version, [TxIn(parent.id(), 0, unlock, 0xFFFFFFFF)], base.tx_outs, 0)
    observe(name, transaction, [parent], expected)


if __name__ == "__main__":
    if rpc("getblockcount") < 600:
        rpc("generate", 600 - rpc("getblockcount"))
    # Preserve each original script and version; the signed outpoint changes
    # because these corpus transactions use synthetic previous outputs.
    replay_flow("flow-01071-rebound", "6651670068", bytes.fromhex("0401000000"), 2, True)
    replay_flow("flow-01073-rebound", "6551670068", bytes.fromhex("51"), 2, False)
    replay_flow("flow-01074-rebound", "6551670068", bytes.fromhex("51"), 1, False)
    true_script = Script()
    true_script.append_data(bytes.fromhex("51"))
    parent = fund(true_script)
    transaction = tx_for([parent, parent], amounts=[7_500_000], version=2)
    observe("duplicate-outpoint-rebound", transaction, [parent], False)
