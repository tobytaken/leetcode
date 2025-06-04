fn main() {}

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

fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: None });
    let mut tail = &mut dummy.next;

    while let (Some(node1), Some(node2)) = (&mut list1, &mut list2) {
        if node1.val <= node2.val {
            let next_node = list1.take().unwrap();
            *tail = Some(next_node);
            tail = &mut tail.as_mut().unwrap().next;
            list1 = tail.take();
        } else {
            let next_node = list2.take().unwrap();
            *tail = Some(next_node);
            tail = &mut tail.as_mut().unwrap().next;
            list2 = tail.take();
        }
    }

    *tail = if list1.is_some() { list1 } else { list2 };

    dummy.next
}
