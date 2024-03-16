//! Here defined [Blockchain] and [BlockchainIter], struct that allows to interact with the
//! blockchain
use crate::blockchain::block::{Block, deserialize};
use chrono::Utc;
use sha256::digest;
use sled::Db;
use crate::consts::TICKS;

use super::block::blockhash_deserialize;

pub struct Blockchain{
    pub last_block : Block,
    pub db : Db,
}

pub struct BlockchainIter {
    pub curr_blockhash : String,
    pub db : Db,
}


impl Blockchain{

///adds block to blockchain
pub fn add_block(&mut self) -> () {
    //if new blockchain
    if self.db.iter().count() == 0{
        self.last_block = genesis();
        self.db.insert(self.last_block.blockhash(), self.last_block.serialize().as_bytes()).unwrap();
        self.db.insert("lh", self.last_block.blockhash().as_bytes()).unwrap();
        return ();
        }
    //if we have already at least two blocks
    let l_b = self.db.get("lh"); 
    match l_b{
        Ok(block_opt) => {
            match block_opt{
                None => {panic!("add_block error, no block found")}
                Some(b) => {
                    let new_b = create_block(std::str::from_utf8(&b).unwrap().to_owned()); 
                    self.last_block = new_b.clone();
                    self.db.insert(new_b.blockhash(), new_b.serialize().as_bytes()).unwrap();
                    self.db.insert("lh", new_b.blockhash().as_bytes()).unwrap();
                }
        
            }
        }
        Err(err) => {panic!("Error: {err}");}
        }
    }
    
    ///returns last blockhash loaded in db
    pub fn get_last_bh(&self) -> String{
        let res = self.db.get("lh").unwrap();
        match res{
            Some(r) => {blockhash_deserialize(r)}
            None => {
                println!("{:?}", res);
                return "0".to_owned();
            }
        }
    }

}
///creates first [Block]
pub fn genesis() -> Block {
    let time = Utc::now();

    let start = digest(time.to_rfc3339());
    let mut hash = start.clone();
    for _ in 0..TICKS {
        hash = digest(hash);
    }
    return Block {
        start_hash: start.clone(),
        last_hash: hash,
    };
}

///given last blockhash, returns new [Block]
///start_hash is the blockhash of previous block
fn create_block(l_b : String) -> Block{
    let s_h = l_b.clone();
    let mut l_h = l_b;
    for _ in 0..TICKS{
        l_h = digest(l_h); 
    }
    Block{start_hash: s_h.to_owned(), last_hash : l_h}

}


impl BlockchainIter{

    ///stores into [BlockchainIter::curr_blockhash] the blockhash of block created before the actual one
    pub fn prev(&mut self) -> (){
        //get block related to current blockhash
        let cur : Block = deserialize(self.db.get(&self.curr_blockhash).unwrap().unwrap());
        //if current start_hash = blockhash previous block   
        if self.db.get(&cur.start_hash).unwrap() == None {
            self.curr_blockhash = "0".to_owned();
        }
        else{
            let prev = deserialize(self.db.get(&cur.start_hash).unwrap().unwrap());
            self.curr_blockhash = prev.blockhash();
        }
    }

}
