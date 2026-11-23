import json, hashlib, struct, subprocess, os, pathlib
W='/Users/linzheming/Codex/2026-11-19/referenced-chatgpt-conversation-this-is-an/work'
p=pathlib.Path(W+'/chain-admission/separator-final-checkout/tests/receipts/node-chain/chronicle-separator-exit-final/receipts.json')
d=json.loads(p.read_text()); records=d['records']; lookup={x['name']:x for x in records}
class Reader:
 def __init__(self,h): self.b=bytes.fromhex(h); self.i=0
 def take(self,n):
  r=self.b[self.i:self.i+n]; self.i+=n
  if len(r)!=n: raise ValueError('short transaction')
  return r
 def integer(self,n): return int.from_bytes(self.take(n),'little')
 def varint(self):
  k=self.integer(1)
  return self.integer({253:2,254:4,255:8}[k]) if k>=253 else k
 def varbytes(self): return self.take(self.varint())
def parse(h):
 r=Reader(h); r.take(4); inputs=[]
 for _ in range(r.varint()):
  txid=r.take(32)[::-1].hex(); ix=r.integer(4); r.varbytes(); r.take(4); inputs.append((txid,ix))
 outputs=[]
 for _ in range(r.varint()): outputs.append((r.integer(8),r.varbytes().hex()))
 r.take(4)
 if r.i!=len(r.b): raise ValueError('trailing bytes')
 return inputs,outputs
def txid(h):return hashlib.sha256(hashlib.sha256(bytes.fromhex(h)).digest()).digest()[::-1].hex()
results=[]
for row in records:
 if 'mempool' not in row: continue
 parents={txid(x):parse(x)[1] for x in row['parents']}; ins,_=parse(row['tx']); prev=[]
 for txid0,ix in ins:
  amount,script=parents[txid0][ix]
  prev.append({'txid':txid0,'vout':ix,'script':script,'satoshis':amount,'height':600})
 companion=lookup.get(row['name']+'-mined') or lookup.get(row['name']+'-block')
 if not companion: raise ValueError('missing block record '+row['name'])
 height=companion['height']
 cmd=[W+'/lean-crosscheck/lean/.lake/build/bin/bsvs','eval','--commit','1c051eaef1077cc95e5d1618686d5efacc4aa097','--tx',row['tx'],'--prevouts',json.dumps(prev,separators=(',',':')),'--config','sv-regtest','--genesis-height','1','--chronicle-height','200','--spend-height',str(height),'--consensus','true']
 proc=subprocess.run(cmd,capture_output=True,text=True,timeout=60)
 if proc.returncode: raise ValueError((row['name'],proc.stderr))
 spec=json.loads(proc.stdout); node=row['mempool']!='rejected'; valid=spec['verdict']=='valid' and spec['gate']['verdict']=='valid'
 results.append({'name':row['name'],'node_mempool':row['mempool'],'node_error':row['rpc_error'],'node_block':companion,'chain_gang':row['library_policy'],'tx':row['tx'],'parents':row['parents'],'prevouts':prev,'prevout_height_assumption':'600, era-equivalent post-Chronicle; actual funding height unavailable','spend_height':height,'lean':spec,'lean_matches_node':valid==node,'lean_matches_chain_gang':valid==row['library_policy']})
 print(len(results),row['name'],spec['verdict'],spec.get('code'),node,flush=True)
out=pathlib.Path('/tmp/bsv-lean-250-replay.json');out.write_text(json.dumps({'metadata':d['metadata'],'lean_commit':'1c051eaef1077cc95e5d1618686d5efacc4aa097','execution':'direct bsvs executable with explicit commit stamp; wrapper guard not invoked','results':results},indent=2)+'\n')
print('RESULT',len(results),'node mismatches',sum(not x['lean_matches_node'] for x in results),'SDK mismatches',sum(not x['lean_matches_chain_gang'] for x in results),'file',out)
