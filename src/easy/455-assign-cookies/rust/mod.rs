#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::unwrap_used,
    clippy::question_mark_used,
)]

// Assume you are an awesome parent and want to give your children some cookies. But, you should give each child at most one cookie.
// Each child `i` has a greed factor `g[i]`, which is the minimum size of a cookie that the child will be content with; and each cookie `j` has a size `s[j]`. If `s[j] >= g[i]`, we can assign the cookie `j` to the child `i`, and the child `i` will be content. Your goal is to maximize the number of your content children and output the maximum number.

struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();
        let (mut content_children, mut j) = (0, g.len() - 1);
        for i in (0 .. s.len()).rev() {
            while j < g.len() && g[j] > s[i] {
                j -= 1;
            }
            if j < g.len() {
                content_children += 1;
                j -= 1;
            }
        }
        content_children
    }
}

fn main() {
    let input0 = vec![1, 2, 3];
    let input1 = vec![1, 1];
    let output0 = Solution::find_content_children(input0, input1);
    println!("{output0:?}");
    let input2 = vec![1, 2];
    let input3 = vec![1, 2, 3];
    let output1 = Solution::find_content_children(input2, input3);
    println!("{output1:?}");
}