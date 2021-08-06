#[derive(Debug)]
pub struct ListNode {
    value: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(value: i32) -> ListNode {
        ListNode { value, next: None }
    }

    pub fn append(&mut self, value: i32) {
        match self.next {
            // Don't like this, 'next' as enum could be a better solution.
            Some(ref mut next) => next.append(value),
            None => self.next = Some(Box::new(ListNode::new(value))),
        }
    }

    pub fn preprend(head: Option<Box<ListNode>>, value: i32) -> Option<Box<ListNode>> {
        match head {
            None => Some(Box::new(ListNode { value, next: None })),
            Some(x) => Some(Box::new(ListNode {
                value,
                next: Some(x),
            })),
        }
    }
}

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if let None = l1 {
        return l2;
    }
    if let None = l2 {
        return l1;
    }

    let mut merged_list: Option<Box<ListNode>> = None;
    let mut iter_1 = &l1;
    let mut iter_2 = &l2;

    loop {
        let mut min_l1: Option<i32> = None;
        let mut min_l2: Option<i32> = None;

        if let Some(iter_node) = iter_1 {
            min_l1 = Some(iter_node.value);
        }

        if let Some(iter_node) = iter_2 {
            min_l2 = Some(iter_node.value);
        }

        if min_l1 == None && min_l2 == None {
            break;
        }

        if min_l1 != None && min_l1 < min_l2 {
            merged_list = match merged_list {
                None => Some(Box::new(ListNode::new(min_l1.unwrap()))),
                Some(mut x) => {
                    x.append(min_l1.unwrap());
                    Some(x)
                }
            };

            if let Some(iter_node) = iter_1 {
                iter_1 = &iter_node.next;
            }
        } else {
            merged_list = match merged_list {
                None => Some(Box::new(ListNode::new(min_l2.unwrap()))),
                Some(mut x) => {
                    x.append(min_l2.unwrap());
                    Some(x)
                }
            };

            if let Some(iter_node) = iter_2 {
                iter_2 = &iter_node.next;
            }
        }
    }

    merged_list
}
