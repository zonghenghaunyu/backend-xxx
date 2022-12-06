use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: Option<i32>,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: Option<i32>) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn create_binary_tree(array: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = array.len();
        if len == 0 {
            return Option::None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(array[0])));

        let mut queue = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        queue.push_front(root.clone());
        // let mut is_left = true;
        let mut idx = 1;
        loop {
            let last = queue.pop_back();
            if let Some(x) = last{
                let mut c_node = x.borrow_mut();
                //left
                c_node.left = Some(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                idx += 1;
                if idx >= len {
                    break;
                }
                queue.push_front(c_node.left.as_ref().unwrap().clone());
                //right
                c_node.right = Some(Rc::new(RefCell::new(TreeNode::new(array[idx]))));
                queue.push_front(c_node.right.as_ref().unwrap().clone());
                idx += 1;
                if idx == len {
                    break;
                }
            }
        }
        // for i in 1..array.len() {
        //     let last = queue.pop_back();
        //     if let Some(x) = last {
        //         if i % 2 == 1 {
        //             let mut change = x.borrow_mut();
        //             let thl = Rc::new(RefCell::new(TreeNode::new(array[i])));
        //             change.left = Option::Some(thl.clone());
        //             queue.push_front(thl);
        //             // is_left = false;
        //         } else {
        //             let mut change = x.borrow_mut();
        //             let thr = Rc::new(RefCell::new(TreeNode::new(array[i])));
        //             change.right = Option::Some(thr.clone());
        //             queue.push_front(thr);
        //         }
        //     }
        // }
        Some(root.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::al::tree::tree_node::TreeNode;
    use std::{cell::RefCell, collections::VecDeque, rc::Rc};

    #[test]
    fn it_works1() {
        // TreeNode bt = TreeNode.createBinaryTree(new Integer[]{-1,0,3,-2,4,null,null,8});
        let array = vec![
            Some(-1),
            Some(0),
            Some(3),
            Some(-2),
            Some(4),
            None,
            None,
            Some(8),
        ];

        let res = TreeNode::create_binary_tree(array);
        println!("{:?}", res);
        println!("==========================================================================");
        let mut queue = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        if let Some(x) = res {
            queue.push_front(x)
        }
        while !queue.is_empty() {
            if let Some(pop) = queue.pop_back() {
                let s = pop.borrow();
                println!("{:?}", s.val);
                let xxl = s.left.as_ref();
                if let Some(aaa) = xxl {
                    queue.push_front(aaa.clone())
                }
                let xxr = s.right.as_ref();
                if let Some(aaar) = xxr {
                    queue.push_front(aaar.clone())
                }
            }
        }
    }

    #[test]
    fn get_vec_by_str() {
        let s = String::from("-1,0,3,-2,4,null,null,8");

        let arr: Vec<&str> = s.split(",").collect();

        let mut rtl = Vec::<String>::new();
        let mut str_last: Vec<char> = Vec::new();
        for ele in arr {
            // let val = ele.to_owned();
            rtl.push(ele.to_string());
        }
        for ele in rtl {
            let charss = ele.chars();
            let mut flag = true;
            for ele in charss.clone() {
                if ele == 'n' {
                    flag = false;
                    break;
                }
            }
            if flag {
                str_last.push('S');
                str_last.push('o');
                str_last.push('m');
                str_last.push('e');
                str_last.push('(');
            }
            for elem in charss {
                if elem == 'n' {
                    str_last.push('N');
                    str_last.push('o');
                    str_last.push('n');
                    str_last.push('e');
                    break;
                }
                str_last.push(elem);
            }
            if flag {
                str_last.push(')');
            }
            str_last.push(',');
        }
        println!("{:?}", String::from_iter(str_last))
    }
}
