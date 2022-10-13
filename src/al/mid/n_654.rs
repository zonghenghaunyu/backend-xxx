
#[cfg(test)]
mod tests {
    // use std::cmp::min;

    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::al::tree::tree_node::TreeNode;

    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn set_node(nums : &[i32]) -> Option<Rc<RefCell<TreeNode>>>{

            if nums.is_empty(){
                return Option::None;
            };

            let max = *nums.iter().max().unwrap();
            let mut iter = nums.split(|x| *x == max).into_iter();

            Some(Rc::new(RefCell::new(TreeNode{
                val:Option::Some(max),
                left: set_node(iter.next().unwrap()),
                right: set_node(iter.next().unwrap()),
            })))
        }
        set_node(&nums)
    }
    #[test]
    fn it_works() {
        let s = vec![10, 15, 20];
        construct_maximum_binary_tree(s);
    }
}


