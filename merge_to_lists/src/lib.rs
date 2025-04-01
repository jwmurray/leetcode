#[derive(PartialEq, Eq, Clone, Debug)] // Definition for singly-linked list.
pub struct ListHead {
    pub head: Option<Box<ListNode>>,
}

impl ListHead {
    pub fn new() -> Self {
        ListHead { head: None }
    }

    pub fn add_head(&mut self, val: i32) {
        let mut new_node = Box::new(ListNode::new(val));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    pub fn add_tail(&mut self, val: i32) {
        let new_node = Box::new(ListNode::new(val));

        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = Some(new_node);
                return;
            }
            current = &mut node.next;
        }
    }

    pub fn get_tail(&self) -> Option<&Box<ListNode>> {
        let mut current = &self.head;
        let mut last = None;

        while let Some(node) = current {
            last = Some(node);
            current = &node.next;
        }

        last
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

    pub fn to_vec(&self) -> Vec<i32> {
        let mut return_vec = Vec::new();
        let mut current = self;
        return_vec.push(current.val);
        while let Some(next) = &current.next {
            return_vec.push(next.val);
            current = next;
        }
        return_vec
    }
}

pub fn get_list_length(list: &Option<Box<ListNode>>) -> i32 {
    let mut length = 0;
    let mut current_node = list;
    while let Some(node) = current_node {
        length += 1;
        current_node = &node.next;
    }
    length
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(l)) | (Some(l), None) => Some(l),
            (Some(mut l1), Some(mut l2)) => {
                if l1.val <= l2.val {
                    l1.next = Self::merge_two_lists(l1.next, Some(l2));
                    Some(l1)
                } else {
                    l2.next = Self::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for value in values.iter().rev() {
            let mut node = ListNode::new(*value);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    #[test]
    fn test_merge_simple_lists() {
        let list1 = create_list(vec![1, 2, 4]);
        let list2 = create_list(vec![1, 3, 4]);
        let list1_length = get_list_length(&list1);
        let list2_length = get_list_length(&list2);
        assert_eq!(list1_length, 3);
        assert_eq!(list2_length, 3);
        let merged_list = Solution::merge_two_lists(list1, list2);
        assert_eq!(
            merged_list.as_ref().unwrap().to_vec(),
            vec![1, 1, 2, 3, 4, 4]
        );
    }

    #[test]
    fn test_list_display() {
        let original = vec![1, 2, 3];
        let list = create_list(original.clone());
        println!("List: {:?}", list);
        if let Some(node) = list {
            assert_eq!(node.to_vec(), original);
        }
    }

    #[test]
    fn test_create_list() {
        let list = create_list(vec![1, 2, 3]);
        assert_eq!(list.as_ref().unwrap().val, 1);
        assert_eq!(list.as_ref().unwrap().next.as_ref().unwrap().val, 2);
        assert_eq!(
            list.as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .val,
            3
        );
    }

    #[test]
    fn test_modify_list() {
        let mut list = create_list(vec![1, 2, 3]);

        // Modify the first value
        if let Some(node) = list.as_mut() {
            node.val = 10;
        }

        // Modify the second value
        if let Some(node) = list.as_mut().and_then(|n| n.next.as_mut()) {
            node.val = 20;
        }

        // Modify the third value
        if let Some(node) = list
            .as_mut()
            .and_then(|n| n.next.as_mut())
            .and_then(|n| n.next.as_mut())
        {
            node.val = 30;
        }

        // Verify the changes
        assert_eq!(list.as_ref().unwrap().val, 10);
        assert_eq!(list.as_ref().unwrap().next.as_ref().unwrap().val, 20);
        assert_eq!(
            list.as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .val,
            30
        );
    }

    #[test]
    fn test_get_tail() {
        let mut list = ListHead::new();

        // Test empty list
        assert!(list.get_tail().is_none());

        // Test single node
        list.add_tail(1);
        assert_eq!(list.get_tail().unwrap().val, 1);

        // Test multiple nodes
        list.add_tail(2);
        list.add_tail(3);
        assert_eq!(list.get_tail().unwrap().val, 3);

        // Test that tail's next is None
        assert!(list.get_tail().unwrap().next.is_none());
    }
}
