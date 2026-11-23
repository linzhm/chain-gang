# chain-gang cloud handoff — 2026-11-24

The Git bundle preserves the original commit IDs and 11 independent review refs. Entries titled WIP snapshot the working file contents for handoff. For signature-boundary, the snapshot intentionally preserves merge-conflict markers in src/script/checker.rs and must be resolved before testing. The staged/unstaged distinction and index conflict stages are not carried into snapshot commits.

## Restore inside a full checkout of linzhm/chain-gang

```sh
git show origin/handoff/local-review-2026-11-24:handoff/chain-gang.bundle > /tmp/chain-gang-handoff.bundle
git bundle verify /tmp/chain-gang-handoff.bundle
git fetch /tmp/chain-gang-handoff.bundle 'refs/heads/handoff/*:refs/heads/handoff/*'
git branch --list 'handoff/*'
```

If the remote tracking branch is absent, first run `git fetch origin refs/heads/handoff/local-review-2026-11-24:refs/remotes/origin/handoff/local-review-2026-11-24`. The bundle requires upstream e08a8cf and fork commits 8b60b71 and 88175dd. If needed, fetch upstream main and the fork branches fix/otda-complete-validation and fix/chronicle-context. The independent BSV Node v1.2.3 replay on upgradeable-nops remains pending.

Bundle SHA-256: e3d80e550d1e15ce4ade614c5a8a01406b71a16cfe277d8b165e6b1b83d80a65.

```text
5449b10 handoff/chronicle-opcodes-main-review WIP: preserve chronicle-opcodes-main-review for cloud handoff
20b4164 handoff/chronicle-opcodes-pr250-review WIP: preserve chronicle-opcodes-pr250-review for cloud handoff
0e508d9 handoff/duplicate-input WIP: preserve duplicate-input for cloud handoff
98766ef handoff/locktime Accept the full uint32 locktime range
53314d7 handoff/multisig-cleanup Match node multisig signature push cleanup
3778a1e handoff/otda-foundation WIP: preserve otda-foundation for cloud handoff
cd3c16c handoff/pr250-pr251-integration WIP: preserve pr250-pr251-integration for cloud handoff
9919b7f handoff/pr251-review WIP: preserve pr251-review for cloud handoff
096198b handoff/signature-boundary WIP: preserve signature-boundary for cloud handoff
9bbebec handoff/truncated-push fix: match node handling of truncated script pushes
194ea88 handoff/upgradeable-nops Export Chronicle NOP activation cases for node replay
```
