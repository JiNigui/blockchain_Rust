use crate::block;
use utils::coder;
use bigint::U256; // 大整数库

const MAX_NONCE: u32 = 0x7FFFFFFF;

pub struct ProofOfWork {
    target: U256,  // 目标难度
}

impl ProofOfWork {
    pub fn new_proof_of_work(bits: u32) -> ProofOfWork { // 创建工作量证明
        println!("bits: {}", bits);     
        let (mant, expt) = {
            let unshifted_expt = bits >> 24;
            println!("unshifted_expt: {}", unshifted_expt);
            if unshifted_expt <= 3 { // unshifted_expt小于等于3，尾数部分需要右移(3 - unshifted_expt) * 8位
                ((bits & 0xFFFFFF) >> (8 * (3-unshifted_expt as usize)), 0)
            } else {
                (bits & 0xFFFFFF, 8 * ((bits >> 24) - 3))
            }
        };

        if mant > 0x7FFFFF { // 计算目标值
            println!("default");

            ProofOfWork {
                target: Default::default(),  
            }   
        } else {
            println!("calcute, mant: {}, expt: {}", mant as u64, expt as usize);

            ProofOfWork {
                target: U256::from(mant as u64) << (expt as usize),  
            }
        }
    }

    fn prepare_data(bc: &mut block::Block, nonce: u32) -> Vec<u8> { // 准备区块数据
        bc.header.nonce = nonce;
        let data = coder::my_serialize(&(bc.header));
        data
    }

    pub fn run(&self, mut bc: &mut block::Block) { // 运行工作量证明，找到一个满足目标难度的哈希值
        let mut nonce: u32 = 0;

        println!("target: {:?}", self.target);

        while nonce <= MAX_NONCE { // 循环寻找满足目标难度的哈希值
            let data = ProofOfWork::prepare_data(&mut bc, nonce);
            let mut hash: [u8; 32] = [0; 32];
            coder::get_hash(&data[..], &mut hash); // 哈希计算
            println!("len ========= {:?}", hash.len());
            let hash_int = U256::from(hash);
            println!("hash_int === {:?}", hash_int);

            if hash_int <= self.target {
                println!("Calcute, hash:  {:?}", hash_int);
                bc.hash = hash.iter().map(|&c| c as char).collect::<String>();
                return;
            }

            nonce += 1;
        } 


    }
}