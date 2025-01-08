/* 二叉树遍历
    从物理结构的角度来看，树是一种基于链表的数据结构，因此其遍历方式是通过指针逐个访问节点。然而，树是一种非线性数据结构，这使得遍历树比遍历链表更加复杂，需要借助搜索算法来实现。
    二叉树常见的遍历方式包括层序遍历、前序遍历和后序遍历等。 */

/* 层序遍历
    层序遍历从顶部到底部逐层遍历二叉树，并在每一层按照从左到右的顺序访问节点。
    层序遍历从本质上属于广度优先遍历，也称为广度优先搜索。它体现了一种“一圈一圈向外扩展”的逐层遍历方式。 */
/* 代码实现
    广度优先遍历通常借助“队列”来实现。队列遵循“先进先出”的规则，而广度优先遍历则遵循“逐层推进”的规则，两者背后的思想是一直的。实现代码如下： */

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[allow(dead_code)]
struct TreeNode{
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
fn level_order(root: &Rc<RefCell<TreeNode>>) ->  Vec<i32> {
    // 初始化队列，加入根节点
    let mut que = VecDeque::new();
    que.push_back(root.clone());
    // 初始化一个列表,用于保存队列
    let mut vec = Vec::new();

    while let Some(node) = que.pop_front() {
        // 队列出队
        vec.push(node.borrow().val);    // 保存节点值
        if let Some(left) = node.borrow().left.as_ref() {
            que.push_back(left.clone());
        }
        if let Some(right) = node.borrow().right.as_ref() {
            que.push_back(right.clone());
        }
    }
    vec
}  
/* 复杂度分析
    时间复杂度O(n): 所有节点被访问一次，使用O(n)时间，其中n是节点数量。
    空间复杂度O(n): 在最差的情况下，即满二叉树下，遍历到最底层之前，队列最多同时存在(n+1)/2个节点，占用O(n)空间。 */

/* 前序、中序、后序遍历
    相应的，前序、中序和后序遍历都属于深度优先遍历，也称为深度优先搜索，它体现了一种“先走到尽头，在回溯继续”的遍历方式。
    深度优先遍历就像是整棵二叉树的外围“走一圈”，在每个节点都会遇到三个位置，分别对应前序遍历、中序遍历和后序遍历。  */
// 代码实现
/* 前序遍历 */
#[allow(dead_code)]
fn pre_order(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];

    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>){
        if let Some(node) = root{
            // 访问优先级： 根节点-> 左子树 -> 右子树
            let node = node.borrow();
            res.push(node.val);
            dfs(node.left.as_ref(), res);
            dfs(node.right.as_ref(), res);
        }
    }
    dfs(root, &mut result);

    result
}
/* 中序遍历 */
#[allow(dead_code)]
fn in_order(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32>{
    let mut result = vec![];

    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>){
        if let Some(node) = root{
            // 访问优先级: 左子树 -> 根节点 -> 右子树
            let node = node.borrow();
            dfs(node.left.as_ref(), res);
            res.push(node.val);
            dfs(node.right.as_ref(), res);
        }
    }
    dfs(root, &mut result);
    result
}
/* 后序遍历 */
#[allow(dead_code)]
fn post_order(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32>{
    let mut result = vec![];

    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>){
        if let Some(node) = root{
            // 访问优先级: 左子树 -> 右子树 -> 根节点
            let node = node.borrow();
            dfs(node.left.as_ref(), res);
            dfs(node.right.as_ref(), res);
            res.push(node.val);
        }
    }
    dfs(root, &mut result);

    result
}
/* 复杂度分析
    时间复杂度O(n): 所有节点都被访问一次，使用O(n)时间。
    空间复杂度O(n): 在最差情况下，即树退化为链表时，递归深度达到n，系统占用O(n)栈帧空间。 */

fn main() {
    println!("Hello, world!");
}
