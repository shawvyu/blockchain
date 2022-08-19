use std::path::Path;

use sled::Db;

use crate::{Storage,error::BlockchainError,TABLE_OF_BLOCK,TIP_KEY,HEIGHT, utils::serialize};



pub struct SledDb {
    db:Db
}

impl SledDb {
    pub fn new(path:impl AsRef<Path>)->Self{
        Self { db: sled::open(path).unwrap() }
    }

    fn get_full_key(table:&str,key:&str)->String{
        format!("{}:{}",table,key)
    }
}

impl Storage for SledDb {
    fn get_block(&self, key: &str) -> Result<Option<crate::Block>, crate::error::BlockchainError> {
        let name = Self::get_full_key(TABLE_OF_BLOCK, key);
        let result=self.db.get(name)?.map(|v|v.into());
        Ok(result)
    }
    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = crate::Block>>, crate::error::BlockchainError> {
        
    }
    fn get_height(&self) -> Result<Option<usize>, crate::error::BlockchainError> {
        
    }
    fn get_tip(&self) -> Result<Option<String>, crate::error::BlockchainError> {
        let result=self.db.get(TIP_KEY)?.map(|v|);
    }
    fn update_blocks(&self, key: &str, block: &crate::Block, height: usize) {
        
    }
}