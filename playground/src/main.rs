// https://leetcode.com/problems/merge-two-sorted-lists/

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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() {
        return list2;
    }
    if list2.is_none() {
        return list1;
    }

    let mut result: Box<ListNode> = Box::new(ListNode::new(3));
    let node1: Box<ListNode>;
    let node2: Box<ListNode>;

    let (mut head1, mut head2) = (list1, list2);

    if head1.as_ref().unwrap().val < head2.as_ref().unwrap().val {
        result = Box::new(ListNode::new(head1.as_ref().unwrap().val));
        node1 = match head1 {
            Some(n) => Box::new(match &n.as_ref().next {
                Some(n) => **n,
                None => ListNode::new(0),
            }),
            // this should never happen:
            None => Box::new(ListNode::new(0)),
        };
        // node2 = match list1 {
        //     Some(n) => Box::new(*n),
        //     // this should never happen:
        //     None => Box::new(ListNode::new(0)),
        // };
    } else {
        // result = Box::new(ListNode::new(list2.as_ref().unwrap().val));
        // node1 = list1;
        // node2 = list2;
    }
    // let cur_node = result;
    // let is_complete = false;
    // while !is_complete {
    //     if node1 == None && node 2 == None {
    //         cur_node.next = None;
    //         break;
    //     } else if node1 == None {
    //         cur_node.next = ListNode::new(list2.val);
    //         node2 = node2.next
    //     } else if node2 == None {
    //         cur_node.next = ListNode::new(list1.val);
    //         node1 = node1.next;
    //     } else if node1.val < list2.val {
    //         cur_node.next = ListNode::new(list1.val);
    //         node1 = node1.next;
    //     } else {
    //         cur_node.next = ListNode::new(list2.val);
    //         node2 = node2.next
    //     }
    // }
    // result
    Some(result)
}
