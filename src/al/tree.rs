use std::{cell::RefCell, rc::Rc};


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
      right: None
    }
  }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;


    #[test]
    fn it_works1() {

        let s = TreeNode::new(Box::new(Option::None),Box::new(Option::None),Option::Some(55));

        println!("{:?}",s)
    }
}