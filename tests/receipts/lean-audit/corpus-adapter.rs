use chain_gang::{messages::{Tx, TxOut}, script::Script, util::Serializable};
use linked_hash_map::LinkedHashMap;
use serde_json::{Value,json};
use std::{collections::HashSet,io::{self,BufRead,Cursor}};
fn run(v: &Value) -> Result<Value,String> {
 let id = v["id"].as_str().ok_or("id")?;
 let p = &v["params"];
 if p.get("flags").is_some() || p["era"].as_str()!=Some("post-chronicle") || p["consensus"].as_bool()!=Some(true) {return Ok(json!({"id":id,"skip":"non-default flags, era, or consensus"}));}
 let raw=hex::decode(v["tx"].as_str().ok_or("tx")?).map_err(|e|e.to_string())?;
 let tx=Tx::read(&mut Cursor::new(raw)).map_err(|e|e.to_string())?;
 let prev=v["prevouts"].as_array().ok_or("prevouts")?;
 if prev.len()!=tx.inputs.len(){return Err("prevout count".into());}
 let mut utxos=LinkedHashMap::new();
 for (i,p) in prev.iter().enumerate() {
  let expected=hex::encode(tx.inputs[i].prev_output.hash.0.iter().rev().copied().collect::<Vec<_>>());
  if p["txid"].as_str()!=Some(expected.as_str()) || p["vout"].as_u64()!=Some(tx.inputs[i].prev_output.index as u64) {return Err("prevout mismatch".into());}
  let lock=hex::decode(p["script"].as_str().ok_or("script")?).map_err(|e|e.to_string())?;
  let sats=p["satoshis"].as_i64().ok_or("satoshis")?;
  utxos.insert(tx.inputs[i].prev_output.clone(),TxOut{satoshis:sats,lock_script:Script(lock)});
 }
 let result=tx.validate_consensus(true,true,&utxos,&HashSet::new());
 Ok(json!({"id":id,"verdict":if result.is_ok(){"valid"}else{"invalid"},"error":result.err().map(|e|e.to_string())}))
}
fn main(){for line in io::stdin().lock().lines(){let l=line.unwrap(); let v:Value=serde_json::from_str(&l).unwrap();let o=run(&v).unwrap_or_else(|e|json!({"id":v["id"],"adapter_error":e}));println!("{}",o);}}
