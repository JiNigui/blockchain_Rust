use core::blockchain;
use std::thread; // 引入线程库
use std::time::Duration; // 引入时间库

fn main() {
    println!("Welcome to DW BlockChain!");
    let mut chain = blockchain::BlockChain::new_blockchain(); // 初始化区块链

    println!("Start mining ..."); 
    thread::sleep(Duration::from_secs(10)); // 创建一个线程，每10秒执行一次
    chain.add_block("TJU => NKU: 6 BTC".to_string());
    println!("Produce a block!");

    println!("Start mining ...");
    thread::sleep(Duration::from_secs(10));
    chain.add_block("TJU => NKU: 8 BTC".to_string());
    println!("Produce a block!");

    for block in chain.blocks { // 遍历区块链并打印
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++");
        println!("{:#?}", block);
        println!("");
    }
}
