// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::f32::consts::E;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution{}

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn setNode(nums : &[i32]) -> Option<Rc<RefCell<TreeNode>>>{

            if nums.is_empty(){
                return Option::None;
            };
        
            let max = *nums.iter().max().unwrap();
            let mut iter = nums.split(|x| *x == max).into_iter();
        
            Some(Rc::new(RefCell::new(TreeNode{
                val:max,
                left:setNode(iter.next().unwrap()),
                right:setNode(iter.next().unwrap()),
            })))
        }
        setNode(&nums)
    }

}

