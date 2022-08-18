use chrono::Utc;
use serde::{Deserialize, Serialize};

#[warn(unused_imports)]
use crate::utils::{hash_to_str, serialize};

use super::pow::ProofOfWork;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct BlockHeader {
    /**
     * 时间戳
     */
    timestamp: i64,
    /**
     * 上一个hash
     */
    prev_hash: String,
    /**
     * 计算难度
     */
    bits: usize,
    /**
     * 重复计算次数(记录满足bits难度计算的次数)
     */
    nonce: usize,
}

impl BlockHeader {
    fn new(prev_hash: &str, bits: usize) -> Self {
        Self {
            timestamp: Utc::now().timestamp(),
            prev_hash: prev_hash.into(),
            bits,
            nonce: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    header: BlockHeader,
    data: String,
    hash: String,
}

impl Block {
    pub fn new(data: &str, prev_hash: &str, bits: usize) -> Self {
        let mut block = Block {
            header: BlockHeader::new(prev_hash, bits),
            data: data.into(),
            hash: String::new(),
        };

        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }

    pub fn create_genesis_block(bits: usize) -> Self {
        Self::new("创世区块", "", bits)
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_header(&self) -> BlockHeader {
        self.header.clone()
    }

    pub fn set_hash(&mut self,hash:String) {
        self.hash=hash
    }

    pub fn set_nonce(&mut self, nonce: usize) {
        self.header.nonce = nonce;
    }
}
