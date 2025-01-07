/* 二叉树是一种非线性数据结构，代表“祖先“和”后代“之间的派生关系，体现了”一分为二“的分治逻辑。与链表类似，二叉树的基本单元是节点，
每个节点包含值、左子节点引用和右子节点引用。 */

use std::rc::Rc;
use std::cell::RefCell;

#[allow(dead_code)]
/* 二叉树节点结构体 */
struct TreeNode{
    val: i32,   // 节点值
    left: Option<Rc<RefCell<TreeNode>>>,    // 左子节点引用
    right: Option<Rc<RefCell<TreeNode>>>,   // 右子节点引用
}
/* 每个节点都有两个引用，分别指向左子节点和右子节点，该节点被称为这两个子节点的父节点。当给定一个二叉树节点时，我们将该节点的左子节点
及其以下的树称为该节点的左子树，同理可得右子树。
    在二叉树中，除叶节点外，其他所有节点都包含子节点和非空子树。 */

/* 二叉树常见术语：
    1.根节点： 位于二叉树顶层的节点，没有父节点。
    2.叶节点： 没有子节点的节点，其两个指针均指向None。
    3.边： 连接两个节点的线段，即节点引用。
    4.节点所在的层： 从顶至底递增，根节点所在层为1。
    5.节点的度： 节点的子节点的数量。在二叉树中，度的取值范围为0、1、2。
    6.二叉树的高度： 从根节点到最远叶节点所经过的边的数量。
    7.节点的深度： 从根节点到该节点所经过的边的数量。
    8.节点的高度： 从距离该节点最远的叶节点到该节点的所经过的边的数量。 */


#[allow(dead_code)]
impl TreeNode {
    /* 构造方法 */
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { val: val, left: None, right: None }))
    }
}

fn main() {
    /* 初始化二叉树 */
    // 初始化节点
    let n1 = TreeNode::new(1);
    let n2 = TreeNode::new(2);
    let n3 = TreeNode::new(3);
    let n4 = TreeNode::new(4);
    let n5 = TreeNode::new(5);

    // 构建节点之间的引用(指针)
    n1.borrow_mut().left = Some(n2.clone());
    n1.borrow_mut().right = Some(n3);
    n2.borrow_mut().left = Some(n4);
    n2.borrow_mut().right = Some(n5);

    /* 插入与删除节点 */
    let p = TreeNode::new(0);
    // 在n1 -> n2中间插入节点p
    n1.borrow_mut().left = Some(p.clone());
    p.borrow_mut().left = Some(n2.clone());

    // 删除节点
    n1.borrow_mut().left = Some(n2);

}
