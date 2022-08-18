use tracing::info;

use super::Block;

const CURR_BITS:usize=8;

pub struct Blockchain {
    blocks: Vec<Block>,
    height: usize,
}

impl Blockchain {
    /**
     * 创建区块链
     */
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::create_genesis_block(CURR_BITS)],
            height: 0,
        }
    }

    /**
     * 挖矿，将区块加入链中
     */
    pub fn mine_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().unwrap();
        let block = Block::new(data, prev_block.get_hash().as_str(),CURR_BITS);
        self.blocks.push(block);
        self.height += 1;
    }

    pub fn blocks_info(&self){
        for block in self.blocks.iter(){
            info!("{:#?}",block)
        }
    }
}
