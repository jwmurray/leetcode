pub struct Solution;

// Definition for singly-linked list.
struct ListHead(Option<Box<ListNode>>);

impl ListHead {
    fn new(option_box_node: Option<Box<ListNode>>) -> Self {
        ListHead(option_box_node)
    }

    // return the length of the list
    fn len(&self) -> usize {
        let mut cur: &Option<Box<ListNode>> = &self.0;
        let mut count = 0;
        while let Some(node) = cur {
            count += 1;
            cur = &node.next;
        }
        count
    }

    // Add a node to the front of the list
    fn prepend(&mut self, val: i32) {
        let mut boxed_node = Box::new(ListNode::new(val));
        boxed_node.next = std::mem::take(&mut self.0); // This will set self.0 to None and move the current self.0 to next!
        self.0 = Some(boxed_node);
    }

    fn to_vector(&self) -> Vec<i32> {
        let mut return_vector = vec![];
        let mut some_current_node = &self.0;
        while let Some(current_node) = some_current_node {
            return_vector.push(current_node.val);
            some_current_node = &current_node.next;
        }
        return_vector
    }

    fn prepend_node(&mut self, mut node: ListNode) {
        node.next = std::mem::take(&mut self.0); // save off the first node in the list which is ListHead.0 as the next of the incoming node -- but we have to take ownership of it
        self.0 = Some(Box::new(node)); // set the ListHead.0 to the Some(of the  incoming node)
    }

    fn reverse(&mut self) {
        let mut some_current_node = std::mem::take(&mut self.0);
        while let Some(mut node) = some_current_node {
            some_current_node = std::mem::take(&mut node.next);
            self.prepend_node(*node);
        }
    }

    fn to_i32_vector(&self) -> Vec<i32> {
        let mut result_vector: Vec<i32> = vec![];
        let mut some_node = &self.0;
        while let Some(node) = some_node {
            result_vector.push(node.val);
            some_node = &node.next;
        }
        result_vector
    }

    fn from_vector(vector: &[i32]) -> Self {
        let mut list_head = ListHead::new(None);
        for val in vector {
            list_head.prepend(*val);
            list_head.reverse();
        }
        list_head
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

fn node_list_to_vector(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut result_vector: Vec<i32> = vec![];
    let mut some_current_node = head;
    while let Some(node) = some_current_node {
        result_vector.push(node.val);
        some_current_node = &node.next;
    }
    result_vector
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
        head = prepend_node(1, head);
        assert_eq!(list_len(&head), 2);
        assert_eq!(node_list_to_vector(&head), vec![1, 5]);
        head = Solution::reverse_list(head);
        assert_eq!(node_list_to_vector(&head), vec![5, 1]);

        let mut list_head: ListHead = ListHead(None);
        assert_eq!(list_head.len(), 0);
        list_head.prepend(5);
        assert_eq!(list_head.len(), 1);
        list_head.prepend(1);
        assert_eq!(list_head.to_i32_vector(), vec![1, 5]);
        assert_eq!(list_head.len(), 2);
        list_head.reverse();
        assert_eq!(list_head.to_i32_vector(), vec![5, 1]);

        let list_head = ListHead::from_vector(&vec![1, 5]);
        assert_eq!(list_head.to_vector(), vec![1, 5]);
    }
}
