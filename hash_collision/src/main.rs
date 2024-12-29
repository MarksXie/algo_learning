/* 每当哈希冲突发生的时候，我们就进行哈希表扩容，知道冲突消失为止.此方法简单粗暴且有效，但是效率太低，因为哈希表扩容需要进行大量的数据搬运和哈希值计算。为了提高效率，我们可以采用一下策略。
    1.改良哈希表数据结构，使得哈希表可以在出现哈希冲突的时候能正常工作。
    2.仅在必要的时候，即在哈希冲突比较严重的时候，才执行扩容操作。 */

use std::collections::HashMap;

/* 1.链式地址
        在原始哈希表中，每个桶仅能存储一个键值对。链式地址(seperate chaining)将单个元素转换为链表，将键值对作为链表节点，将所有发生冲突的键值对都存储在一个链表中。
        
        基于链式地址实现的哈希表的操作方法发生了如下变化：
            1.查询元素：输入key，经过哈希函数得到桶索引，即可访问链表头节点，然后遍历链表并对比key以查找目标键值对。
            2.添加元素：首先通过哈希函数访问链表头节点，然后将节点(键值对)添加到链表中。
            3.根据哈希函数的结果访问链表头部，接着遍历链表以查找目标节点并将其删除。
            
        链式地址存在以下局限性：
            1.占用空间增大：链表需要包含节点指针，它相比数组更加耗费内存空间。
            2.查询效率降低：因为需要线性遍历链表来查找对应元素。
            
        以下给出了链式地址哈希表的简单实现，需要注意两点
            1.使用列表(动态数组)代替链表，从而简化代码。在这种设定下，哈希表(数组)包含多个bucket，每个bucket都是一个列表。
            2.以下实现包含哈希表扩容方法。当负载因子超过2/3时，我们将哈希表扩容至原先的2倍。 */
#[derive(Debug, Clone, PartialEq)]
pub struct Pair{
    key: i32,
    value: String,
}

/* 链式地址哈希表 */
struct HashMapChaining{
    size: usize,
    capacity: usize,
    load_thres: f32,
    extend_ratio: usize,
    buckets: Vec<Vec<Pair>>,
}

#[allow(dead_code)]
impl HashMapChaining {
    /* 构造方法 */
    pub fn new() -> Self {
        Self { size: 0, capacity: 4, load_thres: 2.0/3.0, extend_ratio: 2, buckets: vec![vec![]; 4] }
    }

    /* 哈希函数 */
    fn hash_func(&self, key: i32) -> usize{
        key as usize % self.capacity as usize
    }

    /* 负载因子 */
    fn load_factor(&self) -> f32 {
        self.size as f32 / self.capacity as f32
    }

    /* 删除操作 */
    fn remove(&mut self, key: i32) -> Option<String> {
        let index = self.hash_func(key);
        let bucket = &mut self.buckets[index];

        // 遍历Bucket，从中删除键值对
        for i in 0..bucket.len() {
            if bucket[i].key == key{
                let pair = bucket.remove(i);
                self.size -= 1;
                return Some(pair.value);
            }
        }

        /* 若未找到，从中删除键值对 */
        None
    }

    /* 扩容哈希表 */
    fn extend(&mut self) {
        // 暂存原哈希表
        let buckets_tmp = std::mem::replace(&mut self.buckets, vec![]);

        // 初始化扩容后的新哈希表
        self.capacity *= self.extend_ratio;
        self.buckets = vec![Vec::new(); self.capacity as usize];
        self.size = 0;
        
        // 将键值对从原哈希表中搬运至新的哈希表
        for bucket in buckets_tmp{
            for pair in bucket{
                self.put(pair.key, pair.value);
            }
        }
    }

    /* 打印哈希表 */
    fn printhash(&self){
        for bucket in &self.buckets{
            let mut res = Vec::new();
            for pair in bucket{
                res.push(format!("{} -> {}", pair.key, pair.value));
            }
            println!("{:?}", res);
        }
    }

    /* 添加操作 */
    fn put(&mut self, key: i32, value: String) {
        // 当负载因子超过阈值时，执行扩容
        if self.load_factor() > self.load_thres{
            self.extend();
        }

        let index = self.hash_func(key);
        let bucket = &mut self.buckets[index];

        // 遍历Bucket,若遇到指定key，则更新到对应value并返回
        for pair in bucket{
            if pair.key == key{
                pair.value = value;
                return;
            }
    }
    let bucket = &mut self.buckets[index];

    // 如无该key，则将键值对添加到尾部
    let pair = Pair{key, value: value.clone()};
    bucket.push(pair);
    self.size += 1;
}

    /* 查询操作 */
    fn get(&self, key: i32) -> Option<&str>{
        let index = self.hash_func(key);
        let bucket = &self.buckets[index];

        // 遍历Bucket，若找到key，则返回对应value
        for pair in bucket{
            if pair.key == key{
                return Some(&pair.value);
            }
        }

        // 若没有找到key，则返回None
        None
    }
}

/* 2.开放寻址
        开放寻址不引入额外的数据结构，而是通过“多次探测”来处理哈希冲突，探测方式主要包含线性探测、平方探测和多次哈希等。
        1.线性探测
        线性探测采用固定步长的线性搜索来进行探测，其操作方法和普通哈希表有所不同。
            1.插入元素：通过哈希函数计算bucket索引，若发现bucket内已有元素，则从冲突位置向后线性遍历(步长通常为1),知道找到空bucket，将元素插入其中。
            2.查找元素：若发现哈希冲突，则使用相同步长向后进行线性遍历，知道找到对应的元素，返回value即可；如果遇到空bucket，说明元素不在哈希表中，返回None。
        线性检测容易产生“聚集现象”。
        我们不能在开放寻址哈希表中直接删除元素。这是因为删除元素会在数组内产生一个空桶None，而当查询元素时，线性探测到该空bucket就会返回，因此在该空bucket之下的元素都无法再被访问到，程序可能会误判这些元素不存在。
        为了解决该问题，我们可以采用懒删除(lazy deletion)机制：它不直接从哈希表中移除元素，而是利用一个常量TOMBSTONE来标记这个桶。在该机制下，
        None和TOMBSTONE都代表空桶，都可以放置键值对。但是不同的是，线性检测到TOMBSTONE时应该继续遍历，因为在其下还可能存在键值对。
        然而，懒删除可能会加速哈希表的性能退化。这是因为在每一次删除操作时都会产生一个删除标记，随着TOMBSTONE的增加，搜索时间也会增加，因为线性检测可能需要跳过多个TOMBSTONE才能找到目标元素。
        为此，可以考虑在线性探测中记录首个TOMBSTONE的索引，并将搜索到的目标元素与该TOMBSTONE交换位置。这样做的好处是每当查询或者添加元素的时候，元素会被移动至距离理想位置(探测起始点)更近的桶，从而优化查询效率。
         */

/* 开放寻址哈希表 */
#[allow(dead_code)]
struct HashMAPOpenAdressing{
    size: usize,    // 键值对数量
    capacity: usize, // 哈希表容量
    load_thres: f64, // 触发扩容的负载因子阈值
    extend_ratio: usize, // 扩容倍数
    buckets: Vec<Option<Pair>>, // 桶数组
    tombstone: Option<Pair>, // 删除标记
}

#[allow(dead_code)]
impl HashMAPOpenAdressing {
    /* 构造方法 */
    fn new() -> Self {
        Self { size: 0, 
               capacity: 4,
               load_thres: 2.0/3.0, 
               extend_ratio: 2, 
               buckets: vec![None;4], 
               tombstone: Some(Pair { key: -1, value: "-1".to_string() }) }
    }

    /* 哈希函数 */
    fn hash_func(&self, key: i32) -> usize{
        (key % self.capacity as i32) as usize
    }

    /* 负载因子 */
    fn load_factor(&self) -> f64{
        self.size as f64 / self.capacity as f64
    }

    /* 搜索key对应的桶索引 */
    fn find_bucket(&mut self, key: i32) -> usize {
        let mut index = self.hash_func(key);
        let mut first_tombstone = -1;
        // 线性检测，当遇到空桶时跳出
        while self.buckets[index].is_some() {
            // 若遇到key，返回对应的桶索引
            if self.buckets[index].as_ref().unwrap().key == key{
                // 若之前遇到了删除标记，则将键值对移动至该索引
                if first_tombstone != -1{
                    self.buckets[first_tombstone as usize] = self.buckets[index].take();
                    self.buckets[index] = self.tombstone.clone();
                    return first_tombstone as usize;    // 返回移动的桶索引
                }
                return index;   // 返回桶索引
            }
            // 记录遇到的首个删除标记
            if first_tombstone == -1 && self.buckets[index] == self.tombstone{
                first_tombstone = index as i32;
            }
            // 计算桶索引，越过尾部则返回头部
            index = (index+1) % self.capacity;
        }
        // 若key不存在，则返回添加点的索引
        if first_tombstone == -1{
            index
        }
        else {
            first_tombstone as usize
        }
    }

    /* 查询操作 */
    fn get(&mut self, key: i32) -> Option<&str> {
        // 搜索key对应的桶索引
        let index = self.find_bucket(key);
        // 若找到键值对，则返回对应的value
        if self.buckets[index].is_some() && self.buckets[index] != self.tombstone {
            return self.buckets[index].as_ref().map(|pair| &pair.value as &str);
        }
        // 若键值对不存在，则返回None
        None
    }

    /* 添加操作 */
    fn put(&mut self, key: i32, value: String) {
        // 当负载因子超过阈值时，执行扩容
        if self.load_factor() > self.load_thres{
            self.extend();
        }
        // 搜索key对应的桶索引
        let index = self.find_bucket(key);
        // 若找到键值对，则覆盖value并返回
        if self.buckets[index].is_some() && self.buckets[index] != self.tombstone {
            self.buckets[index].as_mut().unwrap().value = value;
            return;
        }
        // 若键值对不存在，则添加该键值对
        self.buckets[index] = Some(Pair { key: key, value: value });
        self.size += 1;
    }

    /* 删除操作 */
    fn remove(&mut self, key: i32) {
        // 搜索key对应的桶索引
        let index = self.find_bucket(key);
        // 若找到该键值对，则用删除标记覆盖它
        if self.buckets[index].is_some() && self.buckets[index] != self.tombstone{
            self.buckets[index] = self.tombstone.clone();
            self.size -= 1;
        }
    }

    /* 扩容哈希表 */
    fn extend(&mut self){
        // 暂存哈希表
        let buckets_map = self.buckets.clone();
        // 初始化扩容后的哈希表
        self.capacity *= self.extend_ratio;
        self.buckets = vec![None;self.capacity];
        self.size = 0;

        // 将键值对从原哈希表搬运至新哈希表
        for pair in buckets_map{
            if pair.is_none() || pair == self.tombstone{
                continue;
            }
            let pair = pair.unwrap();

            self.put(pair.key, pair.value);
        }
    }

    /* 打印哈希表 */
    fn print(&self){
        for pair in &self.buckets{
            if pair.is_none(){
                println!("NULL");
            }
            else if pair == &self.tombstone {
                println!("TOMBSTONE");
            }
            else {
                let pair = pair.as_ref().unwrap();
            println!("{} -> {}", pair.key, pair.value);           
            }
        }
    }
}

/* 平方探测：
    平方探测与线性检测类似，都是开放寻址的常见策略之一。当冲突发生时，平方探测不是简单地跳过一个固定的步数，而是跳过“探测次数的平方”的步数。
    平方探测主要具有以下优势：
        1.平方探测会跳过探测次数平方的距离，试图缓解线性检测的聚集效应。
        2.平方探测会跳过更大的距离来寻找空位置，有助于使数据分布更均匀。
    然而，平方探测并不是完美的。
        1.仍然会存在聚集现象，即某些位置比其他位置更容易被占用。
        2.由于平方的增长，平方探测可能不会探测整个哈希表，这意味着即使哈希表中有空桶，平方探测也可能无法访问它。 */

/* 多次哈希：
    顾名思义，多次哈希方法就是通过多次哈希函数进行探测。
        1.插入元素：若哈希函数1出现冲突，则尝试哈希函数2，以此类推，直到找到空位后插入元素。
        2.查找元素：在相同的哈希函数顺序下进行查找，直到找到目标元素时返回；若遇到空位或已尝试所有哈希函数，说明哈希表中不存在这个元素，返回None。
    与线性探测相比，多次哈希方法不易产生聚集，但多个哈希函数会带来额外的计算量。 */

fn main() {
    let mut hash_map: HashMap<i32 , String> = HashMap::new();
    hash_map.insert(1, "wudi".to_string());
    println!("{:?}", hash_map);
}