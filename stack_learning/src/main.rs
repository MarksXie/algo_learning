/* 栈是一种遵循先入后出逻辑的线性数据结构。
     */

use std::{cell::RefCell, rc::Rc};

fn main() {
    /* 栈的常用操作 */
    /* 初始化栈 */
    // 把Vec当作栈来使用
    let mut stack: Vec<i32> = Vec::new();

    /* 元素入栈 */
    stack.push(1);
    stack.push(3);
    stack.push(2);
    stack.push(5);
    stack.push(4);
    println!("{:?}", stack);

    /* 访问栈顶元素 */
    let top = stack.last().unwrap();
    println!("top is:{}", &top);
    
    /* 元素出栈 */
    let pop = stack.pop().unwrap();
    println!("pop is {}", &pop);

    /* 获取栈的长度 */
    let size = stack.len();
    println!("size is {}", &size);

    /* 判断是否为空 */
    let is_empty = stack.is_empty();
    println!("{}", &is_empty);
}

/* 栈的实现：栈遵循后入先出的原则，因此我们只能在栈顶体添加或者删除元素。然而，数组或者链表可以在任意位置添加或者删除元素，因此栈可以视为一种受限制的数组或者链表。 */
/* 1.基于链表的实现 */
// 基于链表实现的栈
struct ListNode<T>{
    elem: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> ListNode<T>  {
    fn new(elem: T) -> Rc<RefCell<ListNode<T>>> {
        Rc::new(RefCell::new(ListNode{
            elem,
            next: None,
        }))
    }
}

#[allow(dead_code)]
struct LinkedListStack<T>{
    stack_peek: Option<Rc<RefCell<ListNode<T>>>>,    // 将头节点视作栈顶
    stack_size: usize,  // 栈的长度
}

#[allow(dead_code)]
impl<T: Copy> LinkedListStack<T>  {
    pub fn new() -> Self {
        LinkedListStack{
            stack_peek: None,
            stack_size: 0,
        }
    }

    /* 获取栈的长度 */
    pub fn size(&self) -> usize {
        return self.stack_size;
    }

    /* 判断栈是否为空 */
    pub fn is_empty(&self) -> bool {
        return self.size() == 0;
    }

    /* 入栈 */
    pub fn push(&mut self, elem: T) {
        let node = ListNode::new(elem);
        node.borrow_mut().next = self.stack_peek.take();
        self.stack_peek = Some(node);
        self.stack_size += 1;
    }

    /* 出栈 */
    pub fn pop(&mut self) -> Option<T> {
        self.stack_peek.take().map(| old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) =>{
                    self.stack_peek = Some(new_head);
                }
                None => {
                    self.stack_peek = None;
                }
            }
            self.stack_size -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    /* 访问栈顶元素 */
    pub fn peek(&self) -> Option<&Rc<RefCell<ListNode<T>>>> {
        self.stack_peek.as_ref()
    }

    /* 将list转化成array并返回 */
    pub fn to_array(&self, head: Option<&Rc<RefCell<ListNode<T>>>>) -> Vec<T> {
        if let Some(node) = head{
            let mut elems = self.to_array(node.borrow().next.as_ref());
            elems.push(node.borrow().elem);
            return elems;
        }
        return Vec::new();
    }
}

/* 2.基于数组的实现 */
// 由于入栈的元素可能会源源不断的增加，因此我们需要一个动态数组来实现栈，这样就无须自行处理数组扩容的的问题。
/* 基于数组实现的栈 */
struct ArrayStack<T>{
    stack: Vec<T>,
}

#[allow(dead_code)]
impl <T: Clone> ArrayStack<T> {
    /* 初始化栈 */
    fn new() -> ArrayStack<T> {
        ArrayStack::<T> { 
            stack: Vec::<T>::new(), 
        }
    }

    /* 获取栈的长度 */
    fn size(&self) -> usize {
        return self.stack.len();
    }

    /* 判断栈是否为空 */
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /* 入栈 */
    fn push(&mut self, elem: T) {
        self.stack.push(elem);
    }

    /* 出栈 */
    fn pop(&mut self) -> Option<T>{
        self.stack.pop()
    }

    /* 访问栈顶元素 */
    fn peek(&self) -> Option<&T>{
        if self.is_empty(){
            panic!("Stack is empty!");
        }
        self.stack.last()
    }

    /*返回&vec */
    fn to_array(&self) -> &Vec<T> {
        &self.stack
    }
}

/* 时间效率：
    1.基于数组实现的栈在处罚扩容时效率会降低，但由于扩容是低频操作，所以平均效率更高。
    2.基于链表实现的栈可以提供更加稳定的效率表现。 */

/* 空间效率：
    1.基于数组实现的栈可能会造成一定的空间浪费。
    2.由于链表节点需要额外的存储指针，所以链表节点占用的空间会相对较大。 */

/* 栈的典型应用
    1.浏览器的后退与前进、软件中的撤销与反撤销。每当我们打开一个新的网页的时候，浏览器就会对上一个网页进行入栈，这样我们就可以通过后退操作返回上一个网页。
    后退操作实际上就是在进行出栈操作。如果要同时支持后退和前进，那么需要两个栈来配合实现。
    2.程序内存管理。每次调用函数时，系统都会在栈顶添加一个栈帧，用于记录函数的上下文信息。在递归函数中，向下递推节点会不断地进行入栈操作，
    而向上回溯阶段则会不断执行出栈操作。 */

