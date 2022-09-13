use std::borrow::{BorrowMut, Borrow};
use std::rc::Rc;
use std::cell::RefCell;

use crate::al::tree::TreeNode;



struct Solution{}

impl Solution {
    // pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    //     fn getAns(ans : &mut Vec<Option<i32>>,root : Option<Rc<RefCell<TreeNode>>>){
    //         match root {
    //             Some(node) => {
    //                 getAns(ans,node.borrow_mut().left.take());
    //                 ans.push(node.borrow().val);
    //                 getAns(ans,node.borrow_mut().right.take());
    //             },
    //             None =>{}
    //         }
    //     }

    //     let mut ans = Vec::<Option<i32>>::new();
    //     getAns(&mut ans,root);
    //     ans
    // }
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, cell::RefCell, rc::Rc};
    use crate::al::easy::n_94::Solution;

    use super::TreeNode;

    #[test]
    fn it_works1() {
        // TreeNode bt = TreeNode.createBinaryTree(new Integer[]{-1,0,3,-2,4,null,null,8});
        let array = vec![Some(1),None,Some(2),Some(3)];

        let res = TreeNode::create_binary_tree(array);
        println!("{:?}",res);
        let mut queue = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        // if let Some(x) = res{
        //     queue.push_back(x)
        // }
        queue.push_back(res.as_ref().unwrap().clone());
        while !queue.is_empty() {

            if let Some(pop) = queue.pop_back(){

                let s = pop.borrow();
                println!("{:?}", s.val);
                let xxl = s.left.as_ref();
                if let Some(aaa) = xxl{
                    queue.push_back(aaa.clone())
                }
                let xxr = s.right.as_ref();
                if let Some(aaar) = xxr{
                    queue.push_back(aaar.clone())
                }

            }
        }

        // let s = Solution::inorder_traversal(Some(res.unwrap()));
        // println!("{:?}",s)
    }

    #[test]
    fn getVecByStr(){
        let s = String::from("1,null,2,3");

        let arr :Vec<&str>= s.split(",").collect();

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
        println!("{:?}",String::from_iter(str_last))

    }
}