// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn with_vec(arr: Vec<i32>) -> Self {
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