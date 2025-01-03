// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut carry = 0; // Added to next digit if carry occurs

        // Parsing nodes for the linked list
        let mut temp_node1 = l1;
        let mut temp_node2 = l2;

        let mut list: Option<Box<ListNode>> = None; // List to return 
        let mut sum_val = 0; // Sum of values between both lists
        
        // Parse the lists together
        while temp_node1.as_ref().is_some() || temp_node2.as_ref().is_some() {
            
            // If not at the end fetch the nodes value, if at the end set to 0
            let val1 = match temp_node1.as_ref() { // Borrow temp_node1 with as_ref
                None => 0,
                Some(node) => node.val,
            };

            // If not at the end fetch the nodes value, if at the end set to 0
            let val2 = match temp_node2.as_ref() { 
                None => 0,
                Some(node) => node.val,
            };
            
            sum_val = val1 + val2 + carry;

            list = Self::add_ll(list, sum_val % 10); // Add the digit to new list

            carry  = sum_val / 10; // Compute carry by 'removing' a digit

            // Move to the next node if its present 
            temp_node1 = match temp_node1 { // No need to borrow as we consume it here, no reuse of temp_node1
                None => None,
                Some(node) => node.next,
            };

            // Move to the next node if its present
            temp_node2 = match temp_node2 {
                None => None,
                Some(node) => node.next,
            };
        }
        
        // Ensure left over carries post-addition get added to list
        if carry != 0 {
            list = Self::add_ll(list, carry);
        }

        return list;
    }

    // Adds to end of a linked list
    pub fn add_ll(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let newNode = Some(Box::new(ListNode::new(val as i32)));

        match head {
            None => return newNode,
            Some(ref mut node) => {
                let mut tempNode = node;

                while let Some(ref mut next) = tempNode.next {
                    tempNode = next;
                }
                tempNode.next = newNode;

                return head;
            }
        }
    }
}


/*
**** THIS WAS MY FIRST ATTEMPT AT THE SOLUTION ****
**** IT WAS A BIT TOO COMPLEX AND DIDN'T WORK ****
**** IT PASSED ALL BUT 3 CASES, WHERE THE GIVEN NUM EXCEEDED 64bit INTEGER LIMIT ****

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        let mut sum = Self::ll_to_int(l1) + Self::ll_to_int(l2); // Fetch the sum of both numbers
        let mut list: Option<Box<ListNode>> = None; // Create a linked list to store the numbers

        if sum == 0 {
            list = Self::add_ll(list, 0);
        } else {
            while sum > 0 {
            list = Self::add_ll(list, sum % 10); // Add digit to the list
            sum /= 10; // 'Remove' a digit from the sum
            }
        }
        return list;
    }

    // Coverts LL to integer
    pub fn ll_to_int(l1: Option<Box<ListNode>>) -> i64 {

        let mut tempNode = l1;
        let mut ctr = 1i64; // Counts what wheight digit we are on
        let mut num = 0i64;

        // Traverse to the end of a linked list
        while let Some(node) = tempNode {
            num = num + (node.val as i64 * ctr);
            ctr = ctr * 10; // Increment by wheight of next digit
            tempNode = node.next; // Go to next node in list
        }
        
        return num;
    }

    pub fn add_ll(mut head: Option<Box<ListNode>>, val: i64) -> Option<Box<ListNode>> {
        let newNode = Some(Box::new(ListNode::new(val as i32)));

        match head {
            None => return newNode,
            Some(ref mut node) => {
                let mut tempNode = node;

                while let Some(ref mut next) = tempNode.next {
                    tempNode = next;
                }
                tempNode.next = newNode;

                return head;
            }
        }
    }
}
*/