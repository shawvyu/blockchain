use std::path::Path;

use sled::Db;

use crate::Storage;



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
        
    }
    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = crate::Block>>, crate::error::BlockchainError> {
        
    }
    fn get_height(&self) -> Result<Option<usize>, crate::error::BlockchainError> {
        
    }
    fn get_tip(&self) -> Result<Option<String>, crate::error::BlockchainError> {
        
    }
    fn update_blocks(&self, key: &str, block: &crate::Block, height: usize) {
        
    }
}