# Chronicle separator and termination rerun

The Python extension was rebuilt in a clean worktree from the confirmed GitHub PR #250 head.
The exact tested commit is `8b60b71a12a69fdcb465d3e627dbf6f43d6ed081`.
The node ran in a new Apple Container using the pinned v1.2.3 image and the repository's default profile.

The checked-in matrix ran 44 cases. Eight additional BIP143 controls used the same exact build and node.
The [supplemental control script](bip143-tail-control.py) imports the matrix module via PYTHONPATH.

[Raw RPC results, transactions, funding transactions and candidate blocks](receipts.json)

## Top-level OP_RETURN

| Case | Admission | Node reason | Block | Library |
| --- | --- | --- | --- | --- |
| `return-tail-4c05ab97-61-node-True` | Accepted | — | Mined `305544b2e8874557d517ef4db8c2c5a22916287e115e6503de42527ca986b3ec` | Accepted |
| `return-tail-4c05ab97-61-node-False` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKSIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `return-tail-4dab-61-node-True` | Accepted | — | Mined `08dcd1fd128a815b74c9747301d8ffb078c264101ccc38f7158bdef282e8b23d` | Accepted |
| `return-tail-4dab-61-node-False` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKSIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `return-tail-04ab-61-node-True` | Accepted | — | Mined `0ed769f26fb0da49d5574de9c25bf214e36284d787fccb45e5715569e6979888` | Accepted |
| `return-tail-04ab-61-node-False` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKSIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `return-tail-4e05000000ab-61-node-True` | Accepted | — | Mined `40338673aeb03aee83eb1bca19e57761807851bc1c8ceb9727b6324946ce2c83` | Accepted |
| `return-tail-4e05000000ab-61-node-False` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKSIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `return-tail-4c05ab97-41-node-True` | Accepted | — | Mined `71f17dd9e3d96ea49882fe3cc17d9fcce2c7e81c08396ee4661c32dd0fc6d672` | Accepted |
| `return-tail-4c05ab97-41-node-False` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKSIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `return-tail-4dab-41-node-True` | Accepted | — | Mined `5ba1aebde4726374a123ef6e0c4effa0cb6c457118812f17e48addfd21038523` | Accepted |
| `return-tail-4dab-41-node-False` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKSIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `return-tail-04ab-41-node-True` | Accepted | — | Mined `1974801df4a21860a3b0a002ac99be39d94df289002b96248c9bb1af20eeaec8` | Accepted |
| `return-tail-04ab-41-node-False` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKSIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `return-tail-4e05000000ab-41-node-True` | Accepted | — | Mined `1f20c8fdfe44c2ba7c6ab19bd95866e57b2e005cf0518609bd3b8d2d26747fcc` | Accepted |
| `return-tail-4e05000000ab-41-node-False` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKSIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `return-tail-4c05ab97-41-full-body-True` | Accepted | — | Mined `51b099ef3f706cc69eb1e884582a0036d68673ae1dc7d179c4e931e6ce910833` | Accepted |
| `return-tail-4c05ab97-41-full-body-False` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKSIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `return-tail-4dab-41-full-body-True` | Accepted | — | Mined `1d8f7335071a5e5a79ef6ae69394861adce77d7559dd2707108ec3da83979a54` | Accepted |
| `return-tail-4dab-41-full-body-False` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKSIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `return-tail-04ab-41-full-body-True` | Accepted | — | Mined `437dd0e7362742132497df363b760d5a227e0b3689ecaa4717df1efa4c06420d` | Accepted |
| `return-tail-04ab-41-full-body-False` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKSIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `return-tail-4e05000000ab-41-full-body-True` | Accepted | — | Mined `2942c32dd7c789e819b7b59cb8dc84200af2022a9aed42ab8c31b6e3f0030645` | Accepted |
| `return-tail-4e05000000ab-41-full-body-False` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKSIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |

## Branch-local OP_RETURN

| Case | Admission | Node reason | Block | Library |
| --- | --- | --- | --- | --- |
| `nested-return-61-truncated-False` | Accepted | — | Mined `5a7ad133acfd99260750a5b195193568caca8b63c0458a630fc11e3a773304cb` | Accepted |
| `nested-return-61-truncated-True` | Rejected | 16: mandatory-script-verify-flag-failed (Opcode missing or not understood) | Rejected `blk-bad-inputs` | Rejected |
| `nested-return-41-truncated-False` | Accepted | — | Mined `0434562b57080056694a1cf634a8ba20dc4d4f101187864edb0058d19b6fac26` | Accepted |
| `nested-return-41-truncated-True` | Rejected | 16: mandatory-script-verify-flag-failed (Opcode missing or not understood) | Rejected `blk-bad-inputs` | Rejected |

## Mixed signature matching

| Case | Admission | Node reason | Block | Library |
| --- | --- | --- | --- | --- |
| `mixed-lock-61-poison-False-changed-None` | Accepted | — | Mined `76df8ade8f82e50d1cbb024f7cd5909ad7bcf1af769064a7a5cf8a31ac25f358` | Accepted |
| `mixed-lock-61-poison-False-changed-0` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKMULTISIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `mixed-lock-61-poison-False-changed-1` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKMULTISIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `mixed-lock-61-poison-True-changed-None` | Rejected | 16: mandatory-script-verify-flag-failed (Non-canonical DER signature) | Rejected `blk-bad-inputs` | Rejected |
| `mixed-lock-61-poison-True-changed-0` | Accepted | — | Mined `0383cf6f6e5621e32eb25b09b2a2902cba0a63cb1027974c641efc1697b5257d` | Accepted |
| `mixed-lock-61-poison-True-changed-1` | Accepted | — | Mined `26e36e334324be8756b94ee182f36f5712148c0a5325f7ca735e002e1b22c5ce` | Accepted |
| `mixed-lock-41-poison-False-changed-None` | Accepted | — | Mined `7f6082aa4728b47012fc7fe1bb794fcd2738c38bf749e973919c2376a88115e0` | Accepted |
| `mixed-lock-41-poison-False-changed-0` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKMULTISIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `mixed-lock-41-poison-False-changed-1` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKMULTISIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `mixed-lock-41-poison-True-changed-None` | Rejected | 16: mandatory-script-verify-flag-failed (Non-canonical DER signature) | Rejected `blk-bad-inputs` | Rejected |
| `mixed-lock-41-poison-True-changed-0` | Accepted | — | Mined `6c59d9b09b2a637cb980ac0a964a1772491a3b1b6614f7ee2b7c7aa103e2ea80` | Accepted |
| `mixed-lock-41-poison-True-changed-1` | Accepted | — | Mined `3f96663b2934e122fe51f38a356aed01af2e805a6c44692fa288f768e2f62706` | Accepted |
| `mixed-unlock-61-poison-False-changed-None` | Accepted | — | Mined `05b6a2db58df406088f5651201a16a7f40e1ae236e9032aaf42babfb7a2511ab` | Accepted |
| `mixed-unlock-61-poison-False-changed-0` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKMULTISIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `mixed-unlock-61-poison-False-changed-1` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKMULTISIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `mixed-unlock-61-poison-True-changed-None` | Rejected | 16: mandatory-script-verify-flag-failed (Non-canonical DER signature) | Rejected `blk-bad-inputs` | Rejected |
| `mixed-unlock-61-poison-True-changed-0` | Accepted | — | Mined `7dcb8f2561f80b56d645218841aadbcae60e37fc4e0035a4ba6ef66e5472cbbe` | Accepted |
| `mixed-unlock-61-poison-True-changed-1` | Accepted | — | Mined `30d20070c5a43339d6c93f65b69d7bb3783b2d158b7dbab61466dae5a745a13f` | Accepted |
| `mixed-unlock-41-poison-False-changed-None` | Accepted | — | Mined `5aee61805aa8fd9889571c0ed6d0f70643f3d0edccf6dda649b0341c1362882c` | Accepted |
| `mixed-unlock-41-poison-False-changed-0` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKMULTISIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `mixed-unlock-41-poison-False-changed-1` | Rejected | 16: mandatory-script-verify-flag-failed (Script failed an OP_CHECKMULTISIGVERIFY operation) | Rejected `blk-bad-inputs` | Rejected |
| `mixed-unlock-41-poison-True-changed-None` | Rejected | 16: mandatory-script-verify-flag-failed (Non-canonical DER signature) | Rejected `blk-bad-inputs` | Rejected |
| `mixed-unlock-41-poison-True-changed-0` | Accepted | — | Mined `025eef5da07688cd5369bf300144d00744a69e7240c751fbcdc429ea972e8405` | Accepted |
| `mixed-unlock-41-poison-True-changed-1` | Accepted | — | Mined `7fb3ef3726eef5998d9701f6447f28eaddc8d89a8eed67dcb2176c22be8cd82d` | Accepted |
