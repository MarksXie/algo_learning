/* 列表（List）是一个ie抽象的概念，它表示元素的有序集合，支持元素访问、修改、添加、删除和遍历等操作，无需考虑容量限制的问题。列表可以基于数组或者链表实现。
    1.链表天然可以看作是一个列表，其支持元素增删查改操作，并且可以灵活动态扩容。
    2.数组也支持元素增删查改，但由于其长度不可变，因此只能看作是一个具有长度限制的列表。
   当使用数组实现列表时，长度不可变性质会导致列表的实用性降低=。
   为解决此问题，我们可以采用动态数组来实现列表。他继承了数组的各项优点，并且可以在程序运行过程中进行动态扩容。
   实际上，许多编程语言中的标准库提供的列表都是基于动态数组实现的。 */


fn main() {
    /* 列表常用操作 */
    /* 1.列表初始化操作 */
    // 无初始值
    let _num1: Vec<i32> = Vec::new();
    // 有初始值
    let mut nums = vec![1, 2, 3, 4, 5, ];

    /* 2.访问元素 */
    // 访问元素
    let _num = nums[1];
    // 更新元素
    nums[1] = 0;


    /* 3.插入和删除元素 */
    /* 相较于数组，列表可以自由的添加和删除元素。在列表的尾部添加元素的时间复杂度为O(1),
    但插入和删除元素的效率仍和数组相同，时间复杂度为O(n). */
    // 清空列表
    nums.clear();

    // 在尾部添加元素
    nums.push(1);
    nums.push(3);
    nums.push(2);
    nums.push(5);
    nums.push(4);

    // 在中间插入元素
    nums.insert(3, 6);
    /* println!("{:?}", nums); */

    // 删除元素
    nums.remove(3);
    /* println!("{:?}", nums); */

    /* 4.遍历列表 */
    // 通过索引遍历列lint表
    let mut _count = 0;
    for i in 0..nums.len(){
        _count += nums[i];
    }
    /* println!("{}", _count); */

    // 直接遍历列表元素
    _count = 0;
    for num in &nums{
        _count += num;
    }
    //用迭代器的fold方法实现元素相加
    _count = nums.iter().fold(0, |acc, x| acc + x);
    // 用迭代器的sum方法实现元素相加
    _count = nums.iter().sum();

    /* 5.拼接两个列表 */
    let nums1 = vec![6, 7, 8, 9];
    // nums.extend(nums1);
    println!("{:?}", nums);

    /* 6.将两个列表对应元素相加 */
    let v3: Vec<_> = nums.iter().zip(nums1.iter()).map(| (a, b)| a+b).collect();
    println!("V3:{:?}", v3);

    /* 7.排序列表 */
    nums.sort();
    println!("nums_sort:{:?}", nums);
}

/* 简易列表的实现主要有三个重点：
    1.初始容量：选取一个合理的数组初始容量。
    2.数量记录：声明一个变量size，用于记录列表的当前元素数量，并随着元素插入和删除实时更新。根据此变量，
    我们可以定位列表尾部，以及判断是否需要扩容。
    3.扩容机制：在插入元素时列表已经满了，则需要进行扩容。先根据扩容倍数创建一个更大的数组，再将当前数组的所有元素一次移动至新数组。 */

/* 列表类 */
struct MyList{
    arr: Vec<i32>,    // 数组（存储列表元素）
    capacity: usize,    // 列表容量
    size: usize,    // 列表长度（当前元素数量）
    extend_ratio: usize,    // 每次列表扩容的倍数
}

#[allow(unused)]
impl MyList {
    /* 构造方法 */
    pub fn new(capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        vec.resize(capacity, 0);
        Self { arr: vec, capacity: capacity, size: 0, extend_ratio: 2, }
    }

    /* 获取列表长度（当前元素数量） */
    pub fn size(&self) -> usize {
        return self.capacity;
    }

    /* 访问元素 */
    pub fn get(&self, index: usize) -> i32{
        // 如果索引越界，则抛出异常，下同
        if index >= self.size{
            panic!("索引越界")
        };
        return self.arr[index];
    }

    /* 更新元素 */
    pub fn set(&mut self, index: usize, num: i32) {
        // 如果索引越界，则抛出异常
        if index >= self.size{
            panic!("索引越界")
        }
    self.arr[index] = num;
    }

    /* 在尾部添加元素 */
    pub fn add(&mut self, num: i32) {
        // 元素数量超出容量时，触发扩容机制
        if self.size == self.capacity{
            self.extend_capacity();
        }
        self.arr[self.size] = num;
        // 更新元素数量
        self.size += 1;
    }

    /* 在中间添加元素 */
    pub fn insert(&mut self, index: usize, num: i32) {
        if index >= self.size{
            panic!("索引越界")
        }
        // 元素数量超出容量时，触发扩容机制
        if self.size == self.capacity{
            self.extend_capacity();
        }
        // 将索引index及之后的元素都向后移动一位
        for j in (index..self.size).rev(){
            self.arr[j+1] = self.arr[j];
        }
        self.arr[index] = num;
        // 更新元素数量
        self.size += 1;
    }

    /* 删除元素 */
    pub fn remove(&mut self, index: usize) -> i32 {
        if index >= self.size{
            panic!("索引越界")
        }
        let num = self.arr[index];
        // 将索引index及之后的元素都向前移动一位
        for j in (index..self.size-1){
            self.arr[j] = self.arr[j+1];
        }
        // 更新元素数量
        self.size -= 1;
        //返回被删除的元素
        return num;
    }

    /* 列表扩容 */
    pub fn extend_capacity(&mut self) {
        // 新建一个长度为extend_ratio倍的新数组，并将原数组复制到新数组
        let new_capacity = self.capacity * self.extend_ratio;
        self.arr.resize(new_capacity, 0);
        // 更新列表容量
        self.capacity = new_capacity;
    }

    /* 将列表转换成数组 */
    pub fn to_array(&mut self) -> Vec<i32> {
        // 仅转换有效长度范围内的列表元素
        let mut arr = Vec::new();
        for i in 0..self.size{
            arr.push(self.arr[i]);
        }
        arr
    }
}
