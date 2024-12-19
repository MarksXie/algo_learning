use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
struct ListNode<T>{
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
    front: Option<Rc<RefCell<ListNode<T>>>>,
    rear: Option<Rc<RefCell<ListNode<T>>>>,
    que_size: u32,
}