pub struct Solution;

// Definition for singly-linked list.
struct ListHead(Option<Box<ListNode>>);

impl ListHead {
    fn len(&self) -> usize {
        let mut cur: &Option<Box<ListNode>> = &self.0;
        let mut count = 0;
        while let Some(node) = cur {
            count += 1;
            cur = &node.next;
        }
        count
    }
    fn prepend(&mut self, val: i32) {
        let mut boxed_node = Box::new(ListNode::new(val));
        boxed_node.next = std::mem::take(&mut self.0); // This will set self.0 to None and move the current self.0 to next!
        self.0 = Some(boxed_node);
    }
}

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
}

fn list_len(head: &Option<Box<ListNode>>) -> usize {
    let mut cur = head;
    let mut result_count: usize = 0;
    while let Some(node) = cur {
        result_count += 1;
        cur = &node.next;
    }
    result_count
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut return_list: Option<Box<ListNode>> = None;

        let mut current_node = head;
        while let Some(boxed_node) = current_node {
            return_list = prepend_node(boxed_node.val, return_list);
            current_node = boxed_node.next;
        }

        return_list
    }
}

fn prepend_node(value: i32, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut boxed_node = Box::new(ListNode::new(value));
    boxed_node.next = head;
    Some(boxed_node)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let mut head: Option<Box<ListNode>> = None;

        assert_eq!(list_len(&head), 0);
        head = prepend_node(5, head);
        assert_eq!(list_len(&head), 1);
        head = Solution::reverse_list(head);

        let mut list_head: ListHead = ListHead(None);
        assert_eq!(list_head.len(), 0);
        list_head.prepend(5);
        assert_eq!(list_head.len(), 1);
    }
}
