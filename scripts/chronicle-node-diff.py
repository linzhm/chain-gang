#!/usr/bin/env python3
"""Compare exported Rust cases with BSV v1.2.3 verifyscript in Apple Container.
Run cargo test with CHRONICLE_DIFF_OUTPUT=/absolute/cases.json, then:
python3 scripts/chronicle-node-diff.py cases.json results.json
Node name defaults to cg-chronicle-node. Credentials are isolated regtest only.
"""
import json, os, subprocess, sys
cases=json.load(open(sys.argv[1]))
cli=['container','exec',os.environ.get('BSV_CONTAINER','cg-chronicle-node'),'bitcoin-cli','-regtest','-rpcuser=cgtest','-rpcpassword=local-regtest-only']
version=subprocess.check_output(['container','exec',os.environ.get('BSV_CONTAINER','cg-chronicle-node'),'bitcoind','--version'],text=True).splitlines()[0]
results=[]
for row in cases:
    reply=json.loads(subprocess.check_output(cli+['verifyscript',json.dumps([row['request']]),'false','1000'],text=True))[0]
    result={**row,'node':reply,'matches':reply['result'] in ('ok','error') and (reply['result']=='ok')==row['library']==row['expected']}
    results.append(result)
    print(row['name'],reply['result'], 'MATCH' if result['matches'] else 'MISMATCH',reply.get('description',''))
json.dump({'node_version':version,'image':'docker.io/bitcoinsv/bitcoin-sv@sha256:d96fe4786a00e5fc10bc9e0e76a6006376f9472d4eb74d07e4c1e0d019493d33','runtime':'Apple Container','cases':results},open(sys.argv[2],'w'),indent=2)
sys.exit(0 if all(r['matches'] for r in results) else 1)
