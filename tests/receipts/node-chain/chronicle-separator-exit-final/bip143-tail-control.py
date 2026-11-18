"""Additional BIP143 controls using the exact #250 build and the same node."""
from chronicle_separator_exit import (
    ALL_BIP143, K1PUB, SEP, TxIn, fund, log, push, run_case, script, sign, tx_for,
)

# Keep the skipped separator and declared length in both signatures.
# Shorten only the bytes written into one BIP143 preimage.
for tail, cursor in [
    (b"\x4c\x05\xab\x97", 2),
    (b"\x4d\xab", 1),
    (b"\x04\xab", 1),
    (b"\x4e\x05\x00\x00\x00\xab", 5),
]:
    for full_length in (True, False):
        code = b"\x00\x63" + SEP + b"\x68" + push(K1PUB) + b"\xad\x51\x6a" + tail
        parent = fund(script(b"\x61" + SEP + code))
        tx = tx_for([parent])
        signed = code if full_length else code[:len(code) - len(tail) + cursor]
        sig = sign(tx, parent, signed, ALL_BIP143, declared_length=len(code))
        tx.tx_ins = [TxIn(parent.id(), 0, script(push(sig)), 0xFFFFFFFF)]
        run_case(f"return-tail-{tail.hex()}-41-full-body-{full_length}",
                 parent, tx, full_length, "CHECKSIGVERIFY")
log({"name": "separator-exit-tail-control-finished", "result": "passed"})
