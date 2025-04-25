pub struct Solution;

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
}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let input = vec![0, 1, 2];
        let mut list_head: Option<Box<ListNode>> = None;
        let list_node = ListNode::new(3);
        list_head = Some(Box::new(list_node));
        println!("result: {:?}", list_head);
        let result = list_head.insert(Box::new(ListNode::new(6)));
        println!("result: {:?}", result);
    }
}
