use serde::Serialize;
use crypto::{sha3::Sha3,digest::Digest};
use crate::error::BlockchainError;



pub fn serialize<T>(data:&T)->Result<Vec<u8>,BlockchainError>
where
    T:Serialize + ?Sized
{
    Ok(bincode::serialize(data)?)
}

pub fn hash_to_str(data:&[u8])->String{
    let mut hasher=Sha3::sha3_256();
    hasher.input(data);
    hasher.result_str()
}