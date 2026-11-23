# Lean differential audit, 2026-11-24

Lean commit: `1c051eaef1077cc95e5d1618686d5efacc4aa097`. The evaluator build completed and its self-test passed. The 52 PR #250 cases use the recorded Node v1.2.3 receipts. Lean replay used block heights from those receipts and assumed prevout height 600, an era-equivalent post-Chronicle height, because exact funding heights were not supplied. See ../node-chain/chronicle-separator-exit-final/lean-replay.json for each input and result.

The corpus comparison uses the fixed #250 chain-gang commit `8b60b71a12a69fdcb465d3e627dbf6f43d6ed081`, `Tx::validate_consensus`, and the original transaction and synthetic prevouts. Lean ran against the corpus parameters, and its transaction gate was valid for all three flow differences.

| Corpus | Compared | Matches | Differences | Skipped |
| --- | ---: | ---: | ---: | ---: |
| crypto | 84 | 84 | 0 | 0 |
| xsig-144 | 10 | 10 | 0 | 6 historical-era inputs |
| flow | 1094 | 1091 | 3 | 0 |
| seed | 10000 | 10000 | 0 | 0 |

The three `flow-verif-candidates.json` entries and `duplicate-outpoint-candidate.json` require fresh node execution. Their recorded comparisons and the node v1.2.3 source are evidence of candidate discrepancies. These receipts do not report a fresh node replay. The source corpus includes previously recorded node results, which agreed with Lean for all four corpora.

The corpus adapter is included as `corpus-adapter.rs`. Its full-transaction verdict is compared with Lean's script verdict with the gate checked separately.
