use crate::block;
use crate::block::Block;

pub struct BlockChain{
    pub blocks:Vec<block::Block>,
}

impl BlockChain {
    pub fn add_block(&mut self,data:String){
        let pre_block=&self.blocks[self.blocks.len()-1];
        let new_block=block::Block::new_block(data,pre_block.hash.clone());
        self.blocks.push(new_block)
    }

    fn new_genesis_block()->Block{
        Block::new_block("This is genesis block".to_string(),String::from(""))
    }

    pub fn new_blockChain()->BlockChain{
        BlockChain{
            blocks:vec![BlockChain::new_genesis_block()]
        }
    }
}