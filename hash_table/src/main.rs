/* 哈希表，又称“散列表”，它通过建立键key和值value之间的映射，实现高效的元素查找。具体而言，我们向哈希表中输入一个键key，则可以在O(1)事件内获取到对应的值value */

use std::{collections::HashMap, hash::Hash};

fn main() {
    /* 初始化哈希表 */
    let mut map: HashMap<i32, String> = HashMap::new();

    /* 添加操作 */
    // 在哈希表中添加键值对(key,value)
    map.insert(12836, "小哈".to_string());
    map.insert(15937, "小锣".to_string());
    map.insert(16750, "小算".to_string());
    map.insert(13276, "小法".to_string());
    map.insert(10583, "小鸭".to_string());

    /* 查询操作 */
    // 向哈希表中输入键key，得到值value
    let name = map.get(&15937).unwrap();
    println!("{name}");

    /* 删除操作 */
    // 在哈希表中删除键值对(key, value)
    let remove_value = map.remove(&10583).unwrap();
    println!("{}", remove_value);

    /* 遍历哈希表 */
    // 遍历键值对 key -> value
    for (key, value) in &map{
        println!("{}: {}", key, value);
    }

    /* 单独遍历key */
    for key in map.keys(){
        println!("{key}");
    }

    /* 单独遍历value */
    for value in map.values(){
        println!("{value}");
    }
}

/* 键值对 */
#[derive(Debug, Clone, PartialEq)]
pub struct Pair{
    pub key: i32,
    pub value: String,
}

/* 基于数组实现的哈希表 */
pub struct ArrayHashMap{
    buckets: Vec<Option<Pair>>,
}

impl ArrayHashMap {
    pub fn new() -> ArrayHashMap {
        // 初始化数组，包含100个桶
        Self{
            buckets: vec![None; 100],
        }
    }

    /* 哈希函数 */
    fn hash_func(&self, key: i32) -> usize{
        key as usize % 100
    }

    /* 查询操作 */
    pub fn get(&self, key: i32) -> Option<&String> {
        let index = self.hash_func(key);
        self.buckets[index].as_ref().map(|pair| &pair.value)
    }

    /* 添加操作 */
    pub fn put(&mut self, key: i32, value: &str) {
        let index = self.hash_func(key);
        self.buckets[index] = Some(Pair { key, value: value.to_string() })
    }

    /* 删除操作 */
    pub fn remove(&mut self, key: i32) {
        let index = self.hash_func(key);
        // 置为None，代表删除
        self.buckets[index] = None;
    }

    /* 获取所有键值对 */
    pub fn entry_set(&self) -> Vec<&Pair> {
        self.buckets
            .iter()
            .filter_map(|pair| pair.as_ref())
            .collect()
    }

    /* 获取所有键 */
    pub fn key_set(&self) -> Vec<&String> {
        self.buckets
            .iter()
            .filter_map(|pair| pair.as_ref())
            .map(|pair| &pair.value)
            .collect()
    }

    /* 打印哈希表 */
    pub fn println(&self) {
       for pair in self.entry_set(){
            println!("{} -> {}", pair.key, pair.value);
       }
    }
}

/* 从本质上来看，哈希函数的作用是将所有的key构成的输入空间映射到数组所有索引构成的输出空间，而输入空间往往大于输出空间，
    因此，理论上存在“多个输入对应相同输出”的情况。
    我们将多个输入对应同一输出的情况称为哈希冲突。 */

/* 我们容易想到，哈希表的容量n越大，多个key被分配到同一个桶中的概率就越低，冲突就越少。因此，我们可以通过扩容哈希表解决哈希冲突。 */

/* 类似于数组扩容，哈希表扩容需将所有的键值对从原哈希表迁移到新哈希表，非常耗时；并且由于哈希表容量capacity改变，我们需要通过哈希函数
    来重新计算所有键值对的存储位置，这进一步增加了扩容过程的计算开销。因此，编程语言通常会预留足够大的哈希表容量空间，防止频繁扩容。 */

/* 负载因子是哈希表的一个重要概念，其定义为哈希表的元素数量除以桶数量，用以衡量哈希冲突的严重程度，也常作为哈希表扩容的触发条件。 */
