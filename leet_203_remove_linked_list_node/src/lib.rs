// This is the LeetCode-compatible solution
mod list_node {
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
}

// Use the struct ListNode from the list_node module
use list_node::ListNode;

pub struct Solution;

pub struct Nodes<'a> {
    node: Option<&'a ListNode>,
}

impl<'a> Iterator for Nodes<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.node {
            let val = current.val;
            self.node = current.next.as_ref().map(|boxed_node| boxed_node.as_ref());
            Some(val)
        } else {
            None
        }
    }
}

impl ListNode {
    pub fn from_slice(slice: &[i32]) -> Option<Box<Self>> {
        let mut head = None;

        for &val in slice.iter().rev() {
            let mut new_node = Box::new(ListNode::new(val));
            new_node.next = head;
            head = Some(new_node);
        }

        head
    }

    pub fn nodes(&self) -> Nodes {
        Nodes { node: Some(self) }
    }
}

impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        // Remove matching nodes from the beginning without cloning
        while let Some(node) = head.as_ref() {
            if node.val == val {
                // Use take() and replace to transfer ownership without cloning
                head = head.take().unwrap().next;
            } else {
                break;
            }
        }

        // If the list is empty after removing head nodes
        if head.is_none() {
            return None;
        }

        // Remove nodes from the rest of the list
        let mut current = head.as_mut().unwrap();

        while let Some(next) = current.next.as_ref() {
            if next.val == val {
                // Take ownership of next.next without cloning
                let next_next = current.next.as_mut().unwrap().next.take();
                current.next = next_next;
            } else {
                // Move to next node only if we didn't remove a node
                current = current.next.as_mut().unwrap();
            }
        }

        head
    }
}

// Add this for LeetCode compatibility
#[cfg(feature = "serialize")]
pub trait Serialize {
    fn serialize(&self) -> String;
}

#[cfg(feature = "serialize")]
impl Serialize for Option<Box<ListNode>> {
    fn serialize(&self) -> String {
        match self {
            None => String::from("[]"),
            Some(head) => {
                let mut result = String::from("[");
                let mut current = Some(head.as_ref());

                while let Some(node) = current {
                    result.push_str(&node.val.to_string());
                    current = node.next.as_ref().map(|n| n.as_ref());

                    if current.is_some() {
                        result.push_str(", ");
                    }
                }

                result.push(']');
                result
            }
        }
    }
}

// Keep our test implementation
#[cfg(test)]
mod tests {
    use super::*;
    use list_node::ListNode;

    #[test]
    fn test_remove_elements() {
        // Example 1: [1,2,6,3,4,5,6], val = 6 → [1,2,3,4,5]
        let mut list = ListNode::new(1);
        let mut node = &mut list;
        node.next = Some(Box::new(ListNode::new(2)));
        node = node.next.as_mut().unwrap();
        node.next = Some(Box::new(ListNode::new(6)));
        node = node.next.as_mut().unwrap();
        node.next = Some(Box::new(ListNode::new(3)));
        node = node.next.as_mut().unwrap();
        node.next = Some(Box::new(ListNode::new(4)));
        node = node.next.as_mut().unwrap();
        node.next = Some(Box::new(ListNode::new(5)));
        node = node.next.as_mut().unwrap();
        node.next = Some(Box::new(ListNode::new(6)));

        let input = Some(Box::new(list));
        let result = Solution::remove_elements(input, 6);

        // Convert result to vector for easy assertion
        let mut result_vec = Vec::new();
        let mut current = result;
        while let Some(node) = current {
            result_vec.push(node.val);
            current = node.next;
        }

        assert_eq!(result_vec, vec![1, 2, 3, 4, 5]);

        // Example 2: [] → []
        let empty_input: Option<Box<ListNode>> = None;
        let empty_result = Solution::remove_elements(empty_input, 1);
        assert_eq!(empty_result, None);

        // Example 3: [7,7,7,7] → []
        let mut all_same = ListNode::new(7);
        let mut node = &mut all_same;
        node.next = Some(Box::new(ListNode::new(7)));
        node = node.next.as_mut().unwrap();
        node.next = Some(Box::new(ListNode::new(7)));
        node = node.next.as_mut().unwrap();
        node.next = Some(Box::new(ListNode::new(7)));

        let all_same_input = Some(Box::new(all_same));
        let all_same_result = Solution::remove_elements(all_same_input, 7);
        assert_eq!(all_same_result, None);
    }
}
