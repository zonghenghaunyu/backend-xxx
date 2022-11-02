#[derive(Debug)]
struct AA {
    val: i32,
    id: i32,
}

impl AA {
    pub fn new(val: i32, id: i32) -> Self {
        AA { val, id }
    }
}

#[cfg(test)]
mod tests {
    use crate::al::tree::traversing::AA;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn testse() {
        // let array = vec![Some(-1), Some(0), Some(3), Some(-2), Some(4), None, None, Some(8)];
        // let res = TreeNode::create_binary_tree(array);
        //
        // if let Some(rr) = res{
        //     if let Some(s) = rr.val{
        //         println!("{}",s)
        //     }
        // }
        let rct = Rc::new(RefCell::new(AA::new(5, 3)));
        let refc = RefCell::new(AA::new(5, 3));

        println!("{:?}", rct);
        println!("{:?}", refc);
    }
    // https://www.jianshu.com/p/7a62dcc96304
    // #[test]
    // fn it_works1() {
    //     // TreeNode bt = TreeNode.createBinaryTree(new Integer[]{-1,0,3,-2,4,null,null,8});
    //     let array = vec![Some(-1),Some(0),Some(3),Some(-2),Some(4),None,None,Some(8)];
    //
    //     let res = TreeNode::create_binary_tree(array);
    //
    //     if let Some(l) = res{
    //
    //         pri_recursion(l);
    //     }
    // }

    //前
    // fn pri_recursion(node : Rc<RefCell<TreeNode>>){
    //
    //     // let a = *node.left;
    //     let rn = node.into_inner();
    //     if let Some(l) = rn.left{
    //         pri_recursion(l);
    //     }
    //     // pri_recursion(*node.left);
    //     if let Some(a) = rn.val{
    //         println!("{}",a)
    //     }else {
    //         println!("null")
    //     }
    //     if let Some(r) = rn.right{
    //         pri_recursion(r);
    //     }
    //
    // }

    //中

    //后

    //前

    //中

    //后
}
