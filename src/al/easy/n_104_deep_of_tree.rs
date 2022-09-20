use std::rc::Rc;
use std::cell::RefCell;

use crate::al::tree::TreeNode;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(a) => {
                let left = Self::max_depth(a.borrow_mut().left.take());
                let right = Self::max_depth(a.borrow_mut().right.take());
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

struct Solution{}