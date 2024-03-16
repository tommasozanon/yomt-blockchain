//! Here defined [Block] object, unit of blockchain
use sled::IVec;
use sha256::digest;

#[derive(Debug, Clone)]
pub struct Block {
    pub start_hash: String,
    pub last_hash: String,
}


impl Block{
    ///returns current block blockhash
    pub fn blockhash(&self) -> String{
    let dig: String = self.start_hash.clone() + &self.last_hash;
    digest(dig)
    
    }

    ///returns serialized block
    pub fn serialize(&self)-> String{
        self.start_hash.clone() + &self.last_hash
    
    }
    ///returns block from [IVec] input (usually get from sled)
    pub fn deserialize(&self,bb : IVec)->Block{
        let (a, b) = bb.split_at(64);
        let s_h: String = std::str::from_utf8(a).unwrap().to_string();
        let l_h: String = std::str::from_utf8(b).unwrap().to_string();
        Block { start_hash: s_h, last_hash: l_h }

    }
}

pub fn serialize(b : &Block)-> String{
    b.start_hash.clone() + &b.last_hash
    
}

pub fn deserialize(bb : IVec)->Block{
    let (a, b) = bb.split_at(64);
    let s_h: String = std::str::from_utf8(a).unwrap().to_string();
    let l_h: String = std::str::from_utf8(b).unwrap().to_string();
    Block { start_hash: s_h, last_hash: l_h }

}
///given a blockhash in [IVec] format, returns blockhash in String
pub fn blockhash_deserialize(bh : IVec) -> String{
    std::str::from_utf8(bh.split_at(64).0).unwrap().to_string()

}
