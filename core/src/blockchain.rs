use crate::block;

pub struct BlockChain {
    pub blocks: Vec<block::Block>, // 区块列表
    pub curr_bits: u32, // 当前难度
}

const INIT_BITS: u32 = 0x2100FFFF; // 初始难度

impl BlockChain {

    pub fn add_block(&mut self, data: String) { // 添加区块
        let prev_block = &self.blocks[self.blocks.len() - 1];
        let new_block = block::Block::new_block(data, prev_block.hash.clone(),self.curr_bits);
        self.blocks.push(new_block);
    }
    fn new_genesis_block() -> block::Block { // 创建创世区块
        block::Block::new_block("Genesis Block".to_string(), "".to_string(),INIT_BITS)
    }

    pub fn new_blockchain() -> BlockChain { // 创建区块链并初始化
        BlockChain {
            blocks: vec![BlockChain::new_genesis_block()],
            curr_bits: INIT_BITS,
        }
    }

}