#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    use crate::al::list_node::ListNode;

    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ptr = head.as_ref();
        let mut stack = VecDeque::<(i32, usize)>::new();
        let mut idx : usize = 0;
        let mut rst = vec![];
        while ptr.is_some() {
            let node = ptr.unwrap();
            let val = node.val;
            rst.push(0);
            while stack.front().is_some() && stack.front().unwrap().0 < val{
                let top = stack.pop_front().unwrap();
                rst[top.1] = val;
            }
            stack.push_front((val,idx));
            idx += 1;
            ptr = node.next.as_ref();
        }

        rst
    }

    #[test]
    fn it_work() {
        let ans = ways_to_make_fair1(vec![1, 1, 1, 1, 1, 1, 1]);
        println!("{}", ans)
    }
}