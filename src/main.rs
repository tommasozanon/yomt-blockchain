use yomt_blockchain::blockchain::block::{Block, serialize, deserialize, blockhash_deserialize};
use yomt_blockchain::blockchain::blockchain::{genesis,Blockchain, BlockchainIter};
use sled;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
    }
    let mut chain : Blockchain = Blockchain{last_block : genesis(), db : sled::open("epoch_1").unwrap()};
    let mut iter : BlockchainIter = BlockchainIter{curr_blockhash : chain.get_last_bh(), db : chain.db.to_owned()};
    iter.curr_blockhash = chain.get_last_bh();

    println!("n blocks:{}\n", chain.db.iter().count());
    //chain_analyzer(&mut iter);
    /*
    for i in 0..5 {
        let item = db.iter().count();
        println!("{:?}", item);
        let time = Instant::now();
        b = add_block(&b);
        db.insert(b.blockhash(), serialize(&b).as_bytes()).unwrap();
        db.insert("lh", b.blockhash().as_bytes());
        //db.insert(db.iter().count().to_string(), serialize(&b).as_bytes());
        println!("{:?},\t block:{:?}", time.elapsed(), b);
    }
    
    //println!("{:?}",(db.iter().last().unwrap().unwrap()));
    let mut iter = db.iter();
    println!("{:?}", deserialize(db.get("lh").unwrap().unwrap()));
    db.flush().unwrap();
    drop(db);
    */
}


fn chain_analyzer(iter : &mut BlockchainIter) -> (){
    let mut count = iter.db.iter().count()-1;
    
    while iter.curr_blockhash != "0".to_owned(){
        print!("{}:\t", count);
        println!("{}", &iter.curr_blockhash);
        iter.prev();
        count -= 1;
    }


}
