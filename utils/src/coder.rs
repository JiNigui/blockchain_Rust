use bincode;
use serde::{Deserialize, Serialize}; // 序列化与反序列化框架
use crypto::digest::Digest; // 引入哈希算法库
use crypto::sha3::Sha3; 
pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8> // 实现了 Serialize trait 的数据结构序列化为字节数组
where
    T: Serialize,
{
    let serialized = bincode::serialize(value).unwrap();
    serialized
}
pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T // 将字节数组反序列化为原始数据结构
where
    T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}
pub fn get_hash(value: &[u8], mut out: &mut [u8]) { // 计算哈希值
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result(&mut out);
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{my_deserialize, my_serialize};

    #[test]
    fn test_coder() {
        let point = Point { x: 1, y: 2 };
        let se = my_serialize(&point);
        let de: Point = my_deserialize(&se);
        assert_eq!(point, de);
    }
}