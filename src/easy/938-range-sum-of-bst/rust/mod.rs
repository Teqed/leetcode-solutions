#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::unwrap_used,
    clippy::question_mark_used
)]

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0;
        root.map_or((), |node| {
            let node = node.borrow();
            if node.val >= low && node.val <= high {
                sum += node.val;
            }
            if node.val > low {
                sum += Self::range_sum_bst(node.left.clone(), low, high);
            }
            if node.val < high {
                sum += Self::range_sum_bst(node.right.clone(), low, high);
            }
        });
        sum
    }
}

fn main() {}
