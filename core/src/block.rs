use chrono::prelude::*;
use utils::coder; // 引入自定义模块，完成序列化操作，哈希计算，时间戳计算等功能
use serde::{Deserialize, Serialize};

use crate::pow; // 引入自定义模块，完成工作量证明功能
use std::str;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub time: i64, // 时间戳
    pub tx_hash: String, // 交易的默克尔哈希
    pub pre_hash: String, // 前一个块哈希
    pub bits: u32, // 存储目标难度
    pub nonce: u32, //存储随机数，用于工作量证明
}
#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader, // 区块头
    pub hash: String, // 区块哈希
    pub data: String // 交易数据
}

impl Block {
    // 创建区块
    pub fn new_block(data: String, pre_hash: String, bits: u32) -> Block {
        let transaction = coder::my_serialize(&data);
        let mut tx_hash: [u8; 32] = [0; 32]; // 计算交易数据的哈希，并存储在一个32字节数组中
        coder::get_hash(&transaction[..], &mut tx_hash);

        let time = Utc::now().timestamp();

        let mut block = Block { // 创建一个区块对象，并初始化其字段
            header: BlockHeader {
                time: time,
                tx_hash: tx_hash.iter().map(|&c| c as char).collect::<String>(), //交易的默克尔哈希
                pre_hash: pre_hash,
                bits: bits,
                nonce: 0,
            },
            hash: "".to_string(),
            data: data,
        };
        let my_pow = pow::ProofOfWork::new_proof_of_work(bits); // 创建一个工作量证明对象，并运行工作量证明算法
        my_pow.run(&mut block);

        block // 返回创建的新区快
    }
}