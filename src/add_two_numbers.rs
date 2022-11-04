// You are given two non-empty linked lists representing two non-negative integers.
// The digits are stored in reverse order, and each of their nodes contains a single digit.
// Add the two numbers and return the sum as a linked list.
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
// Example 1:
// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.
// Example 2:
// Input: l1 = [0], l2 = [0]
// Output: [0]
// Example 3:
// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// Output: [8,9,9,9,0,0,0,1]
// Constraints:
// The number of nodes in each linked list is in the range [1, 100].
// 0 <= Node.val <= 9
// It is guaranteed that the list represents a number that does not have leading zeros.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub(crate) fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// time cost: 0ms
// memory cost: 2.2MB
pub fn add_two_numbers(mut l1: Option<Box<ListNode>>,
                       mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let list1 = l1.unwrap();
    let list2 = l2.unwrap();
    let mut list1_val = list1.val;
    let mut list2_val = list2.val;
    let mut add_result = list1_val + list2_val;
    let mut add;
    if add_result > 9 {
        add = 1;
        add_result -= 10;
    } else {
        add = 0;
    }
    let result_list_node = ListNode::new(add_result);
    let mut result = Box::new(result_list_node);
    let mut current = result.as_mut();
    l1 = list1.next;
    l2 = list2.next;
    loop {
        let list1_is_none = &l1.is_none();
        let list2_is_none = &l2.is_none();
        if *list1_is_none && *list2_is_none {
            break;
        }
        if *list1_is_none {
            list1_val = 0;
            list2_val = l2.as_ref().unwrap().val;
            l2 = l2.unwrap().next;
        } else if *list2_is_none {
            list2_val = 0;
            list1_val = l1.as_ref().unwrap().val;
            l1 = l1.unwrap().next;
        } else {
            list1_val = l1.as_ref().unwrap().val;
            list2_val = l2.as_ref().unwrap().val;
            l1 = l1.unwrap().next;
            l2 = l2.unwrap().next;
        }

        add_result = list1_val + list2_val + add;
        if add_result > 9 {
            add = 1;
            add_result -= 10;
        } else {
            add = 0;
        }
        let next_node = ListNode::new(add_result);
        let mut next_optional = Some(Box::new(next_node));
        current.next = next_optional;
        current = current.next.as_mut().unwrap();
    }
    if add == 1 {
        let next_node = ListNode::new(1);
        current.next = Some(Box::new(next_node));
    }
    return Some(result);
}
