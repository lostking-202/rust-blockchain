use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::block;
use utils::coder;

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct BlockHeader{
    pub time:i64,
    pub tx_hash:String,
    pub pre_hash:String
}
#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct Block{
    pub header:BlockHeader,
    pub hash:String,
    pub data:String
}


impl Block {
    fn set_hash(&mut self){
        let header=coder::my_serialize(&(self.header));
        self.hash=coder::get_hash(&header[..])
    }

    pub fn new_block(data:String,pre_hash:String)->Block{
        let transactions=coder::my_serialize(&data);
        let tx_hash=coder::get_hash(&transactions[..]);
        let mut block=Block{
            header:BlockHeader{
                time:Utc::now().timestamp(),
                tx_hash:tx_hash,
                pre_hash:pre_hash,
            },
            data:data,
            hash: "".to_string(),
        };
        block.set_hash();
        block
    }
}