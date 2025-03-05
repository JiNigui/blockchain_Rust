# 概念学习
## CSDN博客
1. 区块链详解：https://blog.csdn.net/m0_68949064/article/details/138346531?ops_request_misc=%257B%2522request%255Fid%2522%253A%25228b0fd54f653bde8f588430419e571c7f%2522%252C%2522scm%2522%253A%252220140713.130102334..%2522%257D&request_id=8b0fd54f653bde8f588430419e571c7f&biz_id=0&utm_medium=distribute.pc_search_result.none-task-blog-2~all~top_positive~default-1-138346531-null-null.142^v101^pc_search_result_base4&utm_term=%E5%8C%BA%E5%9D%97%E9%93%BE&spm=1018.2226.3001.4187
2. 区块链探秘：从基础到深度，全面解读区块链技术与应用：https://blog.csdn.net/qq_51447436/article/details/134344549?ops_request_misc=%257B%2522request%255Fid%2522%253A%25228b0fd54f653bde8f588430419e571c7f%2522%252C%2522scm%2522%253A%252220140713.130102334..%2522%257D&request_id=8b0fd54f653bde8f588430419e571c7f&biz_id=0&utm_medium=distribute.pc_search_result.none-task-blog-2~all~top_positive~default-2-134344549-null-null.142^v101^pc_search_result_base4&utm_term=%E5%8C%BA%E5%9D%97%E9%93%BE&spm=1018.2226.3001.4187
## 哔站视频讲解
1. 【不看后悔系列】6分钟，用人话讲清楚区块链！：https://www.bilibili.com/video/BV1J7411Z7T9?t=311.2
2. 翻译翻译，什么叫区块链？：https://www.bilibili.com/video/BV113411w7ic?t=479.1
# 需求设计：
## 产品方案
- 目标用户：区块链初学开发者、学习者。
- 核心功能：
创世区块生成与输出。
新区块挖矿（PoW 机制）。
交易数据存储与哈希计算。
- 扩展功能（可选）：
密钥对生成与交易签名。
区块链数据持久化（文件或数据库）。
## 技术方案
- 技术选型：
编程语言：Rust（版本2021+）。
核心库：sha2（哈希计算）、serde（序列化）、secp256k1（加密）。
- 数据结构与算法应用：
区块结构：使用结构体存储索引、时间戳、前驱哈希、交易列表、Nonce。
PoW 算法：通过调整哈希前缀零的数量控制挖矿难度。
交易验证：使用哈希树（Merkle Tree）优化交易验证效率。
# 项目实施
## 参考资料学习
1. 区块链学习路线：https://zjubca.github.io/roadmap/ （粗略观看）
2. Rust编程小项目：编写简单的区块链：https://www.bilibili.com/video/BV145411t7qp?t=1.2；GitHub网址：https://github.com/anonymousGiga/blockchain
3. 【区块链开发】从零开始使用 Rust 构建比特币网络（含源码+机翻字幕）：https://www.bilibili.com/video/BV1yTxsevEmA?t=0.8
4. Github优秀作品：https://github.com/yunwei37/blockchain-rust
## 项目简介
1. main：实现“挖矿”和打印数据内容（区块链运行入口）
2. core：block，blockchain的建立，PoW算法的实现
3. utils：负责序列化和反序列化，哈希计算等功能

`只实现了区块链中非常简易的功能`

4. 项目运行效果：
在~main下，使用`cargo run`指令
```Hello to DW BlockChain!
bits: 553713663
unshifted_expt: 33
calcute, mant: 65535, expt: 240
target: 115790322390251417039241401711187164934754157181743688420499462401711837020160
len ========= 32
hash_int === 85951930509931442762620370346544256109975297332247696326649973310701373134835
Calcute, hash:  85951930509931442762620370346544256109975297332247696326649973310701373134835
Start mining ...
bits: 553713663
unshifted_expt: 33
calcute, mant: 65535, expt: 240
target: 115790322390251417039241401711187164934754157181743688420499462401711837020160
len ========= 32
hash_int === 86861114943111850332447496732416188460363576368283965916781871879146590635769
Calcute, hash:  86861114943111850332447496732416188460363576368283965916781871879146590635769
Produce a block!
Start mining ...
bits: 553713663
unshifted_expt: 33
calcute, mant: 65535, expt: 240
target: 115790322390251417039241401711187164934754157181743688420499462401711837020160
len ========= 32
hash_int === 38702988537540664694571121959775882067695034424153388383105630686029018288959
Calcute, hash:  38702988537540664694571121959775882067695034424153388383105630686029018288959
Produce a block!
++++++++++++++++++++++++++++++++++++++++++++++++++
Block {
    header: BlockHeader {
        time: 1741177934,
        tx_hash: "f\u{8e}\\·)Jã9Äh½\u{1e}h48¯\u{8d}¢ \u{82}*|ï*2=3Z·\u{8}\u{83}\u{91}",
        pre_hash: "",
        bits: 553713663,
        nonce: 0,
    },
    hash: "¾\u{7}\u{11}\u{95}\u{1b}ä\u{80}\u{12}\u{13}k\u{c}mæ®û3ôè<z\u{90}¦2üü/\n\0\u{7f}²\u{97}ó",
    data: "Genesis Block",
}

++++++++++++++++++++++++++++++++++++++++++++++++++
Block {
    header: BlockHeader {
        time: 1741177944,
        tx_hash: "eK2\u{f}ï\\þ*u\u{16}\u{89}/=mÈ\u{1}×\u{83}\u{f}ãDÃA\\í,j\u{f}J\\³Â",
        pre_hash: "¾\u{7}\u{11}\u{95}\u{1b}ä\u{80}\u{12}\u{13}k\u{c}mæ®û3ôè<z\u{90}¦2üü/\n\0\u{7f}²\u{97}ó",
        bits: 553713663,
        nonce: 0,
    },
    hash: "À\t¦\u{1a}\u{6}\u{8c}\u{1f}ï<\u{16}á¶|Å1õÊS»æ³L\u{82}\u{1f}©)3\u{8d}\u{81}/\u{6}ù",
    data: "TJU => NKU: 6 BTC",
}

++++++++++++++++++++++++++++++++++++++++++++++++++
Block {
    header: BlockHeader {
        time: 1741177954,
        tx_hash: "Ë}A0\u{9d}\u{9c}\u{3}É\\y2ucVq§¹Æ'ßÿ<s\u{96}Ã\u{9c}¤\u{9b}\u{ad}\u{80}º^",
        pre_hash: "À\t¦\u{1a}\u{6}\u{8c}\u{1f}ï<\u{16}á¶|Å1õÊS»æ³L\u{82}\u{1f}©)3\u{8d}\u{81}/\u{6}ù",
        bits: 553713663,
        nonce: 0,
    },
    hash: "U\u{91}\u{1d}\u{7f}U\u{95}\u{14}3Ïyax Ï¤cl\u{94}>é\\aÎ\u{95}kIP\u{1e}yÓÓ?",
    data: "TJU => NKU: 8 BTC",
}