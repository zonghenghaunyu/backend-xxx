
#[cfg(test)]
mod tests {
    use std::cell::{RefCell};
    use std::rc::Rc;
    use crate::al::tree::tree_node::TreeNode;

    #[test]
    fn it_works1() {
        max_depth(Option::Some(Rc::new(RefCell::new(TreeNode::new(Option::Some(5))))));
    }
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(a) => {
                let left = max_depth(a.borrow_mut().left.take());
                let right = max_depth(a.borrow_mut().right.take());
                if left > right{
                    left + 1
                }else {
                    right + 1
                }
            },
            None => {
                0
            }
        }
    }
}