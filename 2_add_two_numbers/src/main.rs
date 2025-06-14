fn main() {
    println!("Hello, world!");
}

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
}

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: None });
    let mut tail = &mut dummy.next;
    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let mut sum = carry;

        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }

        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }

        carry = sum / 10;

        *tail = Some(Box::new(ListNode {
            val: sum % 10,
            next: None,
        }));

        tail = &mut tail.as_mut().unwrap().next;
    }

    dummy.next
}
