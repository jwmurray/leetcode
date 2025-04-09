struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListHead {
    pub first: Option<Box<ListNode>>,
}

impl ListHead {
    pub fn from_array(a: &[i32]) -> Self {
        let mut list_head = ListHead { first: None };
        let mut current = &mut list_head.first;

        for &val in a {
            let new_node = Box::new(ListNode::new(val));
            *current = Some(new_node);
            if let Some(node) = current {
                current = &mut node.next;
            }
        }

        list_head
    }
}

impl std::fmt::Display for ListHead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = &self.first;
        write!(f, "[")?;

        while let Some(node) = current {
            write!(f, "{}", node.val)?;
            current = &node.next;
            if current.is_some() {
                write!(f, ", ")?;
            }
        }

        write!(f, "]")
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

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self;
        write!(f, "[")?;
        write!(f, "{}", current.val)?;

        while let Some(next) = &current.next {
            write!(f, ", {}", next.val)?;
            current = next;
        }

        write!(f, "]")
    }
}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();
        while let Some(node) = current {
            while let Some(next) = &node.next {
                if node.val == next.val {
                    node.next = next.next.clone();
                } else {
                    break;
                }
            }
            current = node.next.as_mut();
        }
        head
    }

    fn lists_equal(a: &Option<Box<ListNode>>, b: &Option<Box<ListNode>>) -> bool {
        let mut a_current = a;
        let mut b_current = b;

        loop {
            match (a_current, b_current) {
                (None, None) => return true,
                (Some(a_node), Some(b_node)) if a_node.val == b_node.val => {
                    a_current = &a_node.next;
                    b_current = &b_node.next;
                }
                _ => return false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_0() {
        let list = [1, 1, 1];
        let head = ListHead::from_array(&list);
        let output = [1];
        let expected_output = ListHead::from_array(&output);
        let actual_output = Solution::delete_duplicates(head.first);
        assert!(Solution::lists_equal(
            &actual_output,
            &expected_output.first
        ));
    }

    #[test]
    fn example_1() {
        let list = [1, 1, 2];
        let head = ListHead::from_array(&list);
        let output = [1, 2];
        let expected_output = ListHead::from_array(&output);
        let actual_output = Solution::delete_duplicates(head.first);
        assert!(Solution::lists_equal(
            &actual_output,
            &expected_output.first
        ));
    }

    #[test]
    fn example_2() {
        let input_list = [1, 1, 2, 3, 3];
        let input_head = ListHead::from_array(&input_list);
        let expected_output_list = [1, 2, 3];
        let expected_output_head = ListHead::from_array(&expected_output_list);
        let actual_output = Solution::delete_duplicates(input_head.first);
        assert!(Solution::lists_equal(
            &actual_output,
            &expected_output_head.first
        ));
    }
}
