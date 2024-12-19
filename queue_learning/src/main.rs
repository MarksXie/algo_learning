use std::{cell::RefCell, collections::VecDeque, rc::Rc, thread::sleep_ms};

fn main() {
    /* 初始化双向队列 */
    // 在Rust中使用双向队列作为普通队列来使用
    let mut deque: VecDeque<u32> = VecDeque::new();

    /* 元素入队 */
    deque.push_back(1);
    deque.push_back(3);
    deque.push_back(2);
    deque.push_back(5);
    deque.push_back(4);

    /* 访问队首元素  */
    if let Some(front) = deque.front(){
        println!("{}", front);
    }

    /* 元素出队 */
    if let Some(pop) = deque.pop_front(){
        println!("{}", pop);
    }

    /* 获取队列的长度 */
    let size = deque.len();
    println!("{}", size);

    /* 判断队列是否为空 */
    let is_empty = deque.is_empty();
    println!("{}", is_empty);
}

// 先实现一个链表节点
#[allow(dead_code)]
pub struct ListNode<T>{
    elem: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl <T> ListNode<T> {
    pub fn new(elem: T) -> Self {
        Self{
            elem,
            next: None,
        }
    }
}

// 基于链表的实现
#[allow(dead_code)]
pub struct LinkedListQueue<T>{
    front: Option<Rc<RefCell<ListNode<T>>>>,    // 头节点
    rear: Option<Rc<RefCell<ListNode<T>>>>,     // 尾节点
    que_size: usize,                              // 队列的长度
}

// 实现特征
impl<T: Copy> LinkedListQueue<T> {
    // 构造函数
    pub fn new() -> Self{
        Self { 
            front: None, 
            rear: None, 
            que_size: 0, 
        }
    }

    // 获取队列的长度
    pub fn size(&self) -> usize {
        return self.que_size;
    }

    // 判断队列是否为空
    pub fn is_empty(&self) -> bool {
        self.que_size == 0
    }

    // 入队
    pub fn push(&mut self, elem: T) {
        let new_rear = Rc::new(RefCell::new(ListNode::new(elem)));
        match self.rear.take() {
            Some(old_rear) => {
                old_rear.borrow_mut().next = Some(new_rear);
            },
            None => {
                self.front = Some(new_rear.clone());
                self.rear = Some(new_rear);
            },
        }
        self.que_size += 1;
    }

    // 出队
    pub fn pop(&mut self) -> Option<T> {
        self.front.take().map(| old_front| {
            match old_front.borrow_mut().next.take(){
                Some(new_front) => {
                    self.front = Some(new_front);
                },
                None => {
                    self.rear.take();
                }
            }
        self.que_size -= 1;
        Rc::try_unwrap(old_front).ok().unwrap().into_inner().elem
    })
    }

    // 访问队首元素
    pub fn peek(&self) -> Option<&Rc<RefCell<ListNode<T>>>> {
        self.front.as_ref()
    }

    // 将链表转化成Array并返回
    pub fn to_array(&self, head: Option<&Rc<RefCell<ListNode<T>>>>) -> Vec<T> {
        if let Some(node) = head{
            let mut elems = self.to_array(node.borrow().next.as_ref());
            elems.insert(0, node.borrow().elem);
            return elems;
        }
        return Vec::new();
    }
}

/* 基于数组的实现 */
// 使用环形数组，让front或rear在越过数组尾部时，直接回到数组头部继续遍历。这种周期性规律可以通过“取余操作”来实现

// 基于环形数组实现的队列
#[allow(dead_code)]
struct ArrayQueue<T>{
    elems: Vec<T>,      // 用于存储队列元素的数组
    front: usize,           // 队首指针，指向队首元素
    que_size: usize,    // 队列长度
    que_capacity: usize,// 队列容量
}

#[allow(dead_code)]
impl<T: Clone + Copy> ArrayQueue<T>  {
    // 构造函数
    pub fn new(capacity: usize) -> Self {
        Self { 
            elems: Vec::new(), 
            front: 0, 
            que_size: 0, 
            que_capacity: capacity 
        }
    }

    // 获取队列长度
    pub fn size(&self) -> usize{
        self.que_size
    } 

    // 获取队列容量
    pub fn capacity(&self) -> usize {
        self.que_capacity
    }

    // 判断队列是否为空
    pub fn is_empty(&self) -> bool {
        self.que_size == 0
    }

    // 入队
    pub fn push(&mut self, elem: T) {
        if self.que_size == self.capacity(){
            println!("队列已满");
            return;
        }
        // 计算队尾指针，指向队尾索引+1
        // 通过取余操作实现rear越过数组尾部后回到头部
        let rear = (self.front + self.que_size) % self.que_capacity;
        // 将elem添加到队尾
        self.elems[rear] = elem;
        self.que_size += 1;
    }

    // 出队
    pub fn pop(&mut self) -> T{
        let elem = self.peek();
        // 将队首的指针向后移动一位，如果越过尾部，则返回数组头部
        self.front = (self.front + 1) % self.que_capacity;
        self.que_size -= 1;
        return elem;
    }

    // 访问队首元素
    pub fn peek(&self) -> T {
        if self.is_empty(){
            panic!("index out of bounds");
        }
        self.elems[self.front as usize]
    }

    // 返回数组
    fn to_array(&self) -> Vec<T> {
        let cap = self.que_capacity;
        let mut j = self.front;
        let mut arr = Vec::with_capacity(self.size());
        for i in 0..self.que_size{
            arr[i] = self.elems[(j%cap) as usize];
            j += 1;
        }
        arr
    }
}

/* 队列的典型应用
    1.淘宝订单：系统根据下单的时间顺序处理队列中的订单。
    2.各类待办事项。任何需要实现“先进先出”功能的场景，比如说打印机的任务队列、餐厅的出餐队列等，队列在这些场景中可以有效地维护处理顺序。 */