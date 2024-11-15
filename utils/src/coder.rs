use bincode;
use serde::{Serialize,Deserialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn my_serialize<T:?Sized>(value:&T)->Vec<u8> where T:Serialize{
    bincode::serialize(value).unwrap()
}

pub fn my_deserialize<'a,T>(bytes:&'a[u8])->T where T:Deserialize{
    bincode::deserialize(bytes).unwrap()
}

pub fn get_hash(value:&[u8])->String{
    let mut hasher=Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}