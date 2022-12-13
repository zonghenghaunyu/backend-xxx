// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn with_vec(arr: Vec<i32>) -> Self {
        let mut root: Option<Box<ListNode>> = None;
        let len = arr.len();
        for i in (1..len).rev() {
            let mut node = Self::new(arr[i]);
            node.next = root;
            root = Some(Box::new(node));
        }
        let mut node = Self::new(arr[0]);
        node.next = root;
        node
    }
}

#[cfg(test)]
mod tests {
    use crate::al::hard::n_23::ListNode;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    #[test]
    fn rever() {
        let mut heap = BinaryHeap::new();
        let x = vec![11, 31, 9, 87, 90, 61, 23, 69, 23];
        println!("{:?}", x);
        for i in x {
            heap.push(i);
        }

        while !heap.is_empty() {
            let option = heap.pop();
            println!("{:?}", option)
        }
    }

    #[test]
    fn it_works() {
        // [[1,4,5],[1,3,4],[2,6]]
        let n1 = ListNode::with_vec(vec![1, 3, 5]);
        let n2 = ListNode::with_vec(vec![2, 8]);
        let n3 = ListNode::with_vec(vec![6, 7]);

        let arr = vec![Some(Box::new(n1)), Some(Box::new(n2)), Some(Box::new(n3))];
        let a = merge_k_lists_xx(arr);
        println!("{:?}", a)
    }

    pub fn merge_k_lists_xx(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut arr = lists.iter().fold(Vec::<i32>::new(), |mut a, mut plist| {
            while let Some(node) = plist {
                a.push(node.val);
                plist = &node.next;
            }
            a
        });
        arr.sort_by(|a, b| a.cmp(b));
        let mut ans = Box::new(ListNode::new(0));
        let mut chang = &mut ans;
        for x in arr {
            chang.next = Some(Box::new(ListNode::new(x)));
            chang = chang.next.as_mut().unwrap();
        }
        ans.next
    }

    #[test]
    fn merge_k_lists_test() {
        merge_k_lists(vec![]);
    }
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut ans = Box::new(ListNode::new(0));
        let mut pans = &mut ans;
        let mut heap = BinaryHeap::new();
        for list in lists {
            let mut plist = &list;
            while let Some(node) = plist {
                heap.push(Reverse(node.val));
                plist = &node.next;
            }
        }
        while let Some(Reverse(n)) = heap.pop() {
            pans.next = Some(Box::new(ListNode::new(n)));
            pans = pans.next.as_mut().unwrap();
        }

        println!("{:?}", ans);
        ans.next
    }
}
