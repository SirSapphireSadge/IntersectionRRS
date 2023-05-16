#[derive(Debug)]
struct ListNode<T> {
    val: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(val: T) -> Self {
        ListNode { val, next: None }
    }
}

fn get_intersection_node<T>(head1: Option<Box<ListNode<T>>>, head2: Option<Box<ListNode<T>>>) -> Option<&Box<ListNode<T>>> {
    let (mut len1, mut len2) = (0, 0);

    let mut curr1 = head1.as_ref();
    while let Some(node) = curr1 {
        len1 += 1;
        curr1 = node.next.as_ref();
    }

    let mut curr2 = head2.as_ref();
    while let Some(node) = curr2 {
        len2 += 1;
        curr2 = node.next.as_ref();
    }

    let mut longer = if len1 > len2 { head1 } else { head2 };
    let mut shorter = if len1 > len2 { head2 } else { head1 };

    let diff = (len1 - len2).abs();
    for _ in 0..diff {
        if let Some(node) = longer {
            longer = node.next.clone();
        }
    }

    while let (Some(node1), Some(node2)) = (longer, shorter) {
        if std::ptr::eq(&node1, &node2) {
            return Some(node1);
        }
        longer = node1.next.clone();
        shorter = node2.next.clone();
    }

    None
}

// Example usage:
fn main() {
    // Create the first linked list: 1 -> 2 -> 3 -> 4 -> 5 -> 6
    let mut head1 = Some(Box::new(ListNode::new(1)));
    let mut curr1 = head1.as_mut();

    for i in 2..=6 {
        if let Some(node) = curr1 {
            node.next = Some(Box::new(ListNode::new(i)));
            curr1 = node.next.as_mut();
        }
    }

    // Create the second linked list: 9 -> 8 -> 5 -> 6
    let mut head2 = Some(Box::new(ListNode::new(9)));
    let mut curr2 = head2.as_mut();

    for &val in &[8, 5, 6] {
        if let Some(node) = curr2 {
            node.next = Some(Box::new(ListNode::new(val)));
            curr2 = node.next.as_mut();
        }
    }

    // Find the intersection of the two linked lists
    if let Some(node) = get_intersection_node(head1, head2) {
        println!("Intersection node value: {}", node.val);
    } else {
        println!("No intersection found.");
    }
}
