/* 内存空间是所有程序公共的资源，在一个复杂的系统环境下，空闲的内存空间可能散落在各处。我们知道，存储的数组的内存空间要是连续的，而
    当数组非常大的时候,内存可能无法提供这么大的连续的内存空间。这个时候，链表的灵活性的优势就体现出来了。
    
    链表是一种线性的数据结构，其中每个元素都是一个节点对象，各个节点通过“引用”相连接。引用记录了下一个节点的内存地址，
    通过它可以从当前节点访问到下一个节点。

    链表的设计使得各个节点可以分散存储在内存空间各处，他们的内存地址不需要相连。
*/

use std::rc::Rc;
use std::cell::RefCell;

/* 链表节点类 */
#[derive(PartialEq)]
struct ListNode<T>{
    val: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

/* 插入节点：
    插入节点只需要改变两个节点之间的引用即可，事件复杂度为O(1). 相比之下，在数组中插入元素大的时间复杂度为O(n),在大量数据下的效率较低*/
/* 在相邻节点n0、n1间插入节点P */
#[allow(non_snake_case)]
pub fn insert<T>(n0: &Rc<RefCell<ListNode<T>>>, p: Rc<RefCell<ListNode<T>>>) {
    let n1 = n0.borrow_mut().next.take();
    p.borrow_mut().next = n1;
    n0.borrow_mut().next = Some(p);
}

/* 删除节点：
    在链表中删除节点也非常方便，只需要改变一个节点的引用即可 */
pub fn remove<T>(n0: &Rc<RefCell<ListNode<T>>>){
    // 判断n0是否是最后一个节点
    if n0.borrow().next.is_none(){
        return;
    }
    // n0 -> p -> n1
    let p = n0.borrow_mut().next.take();
    if let Some(node) = p{
        let n1 = node.borrow_mut().next.take();
        n0.borrow_mut().next = n1;
    }
}

/* 访问节点：
    在链表中访问节点的效率较低。链表需要从头开始查找节点，也就是说，要访问链表的第i个节点，需要i-1轮，事件复杂度为O(n) */
pub fn access<T: PartialEq>(head: Rc<RefCell<ListNode<T>>>, index: i32) -> Rc<RefCell<ListNode<T>>> {
    if index <= 0 {
        return head;
    }
    if let Some(node) = &head.borrow().next{
        return access(node.clone(), index-1);
    }
    return head;
}

/* 查找节点，遍历链表，查找其中值为target的节点，输出该节点在链表中的索引。此过程也属于线性查找 */
pub fn find<T: PartialEq>(head: Option<Rc<RefCell<ListNode<T>>>>, target: T) -> i32 {
/*     if head.borrow().val == target{
        return index;
    }
    if let Some(node) = &head.borrow_mut().next{
        return find(node.clone(), target, index);
    }

    return -1; */
    let mut current = head;
    let mut index = 0;

    while let Some(node) = current {
        if node.borrow().val == target{
            return index;
        }
        current = node.borrow().next.clone();
        index += 1;
    }
    return -1;
}

/* 链表有三种常见的类型：
    1.单向链表： 节点包含值和指向下一个节点的引用。尾节点指向None。
    通常用于：
        1.栈与队列：当插入与删除都在链表的同一端进行的时候，它的表现为后进先出，对应栈；当它的插入操作在一端链表的一端进行，而删除
        操作在另一端进行时，它的表现为先进先出，对应队列。
        2.哈希表：链式地址是解决哈希冲突的主流方案之一，在该方案中，所有冲突的元素都被放在一个链表中。‘
        3.图：邻接表是表示图的一种常用方式，其中每个图的顶点都于一个链表相关联，链表中的每个元素都代表于该顶点相连的其他顶点。
    2.环形链表： 将尾节点指向头节点（即首尾相接），在环形链表中，任何一个节点都可以视为头节点。
        1.时间片轮转调度算法：在操作系统中，时间片轮转调度算法时一种常见的CPU调度算法，它需要对每一组进程进行循环。每个进程被赋予了一个时间片，当时间片用完后，CPU将切换到下一个进程。
        这种操作可用环形链表来实现。
        2.数据缓冲区：在某些数据缓冲区的实现中，也可能会使用环形链表。比如在音频、视频播放器中，数据流可能会被分成多个缓冲块并放入一个环形链表，以便实现无缝播放。
    3.双向链表： 与单向链表相比，双向链表中包含了两个方向的引用。
        1.高级数据结构：比如在红黑树、B树中，我们需要访问节点的父节点，这时可以通过在节点中保存一个指向父节点的引用来实现，类似于双向链表。
        2.浏览器历史：在网页浏览器中，当用户点击后退或者是前进按钮时，浏览器需要知道用户访问过的前一个网页或者是后一个网页。双向链表使得这种操作变得简单。
        3.LRU算法：在缓存淘汰（LRU）算法中，我们需要凯斯u找到最近最少使用的数据，以及支持快速添加和删除节点。这时候使用双向链表就很合适。
     */

/* 双向链表节点类型*/
struct TwoWayListNode<T>{
    val: T,
    next: Option<Rc<RefCell<TwoWayListNode<T>>>>,
    prev: Option<Rc<RefCell<TwoWayListNode<T>>>>,
} 

/* 构造函数 */
impl<T> TwoWayListNode<T>{
    fn new(val: T) -> Self {
        TwoWayListNode{
            val: val,
            next: None,
            prev: None,
        }
    }
}

fn main() {
    /* 建立节点分为两步，第一步是初始化各个对象，第二步是构建各个节点之间的引用关系。初始化完成后，我们就可以从链表的头节点出发，
    通过引用指向next依次访问所有节点。 */
    /* 初始化链表 1 -> 3 -> 2 -> 5 -> 4 */
    // 初始化各个节点
    let n0 = Rc::new(RefCell::new(ListNode{val: 1, next: None}));
    let n1 = Rc::new(RefCell::new(ListNode{val: 3, next: None}));
    let n2 = Rc::new(RefCell::new(ListNode{val: 2, next: None}));
    let n3 = Rc::new(RefCell::new(ListNode{val: 5, next: None}));
    let n4 = Rc::new(RefCell::new(ListNode{val: 4, next: None}));

    // 构建节点之间的引用
    n0.borrow_mut().next = Some(n1.clone());
    n1.borrow_mut().next = Some(n2.clone());
    n2.borrow_mut().next = Some(n3.clone());
    n3.borrow_mut().next = Some(n4.clone());
    /*链表是由各个独立的节点构成的，我们通常将头节点用来代称链表，比如以上代码中的链表可以记作链表n0. */
}
